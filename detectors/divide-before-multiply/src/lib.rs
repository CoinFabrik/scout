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

fn get_divisions_inside_expr(expr: &Expr<'_>) -> Vec<Span> {
    struct DivisionsInsideExpr {
        divisions: Vec<Span>,
    }

    impl Visitor<'_> for DivisionsInsideExpr {
        fn visit_expr(&mut self, expr: &Expr<'_>) {
            if_chain! {
                if let ExprKind::Binary(op, _lexpr, _rexpr) = expr.kind;
                if BinOpKind::Div == op.node;
                then{
                    self.divisions.push(expr.span);
                }
            }
            walk_expr(self, expr);
        }
    }

    let mut visitor = DivisionsInsideExpr {
        divisions: Vec::default(),
    };

    walk_expr(&mut visitor, expr);

    visitor.divisions
}

impl<'tcx> LateLintPass<'tcx> for DivideBeforeMultiply {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx rustc_hir::Expr<'tcx>) {
        if_chain! {
            if let ExprKind::Binary(op, _lexpr, _rexpr) = expr.kind;
            if BinOpKind::Mul == op.node;
            then{
                for division in get_divisions_inside_expr(expr) {
                    span_lint_and_help(
                        cx,
                        DIVIDE_BEFORE_MULTIPLY,
                        division,
                        "Division before multiplication might result in a loss of precision",
                        None,
                        "Consider reversing the order of operations to reduce the loss of precision.",
                    );
                }
            }
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
