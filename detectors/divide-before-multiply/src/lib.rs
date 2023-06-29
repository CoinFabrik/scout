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
        struct DivideBeforeMultiplyVisitor {
            has_multiplication: bool,
            is_precision_loss: bool,
            is_precision_loss_span: Vec<Option<Span>>,
        }

        impl<'tcx> Visitor<'tcx> for DivideBeforeMultiplyVisitor {
            fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
                if_chain! {
                    if let ExprKind::Binary(binop, _, _) = expr.kind;
                    if let BinOpKind::Mul = binop.node;
                    then {
                        self.has_multiplication = true;
                        walk_expr(self, expr);
                    }
                }

                if_chain!(
                    if self.has_multiplication;
                    if let ExprKind::Binary(binop, _, _) = expr.kind;
                    if let BinOpKind::Div = binop.node;
                    then {
                        self.is_precision_loss = true;
                        self.is_precision_loss_span.push(Some(expr.span));
                    }
                );

                walk_expr(self, expr);
            }
        }

        let mut visitor = DivideBeforeMultiplyVisitor {
            has_multiplication: false,
            is_precision_loss: false,
            is_precision_loss_span: Vec::new(),
        };

        walk_expr(&mut visitor, body.value);

        if visitor.is_precision_loss {
            visitor.is_precision_loss_span.iter().for_each(|span| {
                if let Some(span) = span {
                    span_lint_and_help(
                        cx,
                        DIVIDE_BEFORE_MULTIPLY,
                        *span,
                        "Division before multiplication might result in a loss of precision",
                        None,
                        "Consider reversing the order of operations to reduce the loss of precision.",
                    );
                }
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
