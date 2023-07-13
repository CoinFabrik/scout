#![feature(rustc_private)]

extern crate rustc_abi;
extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;
extern crate rustc_type_ir;

use std::collections::{HashMap, HashSet};

use clippy_utils::diagnostics::span_lint_and_help;
use if_chain::if_chain;
use rustc_abi::VariantIdx;
use rustc_ast::ast::LitKind;
use rustc_hir::def::Res;
use rustc_hir::intravisit::{walk_expr, FnKind};
use rustc_hir::intravisit::{walk_local, Visitor};
use rustc_hir::{Body, FnDecl, HirId, Local, PatKind, QPath};
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::TyKind;
use rustc_span::def_id::LocalDefId;
use rustc_span::{Span, Symbol};

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
    pub REENTRANCY,
    Warn,
    "Reentrancy vulnerability"
}

const SET_ALLOW_REENTRY: &str = "set_allow_reentry";
const INVOKE_CONTRACT: &str = "invoke_contract";
const INSERT: &str = "insert";
const MAPPING: &str = "Mapping";
const ACCOUNT_ID: &str = "AccountId";
const U128: &str = "u128";

impl<'tcx> LateLintPass<'tcx> for Reentrancy {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        _: &'tcx FnDecl<'_>,
        body: &'tcx Body<'_>,
        _: Span,
        _: LocalDefId,
    ) {
        struct ReentrancyVisitor<'a, 'tcx: 'a> {
            cx: &'a LateContext<'tcx>,
            contracts_tainted_for_reentrancy: HashSet<Symbol>,
            current_method_call: Option<Symbol>,
            bool_var_values: HashMap<HirId, bool>,
            reentrancy_spans: Vec<Span>,
            should_look_for_insert: bool,
            has_insert_operation: bool,
        }

        // This function is called whenever a contract is identified as potentially susceptible to reentrancy.
        fn set_tainted_contract(visitor: &mut ReentrancyVisitor) {
            if let Some(method_calls) = &visitor.current_method_call {
                visitor
                    .contracts_tainted_for_reentrancy
                    .insert(*method_calls);
                visitor.current_method_call = None;
            }
        }

        fn handle_set_allow_reentry(visitor: &mut ReentrancyVisitor, args: &&[Expr<'_>]) {
            match &args[0].kind {
                ExprKind::Lit(lit) => {
                    // If the argument is a boolean literal and it's true, call set_tainted_contract
                    if let LitKind::Bool(value) = lit.node {
                        if value {
                            set_tainted_contract(visitor);
                        }
                    }
                }
                ExprKind::Path(qpath) => {
                    // If the argument is a local variable, check if it's a boolean and if it's true
                    if_chain! {
                        if let res = visitor.cx.qpath_res(qpath, args[0].hir_id);
                        if let Res::Local(_) = res;
                        if let QPath::Resolved(_, path) = qpath;
                        then {
                            for path_segment in path.segments {
                                // If the argument is a known boolean variable, check if it's true
                                if let Res::Local(hir_id) = path_segment.res {
                                    if visitor.bool_var_values.get(&hir_id).map_or(true, |v| *v) {
                                        set_tainted_contract(visitor);
                                    }
                                }
                            }
                        }
                    }
                }
                _ => (),
            }
        }

        fn handle_invoke_contract(
            visitor: &mut ReentrancyVisitor,
            args: &&[Expr<'_>],
            expr: &Expr<'_>,
        ) {
            if_chain! {
                if let ExprKind::AddrOf(_, _, invoke_expr) = &args[0].kind;
                if let ExprKind::Path(qpath) = &invoke_expr.kind;
                if let QPath::Resolved(_, path) = qpath;
                then{
                    for path_segment in path.segments {
                        // If the argument is a tainted contract, add the span of this expression to the span vector
                        if visitor.contracts_tainted_for_reentrancy.contains(&path_segment.ident.name) {
                            visitor.should_look_for_insert = true;
                            visitor.reentrancy_spans.push(expr.span);
                        }
                    }
                }
            }
        }

        fn handle_insert(visitor: &mut ReentrancyVisitor, expr: &Expr<'_>) {
            if_chain! {
                if let ExprKind::MethodCall(_, expr1, _, _) = &expr.kind;
                if let object_type = visitor.cx.typeck_results().expr_ty(expr1);
                if let TyKind::Adt(adt_def, substs) = object_type.kind();
                if let Some(variant) = adt_def.variants().get(VariantIdx::from_u32(0));
                if variant.name.as_str() == MAPPING;
                if let mut has_account_id = false;
                if let mut has_u128 = false;
                then{
                    substs.types().for_each(|inner_type| {
                        let str_inner_type = inner_type.to_string();
                        if str_inner_type.contains(ACCOUNT_ID) {
                            has_account_id = true;
                        } else if str_inner_type.contains(U128) {
                            has_u128 = true;
                        }
                    });
                    visitor.has_insert_operation = has_account_id && has_u128;
                }
            }
        }

        impl<'a, 'tcx> Visitor<'tcx> for ReentrancyVisitor<'a, 'tcx> {
            fn visit_local(&mut self, local: &'tcx Local<'tcx>) {
                if let Some(init) = &local.init {
                    if let PatKind::Binding(_, _, ident, _) = &local.pat.kind {
                        match &init.kind {
                            // Check if the variable being declared is a boolean, if so, add it to the bool_declarations hashmap
                            ExprKind::Lit(lit) => {
                                if let LitKind::Bool(value) = lit.node {
                                    self.bool_var_values.insert(local.pat.hir_id, value);
                                }
                            }
                            ExprKind::MethodCall(_, _, _, _) => {
                                self.current_method_call = Some(ident.name);
                            }
                            // Check if the variable being declared is a boolean, if so, add it to the bool_declarations hashmap
                            ExprKind::Path(QPath::Resolved(_, path)) => {
                                if let Some(segment) = path.segments.last() {
                                    if let Res::Local(hir_id) = segment.res {
                                        if let Some(value) = self.bool_var_values.get(&hir_id) {
                                            self.bool_var_values.insert(local.pat.hir_id, *value);
                                        }
                                    }
                                }
                            }
                            _ => (),
                        }
                    }
                    walk_local(self, local);
                }
            }

            // This method is called for every expression.
            fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
                if let ExprKind::MethodCall(func, _, args, _) = &expr.kind {
                    let function_name = func.ident.name.as_str();
                    match function_name {
                        // The function "set_allow_reentry" is being called
                        SET_ALLOW_REENTRY => handle_set_allow_reentry(self, args),
                        // The function "invoke_contract" is being called
                        INVOKE_CONTRACT => handle_invoke_contract(self, args, expr),
                        // The function "insert" is being called
                        INSERT => {
                            if self.should_look_for_insert {
                                handle_insert(self, expr)
                            }
                        }
                        _ => (),
                    }
                }
                walk_expr(self, expr)
            }
        }

        // The main function where we start the visitor to traverse the AST.
        let mut reentrancy_visitor = ReentrancyVisitor {
            cx,
            contracts_tainted_for_reentrancy: HashSet::new(),
            current_method_call: None,
            bool_var_values: HashMap::new(),
            reentrancy_spans: Vec::new(),
            has_insert_operation: false,
            should_look_for_insert: false,
        };
        walk_expr(&mut reentrancy_visitor, body.value);

        // Iterate over all potential reentrancy spans and emit a warning for each.
        if reentrancy_visitor.has_insert_operation {
            reentrancy_visitor.reentrancy_spans.into_iter().for_each(|span| {
                span_lint_and_help(
                    cx,
                    REENTRANCY,
                    span,
                    "External calls could open the opportunity for a malicious contract to execute any arbitrary code",
                    None,
                    "This statement seems to call another contract after the flag set_allow_reentry was enabled [todo: check state changes after this statement]",
                );
            })
        }
    }
}
