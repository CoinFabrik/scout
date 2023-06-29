#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;

use clippy_utils::diagnostics::span_lint_and_help;
use if_chain::if_chain;
use rustc_hir::{Expr, ExprKind, BinOpKind};
use rustc_lint::{LateContext, LateLintPass};

dylint_linting::declare_late_lint! {
    ///# Insufficiently random values
    ///
    ///### What it does
    ///This detector prevents the usage of timestamp and modulo operator as a random number source.
    ///### Why is this bad?
    ///Block timestamp can be influenced by validators.
    ///### Known problems
    ///-
    ///
    ///### Example
    ///-
    ///
    pub INSUFFICIENTLY_RANDOM_VALUES,
    Warn,
    "weak pseudo random number using timestamp"
}

impl<'tcx> LateLintPass<'tcx> for InsufficientlyRandomValues {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'_>) {
        if_chain! {
            if let ExprKind::Binary(op, lexp, _rexp) = expr.kind;
            if op.node == BinOpKind::Rem;
            if let ExprKind::MethodCall(path, _, _, _) = lexp.kind;
            if path.ident.as_str() == "block_timestamp" ||
                path.ident.as_str() == "block_number";
            then {
                span_lint_and_help(
                    cx,
                    INSUFFICIENTLY_RANDOM_VALUES,
                    expr.span,
                    "In order to prevent randomness manipulations by validators block_timestamp should not be used as random number source",
                    None,
                    "This expression seems to use block_timestamp as a pseudo random number",
                );
            }
        }
    }
}
