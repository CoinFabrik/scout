#![feature(rustc_private)]

extern crate rustc_hir;
extern crate rustc_span;

use clippy_utils::diagnostics::span_lint_and_help;
use if_chain::if_chain;
use rustc_hir::intravisit::walk_expr;
use rustc_hir::intravisit::Visitor;
use rustc_hir::BinOpKind;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Span;

dylint_linting::declare_late_lint! {
    /// ### What it does
    /// Checks the existence of a division before a multiplication.
    ///
    /// ### Why is this bad?
    /// Division between two integers might return zero.
    ///
    /// ### Known problems
    ///
    /// ### Example
    /// ```rust
    /// // example code that raises a warning
    /// let x = 1;
    /// let y = 2;
    /// let z = x / y * 3;
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code that passes the linter
    /// let x = 1;
    /// let y = 2;
    /// let z = x * 3 / y;
    /// ```
    pub DIVIDE_BEFORE_MULTIPLY,
    Warn,
    "Division should be performed after multiplication"
}

impl<'tcx> LateLintPass<'tcx> for DivideBeforeMultiply {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: rustc_hir::intravisit::FnKind<'tcx>,
        _: &'tcx rustc_hir::FnDecl<'tcx>,
        body: &'tcx rustc_hir::Body<'tcx>,
        _: Span,
        _: rustc_hir::HirId,
    ) {
        struct DivideBeforeMultiplyVisitor<'a, 'tcx> {
            cx: &'a LateContext<'tcx>,
            span: Vec<Option<Span>>,
            has_division: bool,
            is_precision_loss: bool,
        }

        impl<'a, 'tcx> Visitor<'tcx> for DivideBeforeMultiplyVisitor<'a, 'tcx> {
            fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
                if_chain! {
                    if let ExprKind::Binary(binop, lhs, rhs) = expr.kind;
                    if let BinOpKind::Mul = binop.node;
                    if let lhs_ty = self.cx.typeck_results().expr_ty(lhs);
                    if let rhs_ty = self.cx.typeck_results().expr_ty(rhs);
                    if lhs_ty.is_integral() && rhs_ty.is_integral();
                    then {
                        self.has_division = true;
                        walk_expr(self, expr);
                    }
                }

                if_chain!(
                    if self.has_division;
                    if let ExprKind::Binary(binop, _, _) = expr.kind;
                    if let BinOpKind::Div = binop.node;
                    then {
                        self.is_precision_loss = true;
                        self.span.push(Some(expr.span));
                    }
                );

                walk_expr(self, expr);
            }
        }

        let mut visitor = DivideBeforeMultiplyVisitor {
            cx,
            span: Vec::new(),
            has_division: false,
            is_precision_loss: false,
        };
        walk_expr(&mut visitor, body.value);

        if visitor.is_precision_loss {
            visitor.span.iter().for_each(|span| {
                span_lint_and_help(
                    cx,
                    DIVIDE_BEFORE_MULTIPLY,
                    span.unwrap(),
                    "Division between two integers might return zero",
                    None,
                    "Consider reversing the order of operations to reduce the loss of precision.",
                );
            });
        }
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(
        env!("CARGO_PKG_NAME"),
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("ui"),
    );
}
