#![feature(rustc_private)]
#![feature(let_chains)]
extern crate rustc_ast;
extern crate rustc_span;

use clippy_wrappers::span_lint;
use rustc_ast::{BlockCheckMode, Expr, ExprKind, UnsafeSource};
use rustc_lint::{EarlyContext, EarlyLintPass};

const LINT_MESSAGE: &str = "Avoid using unsafe blocks as it may lead to undefined behavior.";
scout_audit_dylint_linting::impl_pre_expansion_lint! {
    pub AVOID_UNSAFE_BLOCK,
    Warn,
    LINT_MESSAGE,
    AvoidUnsafeBlock::default(),
    {
        name: "Avoid unsafe block",
        long_message: "The unsafe block is used to bypass Rust's safety checks. It is recommended to avoid using unsafe blocks as much as possible, and to use them only when necessary.",
        severity: "Enhancement",
        help: "https://coinfabrik.github.io/scout/docs/vulnerabilities/avoid-unsafe-block",
        vulnerability_class: "Best practices",
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
