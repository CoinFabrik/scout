#![feature(rustc_private)]
#![warn(unused_extern_crates)]
#![feature(let_chains)]
extern crate rustc_hir;
extern crate rustc_span;

use std::collections::HashMap;
use rustc_hir::{
    intravisit::{walk_expr, Visitor},
    Expr, QPath, TyKind, ExprKind
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Span;
use scout_audit_clippy_utils::diagnostics::span_lint_and_help;
use rustc_span::symbol::Ident;

dylint_linting::impl_late_lint! {
    pub VEC_CONSIDERATIONS,
    Warn,
    "Do not use `.insert(..)` with an unsized (dynamically sized) type.",
    VecConsiderations::default()
}

#[derive(Default)]
pub struct VecConsiderations {
    mapping_fields: HashMap<Ident, Span>,
}

impl VecConsiderations {
    pub fn new() -> Self {
        Self::default()
    }
}

const INK_MAPPING_TYPE : &str = "ink_storage::lazy::mapping::Mapping";

impl<'tcx> LateLintPass<'tcx> for VecConsiderations {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: rustc_hir::intravisit::FnKind<'tcx>,
        _: &'tcx rustc_hir::FnDecl<'tcx>,
        body: &'tcx rustc_hir::Body<'tcx>,
        _: rustc_span::Span,
        _: rustc_span::def_id::LocalDefId,
    ) {
        let mut aau_storage = VecConsiderationsVisitor {
            cx,
            mapping_fields: &mut self.mapping_fields,
        };
        walk_expr(&mut aau_storage, body.value);
    }

    fn check_field_def(&mut self, cx: &LateContext<'tcx>, field: &'tcx rustc_hir::FieldDef<'tcx>) {
        if let TyKind::Path(QPath::Resolved(Some(ty), _)) = field.ty.kind
            && let TyKind::Path(QPath::Resolved(None, p)) = ty.kind
            && p.res.opt_def_id().is_some() 
            && cx
                .get_def_path(p.res.def_id())
                .iter()
                .map(|x| x.to_string())
                .reduce(|a, b| a + "::" + &b)
                .unwrap()
                == INK_MAPPING_TYPE
        {
                self.mapping_fields.insert(field.ident, field.span);
        }
    }
}

struct VecConsiderationsVisitor<'tcx, 'tcx_ref> {
    cx: &'tcx_ref LateContext<'tcx>,
    mapping_fields: &'tcx_ref mut HashMap<Ident, Span>,
}

impl<'tcx> Visitor<'tcx> for VecConsiderationsVisitor<'tcx, '_> {
    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {

        if let ExprKind::MethodCall(p,q,_,_) = expr.kind
        && p.ident.name.to_string() == "insert" 
        && let ExprKind::Field(ex, id) = q.kind 
        && let ExprKind::Path(QPath::Resolved(_, path), ..) = ex.kind 
        && path.segments.into_iter().any(|x| x.ident.name.to_string() == "self") 
        && self.mapping_fields.keys().any(|x| x == &id) {
            self.mapping_fields.iter().for_each(
                |(ident, span)| {
                    span_lint_and_help(
                        self.cx,
                        VEC_CONSIDERATIONS,
                        expr.span,
                        "Do not use `.insert(..)` with an unsized (dynamically sized) type.",
                        Some(*span),
                        &format!("The variable `{}` is a Mapping with a dinamically sized value.", ident.name.to_string()),
                    );
                }
            );
        }

        walk_expr(self, expr)
    }
}
