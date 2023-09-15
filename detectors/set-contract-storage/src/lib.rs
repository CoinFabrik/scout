#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;
extern crate rustc_span;

use if_chain::if_chain;
use rustc_hir::def_id::LocalDefId;
use rustc_hir::intravisit::Visitor;
use rustc_hir::intravisit::{walk_expr, FnKind};
use rustc_hir::QPath;
use rustc_hir::{Body, FnDecl};
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Span;
use scout_audit_internal::Detector;

dylint_linting::declare_late_lint! {
    /// ### What it does
    /// Checks for calls to env::set_contract_storage.
    ///
    /// ### Why is this bad?
    /// Functions using keys as variables without proper access control or input sanitation can allow users to perform changes in arbitrary memory locations.
    ///
    /// ### Known problems
    /// Only check the function call, so false positives could result.
    ///
    /// ### Example
    /// ```rust
    /// fn set_contract_storage(
    ///     &mut self,
    ///     user_input_key: [u8; 68],
    ///     user_input_data: u128,
    /// ) -> Result<()> {
    ///     env::set_contract_storage(&user_input_key, &user_input_data);
    ///     Ok(())
    /// }
    /// ```
    /// Use instead:
    /// ```rust
    /// fn set_contract_storage(
    ///     &mut self,
    ///     user_input_key: [u8; 68],
    ///     user_input_data: u128,
    /// ) -> Result<()> {
    ///     if self.env().caller() == self.owner {
    ///         env::set_contract_storage(&user_input_key, &user_input_data);
    ///         Ok(())
    ///     } else {
    ///         Err(Error::UserNotOwner)
    ///     }
    /// }
    /// ```
    pub SET_STORAGE_WARN,
    Warn,
    Detector::SetContractStorage.get_lint_message()
}

fn expr_check_owner(expr: &Expr) -> bool {
    if let ExprKind::Field(_, ident) = expr.kind {
        ident.as_str().contains("owner")
    } else {
        false
    }
}

fn expr_check_caller(expr: &Expr) -> bool {
    if let ExprKind::MethodCall(func, ..) = expr.kind {
        let function_name = func.ident.name.to_string();
        function_name.contains("caller")
    } else {
        false
    }
}

impl<'tcx> LateLintPass<'tcx> for SetStorageWarn {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        _: &'tcx FnDecl<'_>,
        body: &'tcx Body<'_>,
        _: Span,
        _: LocalDefId,
    ) {
        struct SetContractStorage {
            span: Option<Span>,
            unprotected: bool,
            in_conditional: bool,
            has_caller_in_if: bool,
            has_owner_in_if: bool,
            has_set_contract: bool,
        }

        impl<'tcx> Visitor<'tcx> for SetContractStorage {
            fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
                if self.in_conditional {
                    if let ExprKind::Binary(_, left, right) = &expr.kind {
                        self.has_owner_in_if = expr_check_owner(right) || expr_check_owner(left);
                        self.has_caller_in_if = expr_check_caller(right) || expr_check_caller(left);
                    }
                }
                if let ExprKind::If(..) = &expr.kind {
                    self.in_conditional = true;
                    walk_expr(self, expr);
                    self.in_conditional = false;
                } else if let ExprKind::Call(callee, _) = expr.kind {
                    if_chain! {
                        if let ExprKind::Path(method_path) = &callee.kind;
                        if let QPath::Resolved(None, path) = *method_path;
                        if path.segments.len() == 2;
                        if path.segments[0].ident.name.as_str() == "env";
                        if path.segments[1].ident.name.as_str() == "set_contract_storage";
                        then {
                            self.has_set_contract = true;
                            if !self.in_conditional && (!self.has_owner_in_if || !self.has_caller_in_if) {
                                    self.unprotected = true;
                                    self.span = Some(expr.span);
                            }
                        }
                    }
                }
                walk_expr(self, expr);
            }
        }

        let mut reentrant_storage = SetContractStorage {
            span: None,
            unprotected: false,
            in_conditional: false,
            has_caller_in_if: false,
            has_owner_in_if: false,
            has_set_contract: false,
        };

        walk_expr(&mut reentrant_storage, body.value);

        if reentrant_storage.has_set_contract && reentrant_storage.unprotected {
            Detector::SetContractStorage.span_lint_and_help(
                cx,
                SET_STORAGE_WARN,
                // body.value.span,
                reentrant_storage.span.unwrap(),
                "Set access control and proper authorization validation for the set_contract_storage() function",
            );
        }
    }
}
