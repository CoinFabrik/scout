#![feature(rustc_private)]
#![recursion_limit = "256"]
#![feature(let_chains)]
extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;

use std::collections::HashMap;

use itertools::Itertools;
use rustc_hir::intravisit::{walk_expr, FnKind, Visitor};
use rustc_hir::{Body, Expr, ExprKind, FnDecl, GenericArg, GenericArgs, PathSegment, QPath};
use rustc_hir::{Ty, TyKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::def_id::LocalDefId;
use rustc_span::Span;
use scout_audit_clippy_utils::diagnostics::span_lint_and_help;

const LINT_MESSAGE: &str =
    "You are iterating over a vector of tuples using `find`. Consider using a mapping instead.";

const ITERABLE_METHODS: [&str; 1] = ["find"];

dylint_linting::impl_late_lint! {
    pub VEC_COULD_BE_MAPPING,
    Warn,
    LINT_MESSAGE,
    VecCouldBeMapping::default(),
    {
        name: "Vec could be Mapping",
        long_message: "This vector could be a mapping. Consider changing it, because you are using `find` method in a vector of tuples",
        severity: "Enhancement",
        help: "https://coinfabrik.github.io/scout/docs/vulnerabilities/vec-could-be-mapping",
        vulnerability_class: "Gas Usage",
    }
}

#[derive(Default)]
pub struct VecCouldBeMapping {
    storage_names: HashMap<String, Span>,
}

impl<'tcx> LateLintPass<'tcx> for VecCouldBeMapping {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        _: &'tcx FnDecl<'_>,
        body: &'tcx Body<'_>,
        _: Span,
        _: LocalDefId,
    ) {
        let mut vec_mapping_storage = VecMappingStorage {
            storage_names: self.storage_names.clone(),
            uses_as_hashmap: Vec::new(),
        };

        walk_expr(&mut vec_mapping_storage, body.value);

        vec_mapping_storage
            .uses_as_hashmap
            .iter()
            .for_each(|(span, field)| {
                let field_sp = self.storage_names.get(field).copied();

                span_lint_and_help(
                    cx,
                    VEC_COULD_BE_MAPPING,
                    *span,
                    LINT_MESSAGE,
                    field_sp,
                    "Change this to a `ink::storage::Mapping<...>`",
                );
            });
    }

    fn check_field_def(&mut self, _: &LateContext<'tcx>, fdef: &'tcx rustc_hir::FieldDef<'tcx>) {
        if is_vec_type_with_tuple_of_2_elems(fdef) {
            self.storage_names
                .insert(fdef.ident.name.to_string(), fdef.span);
        }
    }
}

struct VecMappingStorage {
    storage_names: HashMap<String, Span>,
    uses_as_hashmap: Vec<(Span, String)>,
}

impl VecMappingStorage {
    fn call_storage_and_any_or_find(&self, methods: &[String]) -> bool {
        methods.first() == Some(&"self".to_string())
            && methods
                .iter()
                .any(|method| ITERABLE_METHODS.contains(&method.as_str()))
            && methods
                .iter()
                .any(|method| self.storage_names.keys().contains(method))
    }
}

impl<'tcx> Visitor<'tcx> for VecMappingStorage {
    fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
        let (methods, field) = get_method_path_names(expr);

        if !methods.is_empty() && self.call_storage_and_any_or_find(&methods) {
            self.uses_as_hashmap.push((expr.span, field));
        } else {
            walk_expr(self, expr);
        }
    }
}

fn get_method_path_names(expr: &Expr<'_>) -> (Vec<String>, String) {
    let mut full_method_ident = Vec::new();
    let mut expr = expr;
    let mut fields = Vec::new();

    'names: loop {
        match expr.kind {
            ExprKind::MethodCall(PathSegment { ident, .. }, rec, ..) => {
                full_method_ident.push(ident.name.to_string());
                expr = rec;
            }
            ExprKind::Field(rec, ident) => {
                full_method_ident.push(ident.name.to_string());
                fields.push(ident.name.to_string());
                expr = rec;
            }
            ExprKind::Path(QPath::Resolved(_, b)) => {
                if !full_method_ident.is_empty() {
                    for seg in b.segments.iter().rev() {
                        full_method_ident.push(seg.ident.name.to_string());
                    }
                }
                break 'names;
            }
            _ => {
                break 'names;
            }
        }
    }
    full_method_ident.reverse();

    let field = if full_method_ident.len() > 1
        && full_method_ident[0] == "self"
        && fields.contains(&full_method_ident[1])
    {
        full_method_ident[1].clone()
    } else {
        "".to_string()
    };

    (full_method_ident, field)
}

fn is_vec_type_with_tuple_of_2_elems(fdef: &'_ rustc_hir::FieldDef<'_>) -> bool {
    if let TyKind::Path(QPath::Resolved(Some(Ty { kind, .. }), ..)) = fdef.ty.kind
        && let TyKind::Path(QPath::Resolved(_, b)) = *kind
        && let Some(last) = b.segments.iter().last()
        && last.ident.name.to_string() == "Vec"
        && let Some(GenericArgs { args, .. }) = last.args
    {
        for arg in args.iter() {
            if let GenericArg::Type(ins) = *arg
                && let TyKind::Tup(insides) = ins.kind
                && insides.len() == 2
            {
                return true;
            }
        }
    }
    false
}
