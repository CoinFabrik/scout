#![feature(rustc_private)]

extern crate rustc_hir;
extern crate rustc_span;

use rustc_hir::intravisit::{self, FnKind, Visitor};
use rustc_hir::*;
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Span;
use scout_audit_clippy_utils::diagnostics::span_lint_and_help;

dylint_linting::declare_late_lint! {
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

struct YourVisitor<'tcx> {
    // Add any fields necessary for your lint
}

impl<'tcx> Visitor<'tcx> for YourVisitor<'tcx> {
    fn visit_expr(&mut self, ex: &'tcx Expr<'_>) {
        // Implement the logic of your lint here

        // Call `walk_expr` to visit the descendants of `ex`
        intravisit::walk_expr(self, ex);
    }

    // Implement other methods of the `Visitor` trait as needed
}

impl<'tcx> LateLintPass<'tcx> for YourLint {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        fn_kind: FnKind<'tcx>,
        decl: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        span: Span,
        hir_id: HirId,
    ) {
        // Implement check_fn and emit any necessary diagnostic messages
    }
}
