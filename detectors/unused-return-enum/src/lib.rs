#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_span;

use clippy_utils::diagnostics::span_lint_and_help;
use rustc_ast::visit::{self, FnKind, Visitor};
use rustc_ast::{Expr, ExprKind, FnRetTy};
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_span::Span;


dylint_linting::declare_early_lint! {
    /// ### What it does
    /// It warns if a fuction that returns a Result type does not return a Result enum variant (Ok/Err)
    ///
    /// ### Why is this bad?
    /// If any of the variants (Ok/Err) is not used, the code could be simplified or it could imply a bug.
    ///
    ///
    /// ### Example
    /// ```rust
    /// // example code where a warning is issued
    ///     #![cfg_attr(not(feature = "std"), no_std)]
    ///     pub enum TradingPairErrors {
    ///         Overflow,
    ///     }
    ///     (...)
    ///
    ///     #[ink(message)]
    ///     pub fn get_percentage_difference(&mut self, value1: Balance, value2: Balance) -> Result<Balance, TradingPairErrors>  {
    ///         let absolute_difference = value1.abs_diff(value2);
    ///         let sum = value1.checked_add(value2).ok_or(TradingPairErrors::Overflow)?;
    ///         let percentage_difference =
    ///         match 100u128.checked_mul(absolute_difference / sum) {
    ///            Some(result) => result,
    ///            None => Err(TradingPairErrors::Overflow),
    ///         }
    ///     }
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code that does not raise a warning
    ///     #![cfg_attr(not(feature = "std"), no_std)]
    ///     pub enum TradingPairErrors {
    ///         Overflow,
    ///     }
    ///     (...)
    ///
    ///     #[ink(message)]
    ///     pub fn get_percentage_difference(&mut self, value1: Balance, value2: Balance) -> Result<Balance, TradingPairErrors>  {
    ///         let absolute_difference = value1.abs_diff(value2);
    ///         let sum = value1.checked_add(value2).ok_or(TradingPairErrors::Overflow)?;
    ///         let coeff = absolute_difference.checked_div(sum).ok_or(TradingPairErrors::DivisionByZero)?;
    ///         let percentage_difference = 100_u128.checked_mul().ok_or(TradingPairErrors::Overflow)?;
    ///         Ok(percentage_difference)
    ///     }
    /// ```
    pub UNUSED_RETURN_ENUM,
    Warn,
    "If any of the variants (Ok/Err) is not used, the code could be simplified or it could imply a bug"
}

struct CounterVisitor {
    count_err: u32,
    count_ok: u32,
    found_try: bool,
    span: Vec<Option<Span>>,
}

impl<'ast> Visitor<'ast> for CounterVisitor {
    fn visit_expr(&mut self, ex: &'ast Expr) {
        match &ex.kind {
            ExprKind::Call(func, _args) => {
                if let ExprKind::Path(_, path) = &func.kind {
                    if let Some(segment) = path.segments.last() {
                        if segment.ident.to_string() == "Err" {
                            self.span.push(Some(ex.span));
                            self.count_err += 1
                        }
                        if segment.ident.to_string() == "Ok" {
                            self.span.push(Some(ex.span));
                            self.count_ok += 1
                        }
                    }
                }
            }
            ExprKind::Try(_) => {
                self.span.push(Some(ex.span));
                self.found_try = true;
            }
            _ => {}
        }

        visit::walk_expr(self, ex);
    }

    fn visit_local(&mut self, l: &'ast rustc_ast::Local) {
        if let Some(expr) = &l.kind.init() {
            if let ExprKind::Try(try_expr) = &expr.kind {
                self.span.push(Some(try_expr.span));
                self.found_try = true;
            }
        }
    }
}

impl EarlyLintPass for UnusedReturnEnum {
    fn check_fn(
        &mut self,
        cx: &EarlyContext<'_>,
        fn_kind: FnKind<'_>,
        _: Span,
        _: rustc_ast::NodeId,
    ) {
        let (fn_sig, block) = match fn_kind {
            FnKind::Fn(_, _, fn_sig, _, _, body) => (fn_sig, body),
            _ => return,
        };

        // If the return type of the function is not a "Result" enum, we don't want to lint it
        if let FnRetTy::Ty(t) = &fn_sig.decl.output {
            if let rustc_ast::TyKind::Path(_, path) = &t.kind {
                if let Some(segment) = path.segments.last() {
                    if segment.ident.to_string() != "Result" {
                        return;
                    }
                }
            }
        }

        let mut visitor = CounterVisitor {
            count_ok: 0,
            count_err: 0,
            found_try: false,
            span: Vec::new(),
        };

        block.into_iter().for_each(|item| {
            for statement in &item.stmts {
                match &statement.kind {
                    rustc_ast::StmtKind::Expr(expr) | rustc_ast::StmtKind::Semi(expr) => {
                        visitor.visit_expr(expr);
                    }
                    rustc_ast::StmtKind::Local(l) => {
                        visitor.visit_local(l);
                    }
                    rustc_ast::StmtKind::Item(_) => {}
                    rustc_ast::StmtKind::Empty => {}
                    rustc_ast::StmtKind::MacCall(_) => {}
                }
            }
        });

        if !visitor.found_try
            && (visitor.count_err < 1 || visitor.count_ok < 1)
            && (visitor.count_err != visitor.count_ok)
        {
            visitor.span.iter().for_each(|span| {
                if let Some(span) = span {
                    span_lint_and_help(
                        cx,
                        UNUSED_RETURN_ENUM,
                        *span,
                        "unused return enum",
                        None,
                        "If any of the variants (Ok/Err) is not used, the code could be simplified or it could imply a bug",
                    );
                }
            });
        }
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(
        env!("CARGO_PKG_NAME"),
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("ui"),
    );
}
