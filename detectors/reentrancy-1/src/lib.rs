#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;
extern crate rustc_span;

use if_chain::if_chain;
use rustc_hir::def_id::LocalDefId;
use rustc_hir::intravisit::{walk_expr, FnKind};
use rustc_hir::intravisit::{walk_stmt, Visitor};
use rustc_hir::{Body, FnDecl, Stmt};
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Span;
use scout_audit_internal::Detector;

dylint_linting::declare_late_lint! {
    /// ### What it does
    /// This linting rule checks whether the 'check-effect' interaction pattern has been properly followed by code that invokes a contract that may call back the original one.
    /// ### Why is this bad?
    /// If state modifications are made after a contract call, reentrant calls may not detect these modifications, potentially leading to unexpected behaviors such as double spending.
    /// ### Known problems
    /// If called method does not perform a malicious reentrancy (i.e. known method from known contract) false positives will arise.
    /// If the usage of set_allow_reentry(true) or later state changes are performed in an auxiliary function, this detector will not detect the reentrancy.
    ///
    /// ### Example
    /// ```rust
    /// let caller_addr = self.env().caller();
    /// let caller_balance = self.balance(caller_addr);
    ///
    /// if amount > caller_balance {
    ///     return Ok(caller_balance);
    /// }
    ///
    /// let call = build_call::<ink::env::DefaultEnvironment>()
    ///     .call(address)
    ///     .transferred_value(amount)
    ///     .exec_input(ink::env::call::ExecutionInput::new(Selector::new(
    ///         selector.to_be_bytes(),
    ///     )))
    ///     .call_flags(ink::env::CallFlags::default().set_allow_reentry(true))
    ///     .returns::<()>()
    ///     .params();
    /// self.env()
    ///     .invoke_contract(&call)
    ///     .map_err(|_| Error::ContractInvokeFailed)?
    ///     .map_err(|_| Error::ContractInvokeFailed)?;
    ///
    /// let new_balance = caller_balance.checked_sub(amount).ok_or(Error::Underflow)?;
    /// self.balances.insert(caller_addr, &new_balance);
    /// ```
    /// Use instead:
    /// ```rust
    /// let caller_addr = self.env().caller();
    /// let caller_balance = self.balances.get(caller_addr).unwrap_or(0);
    /// if amount <= caller_balance {
    ///     //The balance is updated before the contract call
    ///     self.balances
    ///         .insert(caller_addr, &(caller_balance - amount));
    ///     let call = build_call::<ink::env::DefaultEnvironment>()
    ///         .call(address)
    ///         .transferred_value(amount)
    ///         .exec_input(ink::env::call::ExecutionInput::new(Selector::new(
    ///             selector.to_be_bytes(),
    ///         )))
    ///         .call_flags(ink::env::CallFlags::default().set_allow_reentry(true))
    ///         .returns::<()>()
    ///         .params();
    ///     self.env()
    ///         .invoke_contract(&call)
    ///         .unwrap_or_else(|err| panic!("Err {:?}", err))
    ///         .unwrap_or_else(|err| panic!("LangErr {:?}", err));
    ///
    ///     return caller_balance - amount;
    /// } else {
    ///     return caller_balance;
    /// }
    /// ```
    pub REENTRANCY_1,
    Warn,
    Detector::Reentrancy1.get_lint_message()
}

impl<'tcx> LateLintPass<'tcx> for Reentrancy1 {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        _: &'tcx FnDecl<'_>,
        body: &'tcx Body<'_>,
        _: Span,
        _: LocalDefId,
    ) {
        struct ReentrantStorage {
            span: Option<Span>,
            has_invoke_contract_call: bool,
            allow_reentrancy_flag: bool,
            state_change: bool,
        }

        fn check_invoke_contract_call(expr: &Expr) -> Option<Span> {
            if_chain! {
                if let ExprKind::MethodCall(func, _, _, _) = &expr.kind;
                if let function_name = func.ident.name.to_string();
                if function_name == "invoke_contract" ;
                then {
                        return Some(expr.span);
                }
            }
            None
        }
        fn check_allow_reentrancy(expr: &Expr) -> bool {
            if_chain! {
            if let ExprKind::MethodCall(func, _, args, _) = &expr.kind;
            if let function_name = func.ident.name.to_string();
            if function_name.contains("set_allow_reentry");
            then {
                    if_chain! {
                        if let ExprKind::Lit(lit) = &args[0].kind;
                        if &lit.node.to_string() == "true";
                        then {
                            return true;
                        }
                    }
                }
            }
            false
        }
        fn check_state_change(s: &Stmt) -> bool {
            if_chain! {
                if let rustc_hir::StmtKind::Semi(expr) = &s.kind;
                if let rustc_hir::ExprKind::Assign(lhs, ..) = &expr.kind;
                if let rustc_hir::ExprKind::Field(base, _) = lhs.kind; // self.field_name <- base: self, field_name: ident
                if let rustc_hir::ExprKind::Path(path) = &base.kind;
                if let rustc_hir::QPath::Resolved(None, path) = *path;
                if path.segments.iter().any(|base| base.ident.as_str().contains("self"));                then {
                    return true;
                }
            }
            if_chain! {
                // check access to balance.insert
                if let rustc_hir::StmtKind::Semi(expr) = &s.kind;
                if let rustc_hir::ExprKind::MethodCall(func, rec, ..) = &expr.kind;
                if let function_name = func.ident.name.to_string();
                if function_name == "insert";
                // Fix this: checking for "balances"
                if let rustc_hir::ExprKind::Field(base, _) = &rec.kind; // self.field_name <- base: self, field_name: ident
                if let rustc_hir::ExprKind::Path(path) = &base.kind;
                if let rustc_hir::QPath::Resolved(None, path) = *path;
                if path.segments.iter().any(|base| base.ident.as_str().contains("self"));
                then {
                    return true;
                }
            }
            false
        }

        impl<'tcx> Visitor<'tcx> for ReentrantStorage {
            fn visit_stmt(&mut self, stmt: &'tcx Stmt<'tcx>) {
                // check for an statement that modifies the state
                // the state is modified if the statement is an assignment and modifies an struct
                // or if if invokes a function and the receiver is a env::balance
                if self.has_invoke_contract_call && self.allow_reentrancy_flag {
                    if check_state_change(stmt) {
                        self.state_change = true;
                    }
                } else {
                    walk_stmt(self, stmt);
                }
            }

            fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
                if self.allow_reentrancy_flag {
                    let invoke_contract_span = check_invoke_contract_call(expr);
                    if invoke_contract_span.is_some() {
                        self.has_invoke_contract_call = true;
                        self.span = invoke_contract_span;
                    }
                }
                if check_allow_reentrancy(expr) {
                    self.allow_reentrancy_flag = true;
                }

                walk_expr(self, expr);
            }
        }

        let mut reentrant_storage = ReentrantStorage {
            span: None,
            has_invoke_contract_call: false,
            allow_reentrancy_flag: false,
            state_change: false,
        };

        walk_expr(&mut reentrant_storage, body.value);

        if reentrant_storage.has_invoke_contract_call
            && reentrant_storage.allow_reentrancy_flag
            && reentrant_storage.state_change
        {
            Detector::Reentrancy1.span_lint_and_help(
                cx,
                REENTRANCY_1,
                reentrant_storage.span.unwrap(),
                "This statement seems to call another contract after the flag set_allow_reentry was enabled [todo: check state changes after this statement]",
            );
        }
    }
}
