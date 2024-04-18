#![feature(rustc_private)]
#![warn(unused_extern_crates)]
#![feature(let_chains)]
extern crate rustc_hir;
extern crate rustc_span;

use std::collections::HashMap;

use rustc_hir::{
    intravisit::{walk_expr, Visitor},
    Expr, ExprKind, QPath, TyKind,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::symbol::Ident;
use rustc_span::Span;
use scout_audit_clippy_utils::diagnostics::span_lint_and_help;

const LINT_MESSAGE: &str = "Do not use these method with an unsized (dynamically sized) type.";

dylint_linting::impl_late_lint! {
    pub VEC_CONSIDERATIONS,
    Warn,
    LINT_MESSAGE,
    VecConsiderations::default(),
    {
        name: "Vec Considerations",
        long_message: "",
        severity: "Enhancement",
        help: "https://coinfabrik.github.io/scout/docs/vulnerabilities/vec-considerations",
        vulnerability_class: "Best practices",
    }
}

const INK_MAPPING_TYPE: &str = "ink_storage::lazy::mapping::Mapping";
const INK_VEC_STORAGE_TYPE: &str = "ink_storage::lazy::vec::StorageVec";
const FUNCTIONS_TO_CHECK: [&str; 5] = ["insert", "pop", "push", "set", "peek"];

pub fn method_is_wanted(method: &str) -> bool {
    FUNCTIONS_TO_CHECK.contains(&method)
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

impl<'tcx> VecConsiderations {
    fn is_lazy_storage(
        &self,
        cx: &LateContext<'_>,
        field: &'tcx rustc_hir::FieldDef<'tcx>,
    ) -> bool {
        if let TyKind::Path(QPath::Resolved(Some(ty), _)) = field.ty.kind
            && let TyKind::Path(QPath::Resolved(None, p)) = ty.kind
            && p.res.opt_def_id().is_some()
            && let Some(def_path) = cx
                .get_def_path(p.res.def_id())
                .iter()
                .map(|x| x.to_string())
                .reduce(|a, b| a + "::" + &b)
        {
            return def_path == INK_MAPPING_TYPE || def_path == INK_VEC_STORAGE_TYPE;
        }
        false
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
        if self.is_lazy_storage(cx, &field) {
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
        if let ExprKind::MethodCall(p, q, _, _) = expr.kind
            && method_is_wanted(&p.ident.name.to_string())
            && let ExprKind::Field(path_expr, struct_field_name) = q.kind
            && let ExprKind::Path(QPath::Resolved(_, path), ..) = path_expr.kind
            && path
                .segments
                .into_iter()
                .any(|x| x.ident.name.to_string() == "self")
            && self.mapping_fields.keys().any(|x| x == &struct_field_name)
        {
            let met_name = p.ident.name.to_string();
            let field_name = struct_field_name.name.to_string();
            span_lint_and_help(
                self.cx,
                VEC_CONSIDERATIONS,
                expr.span,
                &format!(
                    "Do not use `{}` with an unsized type. Use the `try_{}` method instead.",
                    met_name, met_name
                ),
                Some(self.mapping_fields.get(&struct_field_name).unwrap().clone()),
                &format!(
                    "The variable `{}` is a storage type with a dinamically sized value.",
                    field_name
                ),
            );
        }

        walk_expr(self, expr)
    }
}
