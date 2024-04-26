#![feature(rustc_private)]
#![warn(unused_extern_crates)]
#![feature(let_chains)]

extern crate rustc_hir;
extern crate rustc_span;

use std::collections::HashMap;

use rustc_hir::{
    def_id::LocalDefId,
    intravisit::{walk_expr, FnKind, Visitor},
    BindingAnnotation, Body, Expr, ExprKind, FieldDef, FnDecl, Local, Mutability, Pat, PatKind,
    PathSegment, QPath, Stmt, StmtKind,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Span;
use scout_audit_clippy_utils::diagnostics::span_lint_and_help;

const LINT_MESSAGE: &str = "Use `set` method to update the value.";
const NOTE_MESSAGE: &str = "If you don't want to set the value, consider removing `mut`";

dylint_linting::impl_late_lint! {
    pub LAZY_MUT_GET_NOT_SET,
    Warn,
    LINT_MESSAGE,
    LazyMutGetNotSet::default(),
    {
        name: "Lazy mut get not set",
        long_message: "Data inside it is not automatically flushed to the underlying storage, so it is necessary to call the `set` method to update the value. If you don't want to set the value, consider removing `mut`.",
        severity: "Critical",
        help: "https://coinfabrik.github.io/scout/docs/vulnerabilities/lazy-mut-get-not-set",
        vulnerability_class: "Best practices",
    }
}

#[derive(Default)]
struct LazyMutGetNotSet {
    lazy_fields: HashMap<String, Span>,
}

impl<'tcx> LateLintPass<'tcx> for LazyMutGetNotSet {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        _: &'tcx FnDecl<'_>,
        body: &'tcx Body<'_>,
        _: Span,
        _: LocalDefId,
    ) {
        let mut visitor = LazyMutGetNotSetVisitor::<'_> {
            get_span: HashMap::new(),
            set_span: HashMap::new(),
            lazy_fields: &self.lazy_fields,
        };

        walk_expr(&mut visitor, body.value);

        visitor
            .get_span
            .keys()
            .filter(|k| !visitor.set_span.contains_key(*k))
            .for_each(|k| {
                let span = visitor.get_span.get(k).unwrap();
                span_lint_and_help(
                    cx,
                    LAZY_MUT_GET_NOT_SET,
                    *span,
                    LINT_MESSAGE,
                    None,
                    NOTE_MESSAGE,
                );
            });
    }
    fn check_field_def(&mut self, _: &LateContext<'tcx>, fdef: &'tcx FieldDef<'tcx>) {
        self.lazy_fields
            .insert(fdef.ident.name.to_string(), fdef.span);
    }
}

struct LazyMutGetNotSetVisitor<'tcx_ref> {
    get_span: HashMap<String, Span>,
    set_span: HashMap<String, Span>,
    lazy_fields: &'tcx_ref HashMap<String, Span>,
}

impl LazyMutGetNotSetVisitor<'_> {
    fn method_is_self_lazy_field_get(&mut self, methods: &[String]) -> bool {
        methods.len() > 2
            && methods[0] == "self"
            && self.lazy_fields.keys().any(|k| k == &methods[1])
            && methods[2] == "get"
    }
    fn method_is_self_lazy_field_set(&mut self, methods: &[String]) -> bool {
        methods.len() > 2
            && methods[0] == "self"
            && self.lazy_fields.keys().any(|k| k == &methods[1])
            && methods[2] == "set"
    }
}

impl<'tcx> Visitor<'tcx> for LazyMutGetNotSetVisitor<'_> {
    fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
        let methods = get_method_path_names(expr);
        if methods.is_empty() {
            return;
        }

        if self.method_is_self_lazy_field_get(&methods) {
            self.get_span.insert(methods[1].clone(), expr.span);
            return;
        }
        if self.method_is_self_lazy_field_set(&methods) {
            self.set_span.insert(methods[1].clone(), expr.span);
            return;
        }

        walk_expr(self, expr);
    }

    fn visit_stmt(&mut self, stmt: &'tcx Stmt<'tcx>) {
        if let StmtKind::Local(Local {
            pat:
                Pat {
                    kind: PatKind::Binding(BindingAnnotation(_, muta), ..),
                    ..
                },
            init: Some(init_expr),
            ..
        }) = stmt.kind
            && muta == &Mutability::Mut
        {
            self.visit_expr(init_expr);
        }

        if let StmtKind::Semi(expr) = stmt.kind {
            self.visit_expr(expr)
        }

        if let StmtKind::Expr(expr) = stmt.kind {
            self.visit_expr(expr)
        }
    }
}

fn get_method_path_names(expr: &Expr<'_>) -> Vec<String> {
    let mut full_method_ident = Vec::new();
    let mut expr = expr;
    'names: loop {
        match expr.kind {
            ExprKind::MethodCall(PathSegment { ident, .. }, rec, ..) => {
                full_method_ident.push(ident.name.to_string());
                expr = rec;
            }
            ExprKind::Field(rec, ident) => {
                full_method_ident.push(ident.name.to_string());
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
    full_method_ident
}
