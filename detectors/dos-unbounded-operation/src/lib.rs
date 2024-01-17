#![feature(rustc_private)]
#![feature(let_chains)]
extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_span;

use rustc_ast::LitKind;
use rustc_hir::{
    intravisit::{walk_expr, FnKind, Visitor},
    ExprKind, LangItem, MatchSource, QPath,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::{def_id::LocalDefId, Span};
use scout_audit_internal::Detector;

dylint_linting::declare_late_lint! {
    /// ### What it does
    /// This detector checks that when using for or while loops, their conditions limit the execution to a constant number of iterations.
    /// ### Why is this bad?
    /// If the number of iterations is not limited to a specific range, it could potentially cause out of gas exceptions.
    /// ### Known problems
    /// False positives are to be expected when using variables that can only be set using controlled flows that limit the values within acceptable ranges.
    /// ### Example
    /// ```rust
    /// pub fn pay_out(&mut self) {
    ///     for i in 0..self.next_payee_ix {
    ///         let payee = self.payees.get(&i).unwrap();
    ///         self.env().transfer(payee.address, payee.value).unwrap();
    ///     }
    /// }
    /// ```
    /// Use instead:
    /// ```rust
    /// pub fn pay_out(&mut self, payee: u128) {
    ///     let payee = self.payees.get(&payee).unwrap();
    ///     self.env().transfer(payee.address, payee.value).unwrap();
    /// }
    /// ```
    pub DOS_UNBOUNDED_OPERATION,
    Warn,
    Detector::DosUnboundedOperation.get_lint_message()
}

struct ForLoopVisitor {
    span_constant: Vec<Span>,
}
impl<'tcx> Visitor<'tcx> for ForLoopVisitor {
    fn visit_expr(&mut self, expr: &'tcx rustc_hir::Expr<'tcx>) {
        if let ExprKind::Match(match_expr, _arms, source) = expr.kind
            && source == MatchSource::ForLoopDesugar
            && let ExprKind::Call(func, args) = match_expr.kind
            && let ExprKind::Path(qpath) = &func.kind
            && let QPath::LangItem(item, _span, _id) = qpath
            && item == &LangItem::IntoIterIntoIter
        {
            if args.first().is_some()
                && let ExprKind::Struct(qpath, fields, _) = args.first().unwrap().kind
                && let QPath::LangItem(langitem, _span, _id) = qpath
                && (LangItem::Range == *langitem
                    || LangItem::RangeInclusiveStruct == *langitem
                    || LangItem::RangeInclusiveNew == *langitem)
                && fields.last().is_some()
                && let ExprKind::Lit(lit) = &fields.last().unwrap().expr.kind
                && let LitKind::Int(_v, _typ) = lit.node
            {
                walk_expr(self, expr);
            } else {
                self.span_constant.push(expr.span);
            }
        }
        walk_expr(self, expr);
    }
}
impl<'tcx> LateLintPass<'tcx> for DosUnboundedOperation {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
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
                Detector::DosUnboundedOperation.span_lint_and_help(
                    cx,
                    DOS_UNBOUNDED_OPERATION,
                    span,
                    "This loop seems to do not have a fixed number of iterations",
                );
            }
        }
    }
}
