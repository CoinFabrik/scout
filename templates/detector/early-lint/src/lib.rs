#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_span;

use rustc_ast::visit::{FnKind, Visitor};
use rustc_ast::{Expr, ExprKind};
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_span::Span;
use scout_audit_clippy_utils::diagnostics::span_lint_and_help;

dylint_linting::declare_early_lint! {
    /// ### What it does
    /// Describe what the lint does.
    ///
    /// ### Why is this bad?
    /// Describe why the linted code is considered bad.
    ///
    /// ### Example
    /// ```rust
    /// // example code where a warning is issued
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code that does not raise a warning
    /// ```
    pub YOUR_LINT_NAME,
    Warn,
    "Short description of the lint"
}

struct YourVisitor {
    // Add any fields necessary for your lint
}

impl<'ast> Visitor<'ast> for YourVisitor {
    fn visit_expr(&mut self, ex: &'ast Expr) {
        // Implement the logic of your lint here

        // Call `walk_expr` to visit the descendants of `ex`
        rustc_ast::visit::walk_expr(self, ex);
    }
}

impl EarlyLintPass for YourLint {
    fn check_fn(
        &mut self,
        cx: &EarlyContext<'_>,
        fn_kind: FnKind<'_>,
        _: Span,
        _: rustc_ast::NodeId,
    ) {
        // Implement check_fn and emit any necessary diagnostic messages
    }
}
