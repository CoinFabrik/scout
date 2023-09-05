#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;

use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::higher;
use if_chain::if_chain;
use rustc_hir::{Expr, ExprKind, QPath};
use rustc_lint::{LateContext, LateLintPass};
use scout_audit_internal::Detector;

dylint_linting::declare_late_lint! {
    /// ### What it does
    /// This detector checks that when using for or while loops, their conditions limit the execution to a constant number of iterations.
    /// ### Why is this bad?
    /// If the number of iterations is not limited to a specific range, it could potentially cause out of gas exceptions.
    /// ### Known problems
    /// False positives are to be expected when using variables that can only be set using controlled flows that limit the values within acceptable ranges.
    /// ### Example
    /// ```rust
    /// pub fn pay_out(&mut self) {
    ///     for i in 0..self.next_payee_ix {
    ///         let payee = self.payees.get(&i).unwrap();
    ///         self.env().transfer(payee.address, payee.value).unwrap();
    ///     }
    /// }
    /// ```
    /// Use instead:
    /// ```rust
    /// pub fn pay_out(&mut self, payee: u128) {
    ///     let payee = self.payees.get(&payee).unwrap();
    ///     self.env().transfer(payee.address, payee.value).unwrap();
    /// }
    /// ```
    pub DOS_UNBOUNDED_OPERATION,
    Warn,
    Detector::DosUnboundedOperation.get_lint_message()
}

fn is_self_field(field: &Expr<'_>) -> bool {
    if_chain! {
        if let ExprKind::Field(base, _) = field.kind; // self.field_name <- base: self, field_name: ident
        if let ExprKind::Path(path) = &base.kind;
        if let QPath::Resolved(None, path) = *path;
        if path.segments.last().unwrap().ident.as_str().contains("self");
        then {
            return true;
        }
    }
    false
}

/**
 * This function checks if a field is a function parameter.
 * To achieve this, the function obtains the Hirid of the field and iterates through its parent nodes
 * searching for function names that match the given field name
 */
fn is_func_parameter<'tcx>(cx: &LateContext<'tcx>, field: &'tcx Expr<'_>) -> bool {
    if_chain! {
        if let ExprKind::Path(path) = &field.kind;
        if let QPath::Resolved(None, path) = *path;
        then {
            let mut parent_iter = cx.tcx.hir().parent_iter(field.hir_id);
            for node in  &mut parent_iter {
                let body = cx.tcx.hir().maybe_body_owned_by(node.0.owner.def_id);
                if let Some(body_id) = body {
                    for param in cx.tcx.hir().body(body_id).params {
                        if let rustc_hir::PatKind::Binding(_, _, param, _) = &param.pat.kind {
                            if path.segments.last().unwrap().ident.as_str().contains(param.name.as_str()) {
                                return true;
                            }
                        }
                    }
                    return false;
                }
            }
        }
    }
    false
}

impl<'tcx> LateLintPass<'tcx> for DosUnboundedOperation {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'_>) {
        let mut warn = false;
        if_chain! {
            if let Some(higher::ForLoop { pat: _, arg, body: _, .. }) = higher::ForLoop::hir(expr);
            if let ExprKind::Struct(_, field, _) = arg.kind;
            if is_self_field(field[1].expr) || is_func_parameter(cx, field[1].expr);
            then {
                warn = true;
            }
        }
        if warn {
            span_lint_and_help(
                cx,
                DOS_UNBOUNDED_OPERATION,
                expr.span,
                Detector::DosUnboundedOperation.get_lint_message(),
                None,
                "This loop seems to do not have a fixed number of iterations",
            );
        }
    }
}
