#![feature(rustc_private)]

extern crate rustc_hir;

use clippy_utils::diagnostics::span_lint_and_help;
use if_chain::if_chain;
use rustc_hir::{BinOpKind, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};

dylint_linting::declare_late_lint! {
    /// ### What it does
    /// This detector prevents the usage of timestamp/block number and modulo operator as a random number source.
    ///
    /// ### Why is this bad?
    /// The value of the block timestamp and block number can be manipulated by validators, which means they're not a secure source of randomness. Therefore, they shouldn't be used for generating random numbers, especially in the context of a betting contract where the outcomes of bets could be manipulated.
    ///
    /// ### Example
    /// ```rust
    /// let pseudo_random: u8 = (self.env().block_timestamp() % 37).try_into().unwrap();
    /// ```
    ///
    pub INSUFFICIENTLY_RANDOM_VALUES,
    Warn,
    "Weak pseudo random number using block timestamp or block number"
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
