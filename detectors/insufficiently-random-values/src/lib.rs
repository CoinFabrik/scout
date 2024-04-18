#![feature(rustc_private)]

extern crate rustc_hir;

use if_chain::if_chain;
use rustc_hir::{BinOpKind, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};

const LINT_MESSAGE: &str = "In order to prevent randomness manipulations by validators block_timestamp should not be used as random number source";

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
    LINT_MESSAGE,
    {
        name: "Insufficiently Random Values",
        long_message: "Using block attributes like block_timestamp or block_number for random number generation in ink! Substrate smart contracts is not recommended due to the predictability of these values. Block attributes are publicly visible and deterministic, making it easy for malicious actors to anticipate their values and manipulate outcomes to their advantage.",
        severity: "Critical",
        help: "https://coinfabrik.github.io/scout/docs/vulnerabilities/insufficiently-random-values",
        vulnerability_class: "Block attributes",
    }
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
                scout_audit_clippy_utils::diagnostics::span_lint_and_help(
                    cx,
                    INSUFFICIENTLY_RANDOM_VALUES,
                    expr.span,
                    LINT_MESSAGE,
                    None,
                    "This expression seems to use block_timestamp as a pseudo random number",
                );
            }
        }
    }
}
