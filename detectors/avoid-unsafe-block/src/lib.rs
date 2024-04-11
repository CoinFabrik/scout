#![feature(rustc_private)]
#![warn(unused_extern_crates)]
#![feature(let_chains)]
extern crate rustc_hir;
extern crate rustc_span;

use std::vec::Vec;
use rustc_hir::{
    intravisit::{walk_expr, Visitor},
    Expr, ExprKind, BlockCheckMode,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Span;
use scout_audit_clippy_utils::diagnostics;


dylint_linting::impl_late_lint! {
    pub AVOID_UNSAFE_BLOCK,
    Warn,
    "Avoid using unsafe blocks as it may lead to undefined behavior.",
    AvoidUnsafeBlock::default()
}


#[derive(Default)]
struct AvoidUnsafeBlock{}

impl<'tcx> LateLintPass<'tcx> for AvoidUnsafeBlock {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: rustc_hir::intravisit::FnKind<'tcx>,
        _: &'tcx rustc_hir::FnDecl<'tcx>,
        body: &'tcx rustc_hir::Body<'tcx>,
        _: rustc_span::Span,
        _: rustc_span::def_id::LocalDefId,
    ) {
        let mut unsafety = AvoidUnsafeBlockVisitor {
            is_unsafe: false,
            span: Vec::new(),
        };

        walk_expr(&mut unsafety, body.value);

        if unsafety.is_unsafe{
            for var in unsafety.span.iter(){

                diagnostics::span_lint(
                    cx,
                    AVOID_UNSAFE_BLOCK,
                    *var,
                    "Avoid using unsafe blocks as it may lead to undefined behavior."
                )

            }
        }
    }

}


struct AvoidUnsafeBlockVisitor {
    is_unsafe: bool,
    span: Vec<Span>
}

impl<'tcx> Visitor<'tcx> for AvoidUnsafeBlockVisitor {
    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {

        if let ExprKind::Block(block, _) = expr.kind
        && let BlockCheckMode::UnsafeBlock(_) = block.rules {
            self.is_unsafe = true;
            self.span.push(expr.span);
        }

        walk_expr(self, expr)
    }
}