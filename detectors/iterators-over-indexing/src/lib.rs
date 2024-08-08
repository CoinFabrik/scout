#![feature(rustc_private)]
#![feature(let_chains)]
extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_span;

use rustc_ast::LitKind;
use rustc_hir::def_id::LocalDefId;
use rustc_hir::{
    def::Res,
    intravisit::{walk_expr, FnKind, Visitor},
    Expr, ExprKind, HirId, LangItem, LoopSource, MatchSource, PatKind, QPath, StmtKind,
};
use rustc_lint::LateLintPass;
use rustc_span::Span;

const LINT_MESSAGE: &str =
    "Hardcoding an index could lead to panic if the top bound is out of bounds.";

scout_audit_dylint_linting::declare_late_lint! {
    pub ITERATOR_OVER_INDEXING,
    Warn,
    LINT_MESSAGE,
    {
        name: "Iterator Over Indexing",
        long_message: "The use of iterators over indexing is a best practice that should be followed in Rust. This is because accessing a vector by index is slower than using an iterator. Also, if the index is out of bounds, it will panic.    ",
        severity: "Enhancement",
        help: "https://coinfabrik.github.io/scout/docs/vulnerabilities/iterators-over-indexing",
        vulnerability_class: "Best practices",
    }
}

struct ForLoopVisitor {
    span_constant: Vec<Span>,
}
struct VectorAccessVisitor {
    index_id: HirId,
    has_vector_access: bool,
}

impl<'tcx> Visitor<'tcx> for VectorAccessVisitor {
    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
        if let ExprKind::Index(_, id, _) = expr.kind
            && let ExprKind::Path(qpath) = &id.kind
            && let QPath::Resolved(_, path) = qpath
            && let Res::Local(hir_id) = path.res
            && hir_id == self.index_id
        {
            self.has_vector_access = true;
        }
        walk_expr(self, expr);
    }
}

impl<'tcx> Visitor<'tcx> for ForLoopVisitor {
    fn visit_expr(&mut self, expr: &'tcx rustc_hir::Expr<'tcx>) {
        if let ExprKind::Match(match_expr, arms, source) = expr.kind
            && source == MatchSource::ForLoopDesugar
            && let ExprKind::Call(func, args) = match_expr.kind
            && let ExprKind::Path(qpath) = &func.kind
            && let QPath::LangItem(item, _) = qpath
            && item == &LangItem::IntoIterIntoIter
            && args.first().is_some()
            && let ExprKind::Struct(qpath, fields, _) = args.first().unwrap().kind
            && let QPath::LangItem(langitem, _) = qpath
            && (LangItem::Range == *langitem
                || LangItem::RangeInclusiveStruct == *langitem
                || LangItem::RangeInclusiveNew == *langitem)
            && fields.last().is_some()
            && let ExprKind::Lit(lit) = &fields.last().unwrap().expr.kind
            && let LitKind::Int(_v, _typ) = lit.node
            && arms.first().is_some()
            && let ExprKind::Loop(block, _, loopsource, _) = arms.first().unwrap().body.kind
            && LoopSource::ForLoop == loopsource
            && block.stmts.first().is_some()
            && let StmtKind::Expr(stmtexpr) = block.stmts.first().unwrap().kind
            && let ExprKind::Match(_match_expr, some_none_arms, match_source) = stmtexpr.kind
            && MatchSource::ForLoopDesugar == match_source
        {
            let mut visitor = VectorAccessVisitor {
                has_vector_access: false,
                index_id: expr.hir_id,
            };
            for arm in some_none_arms {
                if let PatKind::Struct(qpath, pats, _) = &arm.pat.kind
                    && let QPath::LangItem(item_type, _) = qpath
                    && LangItem::OptionSome == *item_type
                    && pats.last().is_some()
                {
                    if let PatKind::Binding(_, hir_id, _ident, _) = pats.last().unwrap().pat.kind {
                        visitor.index_id = hir_id;
                        walk_expr(&mut visitor, arm.body);
                    }
                }
            }

            if visitor.has_vector_access {
                self.span_constant.push(expr.span);
            }
        }
        walk_expr(self, expr);
    }
}
impl<'tcx> LateLintPass<'tcx> for IteratorOverIndexing {
    fn check_fn(
        &mut self,
        cx: &rustc_lint::LateContext<'tcx>,
        kind: rustc_hir::intravisit::FnKind<'tcx>,
        _: &'tcx rustc_hir::FnDecl<'tcx>,
        body: &'tcx rustc_hir::Body<'tcx>,
        _: Span,
        _: LocalDefId,
    ) {
        if let FnKind::Method(_ident, _sig) = kind {
            let mut visitor = ForLoopVisitor {
                span_constant: vec![],
            };
            walk_expr(&mut visitor, body.value);

            for span in visitor.span_constant {
                clippy_wrappers::span_lint_and_help(
                    cx,
                    ITERATOR_OVER_INDEXING,
                    span,
                    LINT_MESSAGE,
                    None,
                    "Instead, use an iterator or index to `.len()`.",
                );
            }
        }
    }
}
