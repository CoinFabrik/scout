#![feature(rustc_private)]
#![warn(unused_extern_crates)]
#![feature(let_chains)]
extern crate rustc_hir;
extern crate rustc_span;
use std::collections::HashMap;

use rustc_hir::{
    intravisit::{walk_expr, Visitor},
    Expr, ExprKind, GenericArg, QPath, Ty, TyKind,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::symbol::Ident;
use rustc_span::Span;
use clippy_utils::diagnostics::span_lint_and_help;
const LINT_MESSAGE: &str = "Do not use these method with an unsized (dynamically sized) type.";
scout_audit_dylint_linting::impl_late_lint! {
    pub BUFFERING_UNSIZED_TYPES,
    Warn,
    LINT_MESSAGE,
    BufferingUnsizedTypes::default(),
    {
        name: "Buffering unsized types",
        long_message: "",
        severity: "Enhancement",
        help: "https://coinfabrik.github.io/scout/docs/vulnerabilities/vec-considerations",
        vulnerability_class: "Best practices",
    }
}
const INK_MAPPING_TYPE: &str = "ink_storage::lazy::mapping::Mapping";
const INK_VEC_STORAGE_TYPE: &str = "ink_storage::lazy::vec::StorageVec";
const FUNCTIONS_TO_CHECK: [&str; 6] = ["insert", "pop", "push", "set", "peek", "get"];
const UNSIZED_TYPES: [&str; 2] = ["String", "Vec"];
pub fn method_is_wanted(method: &str) -> bool {
    FUNCTIONS_TO_CHECK.contains(&method)
}
#[derive(Default)]
pub struct BufferingUnsizedTypes {
    mapping_fields: HashMap<Ident, Span>,
}
impl BufferingUnsizedTypes {
    pub fn new() -> Self {
        Self::default()
    }
}
fn is_lazy_type_with_unsized_generic_arg<'tcx>(
    cx: &LateContext<'_>,
    field: &'tcx rustc_hir::FieldDef<'tcx>,
) -> Option<String> {
    // first check if the ty is a lazy/buffered type
    if let TyKind::Path(QPath::Resolved(Some(ty), _)) = field.ty.kind
        && let TyKind::Path(QPath::Resolved(None, p)) = ty.kind
        && p.res.opt_def_id().is_some()
        // any generic arg of the type is a known unsized type
        && p.segments.iter().any(|seg| {
            seg.args().args.iter().any(|arg| {
                if let GenericArg::Type(Ty { kind, .. }) = arg
                    && let TyKind::Path(QPath::Resolved(_, pth)) = kind
                {
                    pth.segments.iter().any(|&pthsgs| {
                        UNSIZED_TYPES.contains(&pthsgs.ident.name.to_string().as_str())
                    })
                } else {
                    false
                }
            })
        })
    {
        //if it is, return the def_path of the type
        return cx
            .get_def_path(p.res.def_id())
            .iter()
            .map(|x| x.to_string())
            .reduce(|a, b| a + "::" + &b);
    };
    None
}
impl<'tcx> BufferingUnsizedTypes {
    fn is_lazy_storage(
        &self,
        cx: &LateContext<'_>,
        field: &'tcx rustc_hir::FieldDef<'tcx>,
    ) -> bool {
        if let Some(def_path) = is_lazy_type_with_unsized_generic_arg(cx, field) {
            return def_path == INK_MAPPING_TYPE || def_path == INK_VEC_STORAGE_TYPE;
        }
        false
    }
}
impl<'tcx> LateLintPass<'tcx> for BufferingUnsizedTypes {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: rustc_hir::intravisit::FnKind<'tcx>,
        _: &'tcx rustc_hir::FnDecl<'tcx>,
        body: &'tcx rustc_hir::Body<'tcx>,
        _: rustc_span::Span,
        _: rustc_span::def_id::LocalDefId,
    ) {
        let mut aau_storage = BufferingUnsizedTypesVisitor {
            cx,
            mapping_fields: &mut self.mapping_fields,
        };
        walk_expr(&mut aau_storage, body.value);
    }
    fn check_field_def(&mut self, cx: &LateContext<'tcx>, field: &'tcx rustc_hir::FieldDef<'tcx>) {
        if self.is_lazy_storage(cx, field) {
            self.mapping_fields.insert(field.ident, field.span);
        }
    }
}
struct BufferingUnsizedTypesVisitor<'tcx, 'tcx_ref> {
    cx: &'tcx_ref LateContext<'tcx>,
    mapping_fields: &'tcx_ref mut HashMap<Ident, Span>,
}
impl<'tcx> Visitor<'tcx> for BufferingUnsizedTypesVisitor<'tcx, '_> {
    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
        if let ExprKind::MethodCall(p, q, _, _) = expr.kind
            && method_is_wanted(&p.ident.name.to_string())
            && let ExprKind::Field(path_expr, struct_field_name) = q.kind
            && let ExprKind::Path(QPath::Resolved(_, path), ..) = path_expr.kind
            && path
                .segments
                .iter()
                .any(|x| x.ident.name.to_string() == "self")
            && self.mapping_fields.keys().any(|x| x == &struct_field_name)
        {
            let met_name = p.ident.name.to_string();
            let field_name = struct_field_name.name.to_string();
            span_lint_and_help(
                self.cx,
                BUFFERING_UNSIZED_TYPES,
                expr.span,
                &format!(
                    "Do not use `{}` with an unsized type. Use the `try_{}` method instead.",
                    met_name, met_name
                ),
                Some(*self.mapping_fields.get(&struct_field_name).unwrap()),
                &format!(
                    "The variable `{}` is a storage type with a dinamically sized value.",
                    field_name
                ),
            );
        }
        walk_expr(self, expr)
    }
}
