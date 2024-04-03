#![feature(rustc_private)]
#![warn(unused_extern_crates)]
#![feature(let_chains)]
extern crate rustc_hir;
extern crate rustc_span;

use rustc_hir::GenericArg;
use rustc_hir::{
    intravisit::{walk_expr, Visitor},
    Expr, QPath, TyKind,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Span;
use scout_audit_clippy_utils::diagnostics::span_lint_and_help;
//use scout_audit_internal::{DetectorImpl, InkDetector as Detector};

dylint_linting::impl_late_lint! {
    pub VEC_CONSIDERATIONS,
    Warn,
    "",
    VecConsiderations::default()
}

#[derive(Default)]
pub struct VecConsiderations {
    mapping_fields: Vec<Span>,
}

impl VecConsiderations {
    pub fn new() -> Self {
        Self::default()
    }
}

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
            && cx
                .get_def_path(p.res.def_id())
                .iter()
                .map(|x| x.to_string())
                .reduce(|a, b| a + "::" + &b)
                .unwrap()
                == "ink_storage::lazy::Lazy"
        {
            if let Some(gargs) = p.segments[0].args
                && gargs.args.len() > 1
                && let GenericArg::Type(ty) = gargs.args[1]
                && let TyKind::Path(QPath::Resolved(None, p)) = ty.kind
                && p.segments[0].ident.to_string().contains("ManualKey")
            {
            } else {
                self.mapping_fields.push(field.span);
            }
        }
    }
}

struct VecConsiderationsVisitor<'tcx, 'tcx_ref> {
    cx: &'tcx_ref LateContext<'tcx>,
    mapping_fields: &'tcx_ref mut Vec<Span>,
}

impl<'tcx> Visitor<'tcx> for VecConsiderationsVisitor<'tcx, '_> {
    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
        if let Some(v) = expr.method_ident()
            && v.name.to_string() == "set_code_hash".to_string()
        {
            self.mapping_fields.iter().for_each(
                |lazy_field| {
                    span_lint_and_help(
                        self.cx,
                        
                        VEC_CONSIDERATIONS,
                        lazy_field.to(expr.span),
                        "Avoid using `Lazy` fields without `ManualKey` in upgradable contracts",
                        Some(*lazy_field),
                        &format!(
                            "Consider using `Lazy` fields with `ManualKey<...>` instead of leaving it to the compiler \
                            \nThis will allow you to upgrade the contract without losing the data stored in the `Lazy` field. \
                            \nFor more information, see: \n[#171](https://github.com/CoinFabrik/scout/issues/171) \
                            \n[Manual vs. Automatic Key Generation](https://use.ink/datastructures/storage-layout/#manual-vs-automatic-key-generation)"
                        ),
                    );
                }
            );
        }
        walk_expr(self, expr)
    }
}
