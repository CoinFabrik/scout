#![feature(rustc_private)]
extern crate rustc_ast;
extern crate rustc_span;

use clippy_utils::diagnostics::span_lint_and_help;
use if_chain::if_chain;
use rustc_ast::visit::{walk_expr, FnKind, Visitor};
use rustc_ast::{Expr, ExprKind};
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_span::Span;
use scout_audit_internal::Detector;

dylint_linting::declare_early_lint! {
    /// ### What it does
    /// It warns if for loop uses indexing instead of iterator.
    /// If the indexing goes to `.len()` it will not warn.
    ///
    /// ### Why is this bad?
    /// Accessing a vector by index is slower than using an iterator.
    /// Also, if the index is out of bounds, it will panic.
    ///
    ///
    /// ### Example
    /// ```rust
    ///     #[ink(message)]
    ///     pub fn bad_indexing(&self){
    ///         for i in 0..3 {
    ///             foo(self.value[i]);
    ///         }
    ///     }
    /// ```
    /// Use instead:
    /// ```rust
    ///    #[ink(message)]
    ///    pub fn iterator(&self) {
    ///        for item in self.value.iter() {
    ///             foo(self.value[i]);
    ///        }
    ///    }
    ///
    /// // or if its not iterable (with `in`, `iter` or `to_iter()`)
    ///
    ///    #[ink(message)]
    ///    pub fn index_to_len(&self){
    ///        for i in 0..self.value.len() {
    ///             foo(self.value[i]);
    ///        }
    /// ```

    pub ITERATOR_OVER_INDEXING,
    Warn,
    Detector::IteratorsOverIndexing.get_lint_message()
}

struct ForLoopVisitor {
    span: Vec<Span>,
}

impl<'ast> Visitor<'ast> for ForLoopVisitor {
    fn visit_expr(&mut self, ex: &'ast Expr) {
        if_chain! {
            if let ExprKind::ForLoop(_, expr, _, _) = &ex.kind;
            if let ExprKind::Range(from, to, _) = &expr.kind;
            if let Some(from) = from;
            if let Some(to) = to;
            if let ExprKind::Lit(from_lit) = &from.kind;
            if let ExprKind::Lit(to_lit) = &to.kind;
            if let nr_from = from_lit.symbol.as_str().parse::<i64>().unwrap();
            if nr_from >= 0;
            if let nr_to = to_lit.symbol.as_str().parse::<i64>().unwrap();
            if nr_to >= nr_from;
            then {
                self.span.push(ex.span);
            }
        }
        walk_expr(self, ex);
    }
}

impl EarlyLintPass for IteratorOverIndexing {
    fn check_fn(
        &mut self,
        cx: &EarlyContext<'_>,
        fn_kind: FnKind<'_>,
        _: Span,
        _: rustc_ast::NodeId,
    ) {
        let mut visitor = ForLoopVisitor { span: vec![] };

        let block = match fn_kind {
            FnKind::Fn(_, _, _, _, _, body) => body,
            _ => return,
        };

        block.into_iter().for_each(|item| {
            for statement in &item.stmts {
                match &statement.kind {
                    rustc_ast::StmtKind::Expr(expr) | rustc_ast::StmtKind::Semi(expr) => {
                        visitor.visit_expr(expr);
                    }
                    _ => {}
                }
            }
        });

        for sp in visitor.span.iter() {
            span_lint_and_help(
                cx,
                ITERATOR_OVER_INDEXING,
                *sp,
                Detector::IteratorsOverIndexing.get_lint_message(),
                None,
                "Instead, use an iterator or index to `.len()`.",
            );
        }
    }
}
