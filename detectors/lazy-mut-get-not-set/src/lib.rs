#![feature(rustc_private)]
#![warn(unused_extern_crates)]
#![feature(let_chains)]

extern crate rustc_hir;
extern crate rustc_span;

use clippy_utils::diagnostics::{self, span_lint};
//use dylint_linting::declare_late_lint;
use if_chain::if_chain;
use rustc_hir::{
    intravisit::{walk_expr, Visitor},
    Expr, ExprKind, QPath, TyKind,
};
use rustc_lint::{LateContext, LateLintPass};
//use rustc_hir::{LateContext, LateLintPass};
use rustc_span::Span;

dylint_linting::declare_late_lint! {
    pub AVOID_AUTOKEY_UPGRADABLE,
    Warn,
    "UN MENSAJEEEEEEEEEEE"
}

// espub struct AvoidAutokeyUpgradable {
//     is_lazy: Vec<String>,
// }

impl<'tcx> LateLintPass<'tcx> for AvoidAutokeyUpgradable {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: rustc_hir::intravisit::FnKind<'tcx>,
        _: &'tcx rustc_hir::FnDecl<'tcx>,
        body: &'tcx rustc_hir::Body<'tcx>, //una sola expresion, block de expression
        _: rustc_span::Span,
        _: rustc_span::def_id::LocalDefId,
    ) {
        let mut au_storage = AvoidAutokeyUpgradableVisitor {
            is_lazy: Vec::new(),
            has_sets: std::collections::HashMap::new(),
            has_gets: std::collections::HashMap::new(),
            span: std::collections::HashMap::new(),
        };

        walk_expr(&mut au_storage, body.value);
    }

    fn check_field_def(&mut self, cx: &LateContext<'tcx>, field: &'tcx rustc_hir::FieldDef<'tcx>) {}

    fn check_stmt(&mut self, _: &LateContext<'tcx>, _: &'tcx rustc_hir::Stmt<'tcx>) {}
}

pub struct AvoidAutokeyUpgradableVisitor {
    is_lazy: Vec<String>,
    has_sets: std::collections::HashMap<String, bool>,
    has_gets: std::collections::HashMap<String, bool>,
    span: std::collections::HashMap<String, Span>,
}

impl<'tcx> Visitor<'tcx> for AvoidAutokeyUpgradableVisitor {
    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
        walk_expr(self, expr)
    }
}
