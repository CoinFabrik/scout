#![feature(rustc_private)]
#![warn(unused_extern_crates)]
#![feature(let_chains)]
extern crate rustc_hir;
extern crate rustc_span;

use std::vec::Vec;

use rustc_hir::{
    intravisit::{walk_expr, Visitor},
    BlockCheckMode, Expr, ExprKind, UnsafeSource,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Span;
use scout_audit_clippy_utils::diagnostics;

const LINT_MESSAGE: &str = "Avoid using unsafe blocks as it may lead to undefined behavior.";
dylint_linting::impl_late_lint! {
    pub AVOID_UNSAFE_BLOCK,
    Warn,
    LINT_MESSAGE,
    AvoidUnsafeBlock::default(),
    {
        name: "Avoid unsafe block",
        long_message: "The unsafe block is used to bypass Rust's safety checks. It is recommended to avoid using unsafe blocks as much as possible, and to use them only when necessary.    ",
        severity: "Enhancement",
        help: "https://coinfabrik.github.io/scout/docs/vulnerabilities/avoid-unsafe-block",
        vulnerability_class: "Best practices",
    }
}

#[derive(Default)]
struct AvoidUnsafeBlock {}

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
        let mut unsafety = AvoidUnsafeBlockVisitor { span: Vec::new() };

        walk_expr(&mut unsafety, body.value);

        unsafety
            .span
            .iter()
            .for_each(|sp| diagnostics::span_lint(cx, AVOID_UNSAFE_BLOCK, *sp, LINT_MESSAGE));
    }
}

struct AvoidUnsafeBlockVisitor {
    span: Vec<Span>,
}

impl<'tcx> Visitor<'tcx> for AvoidUnsafeBlockVisitor {
    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
        if let ExprKind::Block(block, _) = expr.kind
            && block.rules == BlockCheckMode::UnsafeBlock(UnsafeSource::UserProvided)
            && block.span.in_macro_expansion_with_collapse_debuginfo()
        {
            self.span.push(expr.span);
        }
        walk_expr(self, expr)
    }
}
