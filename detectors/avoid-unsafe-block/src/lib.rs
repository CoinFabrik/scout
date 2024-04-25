#![feature(rustc_private)]
#![feature(let_chains)]
extern crate rustc_ast;
extern crate rustc_span;

use rustc_ast::{BlockCheckMode, Expr, ExprKind, UnsafeSource};
use rustc_lint::{EarlyContext, EarlyLintPass};
use scout_audit_clippy_utils::diagnostics::span_lint;

const LINT_MESSAGE: &str = "Assert causes panic. Instead, return a proper error.";
dylint_linting::impl_pre_expansion_lint! {
    pub AVOID_UNSAFE_BLOCK,
    Warn,
    LINT_MESSAGE,
    AvoidUnsafeBlock::default(),
    {
        name: "Unprotected Mapping Operation",
        long_message: "Modifying mappings with an arbitrary key given by the user could lead to unintented modifications of critical data, modifying data belonging to other users, causing denial of service, unathorized access, and other potential issues.    ",
        severity: "Critical",
        help: "https://coinfabrik.github.io/scout/docs/vulnerabilities/unprotected-mapping-operation",
        vulnerability_class: "Validations and error handling",
    }
}

#[derive(Default)]
pub struct AvoidUnsafeBlock {}

impl EarlyLintPass for AvoidUnsafeBlock {
    fn check_expr(&mut self, cx: &EarlyContext, expr: &Expr) {
        if let ExprKind::Block(block, ..) = &expr.kind
            && block.rules == BlockCheckMode::Unsafe(UnsafeSource::UserProvided)
        {
            span_lint(cx, AVOID_UNSAFE_BLOCK, expr.span, LINT_MESSAGE)
        }
    }
}
