#![feature(rustc_private)]
#![warn(unused_extern_crates)]
#![feature(let_chains)]
extern crate rustc_error_messages;
extern crate rustc_hir;
extern crate rustc_span;

use clippy_wrappers::span_lint_and_note;
use itertools::Itertools;
use rustc_error_messages::MultiSpan;
use rustc_hir::GenericArg;
use rustc_hir::{
    intravisit::{walk_expr, Visitor},
    Expr, GenericArgs, QPath, TyKind,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Span;

scout_audit_dylint_linting::impl_late_lint! {
    pub AVOID_AUTOKEY_UPGRADABLE,
    Warn,
    "",
    AvoidAutokeyUpgradable::default(),
    {
        name: "Avoid AutoKey Upgradable",
        long_message: "Avoid using `Lazy` fields without `ManualKey` in upgradable contracts. This could lead to a locked contract after an upgrade.",
        severity: "Critical",
        help: "https://coinfabrik.github.io/scout/docs/vulnerabilities/avoid-autokey-upgradable",
        vulnerability_class: "Upgradability",
    }
}

const LAZY_TYPE: &str = "ink_storage::lazy::Lazy";
const MAPPING_TYPE: &str = "ink_storage::lazy::mapping::Mapping";
const INK_VEC_STORAGE_TYPE: &str = "ink_storage::lazy::vec::StorageVec";

const SET_CODE_HASH_METHOD: &str = "set_code_hash";

#[derive(Default)]
pub struct AvoidAutokeyUpgradable {
    lazy_fields: Vec<Span>,
}

impl AvoidAutokeyUpgradable {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'tcx> LateLintPass<'tcx> for AvoidAutokeyUpgradable {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: rustc_hir::intravisit::FnKind<'tcx>,
        _: &'tcx rustc_hir::FnDecl<'tcx>,
        body: &'tcx rustc_hir::Body<'tcx>,
        _: rustc_span::Span,
        _: rustc_span::def_id::LocalDefId,
    ) {
        let mut aau_storage = AvoidAutokeyUpgradableVisitor {
            cx,
            lazy_fields: &mut self.lazy_fields,
        };
        walk_expr(&mut aau_storage, body.value);
    }

    fn check_field_def(&mut self, cx: &LateContext<'tcx>, field: &'tcx rustc_hir::FieldDef<'tcx>) {
        if self.lazy_value_has_manual_key(cx, field) {
            self.lazy_fields.push(field.span);
        }
    }
}

fn is_lazy_type(def_path: &String) -> bool {
    def_path == LAZY_TYPE || def_path == MAPPING_TYPE || def_path == INK_VEC_STORAGE_TYPE
}
impl<'tcx> AvoidAutokeyUpgradable {
    fn lazy_value_has_manual_key(
        &mut self,
        cx: &LateContext<'_>,
        field: &'tcx rustc_hir::FieldDef<'tcx>,
    ) -> bool {
        //check if the type is lazy
        if let TyKind::Path(QPath::Resolved(Some(ty), _)) = field.ty.kind
            && let TyKind::Path(QPath::Resolved(None, p)) = ty.kind
            && let Some(did) = p.res.opt_def_id()
            && let def_path = cx
                .get_def_path(did)
                .iter()
                .map(|x| x.to_string())
                .join("::")
            && is_lazy_type(&def_path)
        {
            if let Some(GenericArgs { args, .. }) = p.segments[0].args
                && let Some(GenericArg::Type(ty)) = args.iter().last()
                && let TyKind::Path(QPath::Resolved(None, p)) = ty.kind
            {
                return p.segments[0].ident.name.to_string() != "ManualKey";
            }
            return true;
        }
        false
    }
}

struct AvoidAutokeyUpgradableVisitor<'tcx, 'tcx_ref> {
    cx: &'tcx_ref LateContext<'tcx>,
    lazy_fields: &'tcx_ref mut Vec<Span>,
}

impl<'tcx> Visitor<'tcx> for AvoidAutokeyUpgradableVisitor<'tcx, '_> {
    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
        if let Some(v) = expr.method_ident()
            && v.name.to_string() == SET_CODE_HASH_METHOD
            && !self.lazy_fields.is_empty()
        {
            let mut spans: MultiSpan = MultiSpan::from_spans(
                self.lazy_fields
                    .iter()
                    .dedup()
                    .copied()
                    .collect::<Vec<Span>>(),
            );

            spans.push_span_label(
                *self.lazy_fields.iter().last().unwrap(),
                "These fields have an automatic storage key generation",
            );

            spans.push_span_label(expr.span, "This makes the contract upgradable");

            span_lint_and_note(
                        self.cx,
                        AVOID_AUTOKEY_UPGRADABLE,
                        spans,
                        "Avoid using `Lazy` fields without `ManualKey` in upgradable contracts",
                        None,
                        "For more information, see: \n[#171](https://github.com/CoinFabrik/scout/issues/171) \
                            \n[Manual vs. Automatic Key Generation](https://use.ink/datastructures/storage-layout/#manual-vs-automatic-key-generation)"
                        ,
                    );
        }
        walk_expr(self, expr)
    }
}
