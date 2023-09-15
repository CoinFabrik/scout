#![feature(rustc_private)]

extern crate rustc_hir;
extern crate rustc_span;

use clippy_utils::consts::constant_simple;
use clippy_utils::is_integer_literal;
use rustc_hir::{self as hir, Body, Expr, ExprKind, UnOp};
use rustc_lint::LateContext;
use rustc_lint::LateLintPass;
use rustc_span::source_map::Span;
use scout_audit_internal::Detector;

dylint_linting::impl_late_lint! {
    /// ### What it does
    /// Checks for integer arithmetic operations which could overflow or panic.
    ///
    /// Specifically, checks for any operators (`+`, `-`, `*`, `<<`, etc) which are capable
    /// of overflowing according to the [Rust
    /// Reference](https://doc.rust-lang.org/reference/expressions/operator-expr.html#overflow),
    /// or which can panic (`/`, `%`). No bounds analysis or sophisticated reasoning is
    /// attempted.
    ///
    /// ### Why is this bad?
    /// Integer overflow will trigger a panic in debug builds or will wrap in
    /// release mode. Division by zero will cause a panic in either mode. In some applications one
    /// wants explicitly checked, wrapping or saturating arithmetic.
    ///
    /// ### Example
    /// ```rust
    /// # let a = 0;
    /// a + 1;
    /// ```
    pub INTEGER_OVERFLOW_UNDERFLOW,
    Warn,
    Detector::IntegerOverflowOrUnderflow.get_lint_message(),
    IntegerOverflowUnderflow::default()
}

#[derive(Default)]
pub struct IntegerOverflowUnderflow {
    arithmetic_context: ArithmeticContext,
}
impl IntegerOverflowUnderflow {
    pub fn new() -> Self {
        Self {
            arithmetic_context: ArithmeticContext::default(),
        }
    }
}

impl<'tcx> LateLintPass<'tcx> for IntegerOverflowUnderflow {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, e: &'tcx Expr<'_>) {
        match e.kind {
            ExprKind::Binary(op, lhs, rhs) => {
                self.arithmetic_context
                    .check_binary(cx, e, op.node, lhs, rhs);
            }
            ExprKind::AssignOp(op, lhs, rhs) => {
                self.arithmetic_context
                    .check_binary(cx, e, op.node, lhs, rhs);
            }
            ExprKind::Unary(op, arg) => {
                if op == UnOp::Neg {
                    self.arithmetic_context.check_negate(cx, e, arg);
                }
            }
            _ => (),
        }
    }

    fn check_expr_post(&mut self, _: &LateContext<'_>, e: &Expr<'_>) {
        self.arithmetic_context.expr_post(e.hir_id);
    }

    fn check_body(&mut self, cx: &LateContext<'tcx>, b: &'tcx Body<'_>) {
        self.arithmetic_context.enter_body(cx, b);
    }

    fn check_body_post(&mut self, cx: &LateContext<'tcx>, b: &'tcx Body<'_>) {
        self.arithmetic_context.body_post(cx, b);
    }
}

#[derive(Default)]
pub struct ArithmeticContext {
    expr_id: Option<hir::HirId>,
    /// This field is used to check whether expressions are constants, such as in enum discriminants
    /// and consts
    const_span: Option<Span>,
}
impl ArithmeticContext {
    fn skip_expr(&mut self, e: &hir::Expr<'_>) -> bool {
        self.expr_id.is_some() || self.const_span.map_or(false, |span| span.contains(e.span))
    }

    pub fn check_binary<'tcx>(
        &mut self,
        cx: &LateContext<'tcx>,
        expr: &'tcx hir::Expr<'_>,
        op: hir::BinOpKind,
        l: &'tcx hir::Expr<'_>,
        r: &'tcx hir::Expr<'_>,
    ) {
        if self.skip_expr(expr) {
            return;
        }
        match op {
            hir::BinOpKind::And
            | hir::BinOpKind::Or
            | hir::BinOpKind::BitAnd
            | hir::BinOpKind::BitOr
            | hir::BinOpKind::BitXor
            | hir::BinOpKind::Eq
            | hir::BinOpKind::Lt
            | hir::BinOpKind::Le
            | hir::BinOpKind::Ne
            | hir::BinOpKind::Ge
            | hir::BinOpKind::Gt => return,
            _ => (),
        }

        let (l_ty, r_ty) = (
            cx.typeck_results().expr_ty(l),
            cx.typeck_results().expr_ty(r),
        );
        if l_ty.peel_refs().is_integral() && r_ty.peel_refs().is_integral() {
            match op {
                hir::BinOpKind::Div | hir::BinOpKind::Rem => match &r.kind {
                    hir::ExprKind::Lit(_lit) => (),
                    hir::ExprKind::Unary(hir::UnOp::Neg, expr) => {
                        if is_integer_literal(expr, 1) {
                            Detector::IntegerOverflowOrUnderflow.span_lint_and_help(
                                cx,
                                INTEGER_OVERFLOW_UNDERFLOW,
                                expr.span,
                                "Potential for integer arithmetic overflow/underflow in unary operation with negative expression. Consider checked, wrapping or saturating arithmetic.",
                            );
                            self.expr_id = Some(expr.hir_id);
                        }
                    }
                    _ => {
                        Detector::IntegerOverflowOrUnderflow.span_lint_and_help(
                            cx,
                            INTEGER_OVERFLOW_UNDERFLOW,
                            expr.span,
                            &format!("Potential for integer arithmetic overflow/underflow in operation '{}'. Consider checked, wrapping or saturating arithmetic.", op.as_str()),
                        );
                        self.expr_id = Some(expr.hir_id);
                    }
                },
                _ => {
                    Detector::IntegerOverflowOrUnderflow.span_lint_and_help(
                        cx,
                        INTEGER_OVERFLOW_UNDERFLOW,
                        expr.span,
                        &format!("Potential for integer arithmetic overflow/underflow in operation '{}'. Consider checked, wrapping or saturating arithmetic.", op.as_str()),
                    );
                    self.expr_id = Some(expr.hir_id);
                }
            }
        }
    }

    pub fn check_negate<'tcx>(
        &mut self,
        cx: &LateContext<'tcx>,
        expr: &'tcx hir::Expr<'_>,
        arg: &'tcx hir::Expr<'_>,
    ) {
        if self.skip_expr(expr) {
            return;
        }
        let ty = cx.typeck_results().expr_ty(arg);
        if constant_simple(cx, cx.typeck_results(), expr).is_none() && ty.is_integral() {
            Detector::IntegerOverflowOrUnderflow.span_lint_and_help(
                cx,
                INTEGER_OVERFLOW_UNDERFLOW,
                expr.span,
                "Potential for integer arithmetic overflow/underflow. Consider checked, wrapping or saturating arithmetic.",
            );
            self.expr_id = Some(expr.hir_id);
        }
    }

    pub fn expr_post(&mut self, id: hir::HirId) {
        if Some(id) == self.expr_id {
            self.expr_id = None;
        }
    }

    pub fn enter_body(&mut self, cx: &LateContext<'_>, body: &hir::Body<'_>) {
        let body_owner = cx.tcx.hir().body_owner(body.id());
        let body_owner_def_id = cx.tcx.hir().body_owner_def_id(body.id());

        match cx.tcx.hir().body_owner_kind(body_owner_def_id) {
            hir::BodyOwnerKind::Static(_) | hir::BodyOwnerKind::Const => {
                let body_span = cx.tcx.hir().span_with_body(body_owner);

                if let Some(span) = self.const_span {
                    if span.contains(body_span) {
                        return;
                    }
                }
                self.const_span = Some(body_span);
            }
            hir::BodyOwnerKind::Fn | hir::BodyOwnerKind::Closure => (),
        }
    }

    pub fn body_post(&mut self, cx: &LateContext<'_>, body: &hir::Body<'_>) {
        let body_owner = cx.tcx.hir().body_owner(body.id());
        let body_span = cx.tcx.hir().span_with_body(body_owner);

        if let Some(span) = self.const_span {
            if span.contains(body_span) {
                return;
            }
        }
        self.const_span = None;
    }
}
