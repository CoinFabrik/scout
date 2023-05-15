#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;
extern crate rustc_span;

use clippy_utils::diagnostics::span_lint_and_help;
use if_chain::if_chain;
use rustc_hir::intravisit::Visitor;
use rustc_hir::intravisit::{walk_expr, FnKind};
use rustc_hir::QPath;
use rustc_hir::{Body, FnDecl, HirId};
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Span;

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
    "set_contract_storage only must be used with proper access control or input sanitation"
}

impl<'tcx> LateLintPass<'tcx> for SetStorageWarn {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        _: &'tcx FnDecl<'_>,
        body: &'tcx Body<'_>,
        _: Span,
        _: HirId,
    ) {
        // TODO: La razón por la que se usó un visitor de esta forma (para almacenar la info "global" en el scope de un if)
        // es porque no encontré una forma de poner un struct por fuera al que se pueda acceder desde la implementación
        // de los métodos de LateLintPass. Cambiar esta forma, o borrar comentario.
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
                        // TODO: falta chequear al reves
                        // TODO: se puede agregar si la operación que se realiza es "Eq"
                        if let ExprKind::Field(_, ident) = right.kind {
                            self.has_owner_in_if = ident.as_str().contains("owner");
                        }
                        if let ExprKind::MethodCall(func, ..) = &left.kind {
                            let function_name = func.ident.name.to_string();
                            if self.in_conditional {
                                self.has_caller_in_if = function_name.contains("caller");
                            }
                        }
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
                        if path.segments.len() == 2 && path.segments[0].ident.name.as_str() == "env" && path.segments[1].ident.name.as_str() == "set_contract_storage";
                        then {
                            self.has_set_contract = true;
                            if !self.in_conditional && (!self.has_owner_in_if || !self.has_caller_in_if) {
                                    self.unprotected = true;
                                    self.span = Some(expr.span);
                            }
                            // dbg!("has_caller:{} - has_owner:{} - in_conditional: {}", self.has_caller_in_if, self.has_owner_in_if, self.in_conditional);
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
            span_lint_and_help(
                cx,
                SET_STORAGE_WARN,
                // body.value.span,
                reentrant_storage.span.unwrap(),
                "Abitrary users should not have control over keys because it implies writing any value of left mapping, lazy variable, or the main struct of the contract located in position 0 of the storage",
                None,
                "Set access control and proper authorization validation for the set_contract_storage() function",
            );
        }
    }
}
