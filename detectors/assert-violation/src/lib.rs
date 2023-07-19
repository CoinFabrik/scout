#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_span;

use clippy_utils::diagnostics::span_lint;
use if_chain::if_chain;
use rustc_ast::ast::LitKind;
use rustc_hir::intravisit::Visitor;
use rustc_hir::intravisit::{walk_expr, FnKind};
use rustc_hir::{Body, FnDecl, HirId};
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Span;

dylint_linting::declare_late_lint! {
    /// ### What it does
    /// Checks for `assert!` usage.
    /// ### Why is this bad?
    /// `assert!` causes a panic, and panicking it's not a good practice. Instead, use proper error handling.
    /// ### Example
    /// ```rust
    ///    #[ink(message)]
    ///    pub fn assert_if_greater_than_10(&self, value: u128) -> bool {
    ///        assert!(value <= 10, "value should be less than 10");
    ///        true
    ///    }
    ///     ```
    /// Use instead:
    ///```rust
    ///     #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    ///     #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    ///     pub enum Error {
    ///         GreaterThan10,
    ///     }
    ///
    ///    #[ink(message)]
    ///    pub fn revert_if_greater_than_10(&self, value: u128) -> Result<bool, Error> {
    ///        if value <= 10 {
    ///            return Ok(true)
    ///        } else {
    ///            return Err(Error::GreaterThan10)
    ///        }
    ///    }
    ///```

    pub ASSERT_VIOLATION,
    Warn,
    "Assert causes panic. Instead, return a proper error."
}
impl<'tcx> LateLintPass<'tcx> for AssertViolation {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        _: &'tcx FnDecl<'_>,
        body: &'tcx Body<'_>,
        _: Span,
        _: HirId,
    ) {
        struct AssertViolationStorage {
            span: Vec<Span>,
        }

        impl<'tcx> Visitor<'tcx> for AssertViolationStorage {
            fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
                if_chain! {
                    if let ExprKind::If(_, b, _) = expr.kind;
                    if let ExprKind::Block(block, _) = b.kind;
                    if let Some(expr1) = block.expr;
                    if let ExprKind::Call(_, args) = expr1.kind;
                    if let ExprKind::Call(_, args2) = args[0].kind;
                    if args2.len() == 2;
                    if let ExprKind::AddrOf(_, _, expr2) = args2[0].kind;
                    if let ExprKind::Array(args3) = expr2.kind;
                    if let ExprKind::Lit(lit) = &args3[0].kind;
                    if let LitKind::Str(_, _) = lit.node;
                    then {
                        self.span.push(b.span);
                    }
                }

                walk_expr(self, expr);
            }
        }
        let mut av_storage = AssertViolationStorage { span: Vec::new() };

        walk_expr(&mut av_storage, body.value);

        av_storage.span.iter().for_each(|span| {
            let error_msg = "Assert causes panic. Instead, return a proper error";
            if let Some(span_outer) = span.ctxt().outer_expn().expansion_cause() {
                span_lint(cx, ASSERT_VIOLATION, span_outer, error_msg);
            } else {
                span_lint(cx, ASSERT_VIOLATION, *span, error_msg);
            }
        });
    }
}
