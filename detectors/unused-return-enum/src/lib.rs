#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;
extern crate rustc_span;

use clippy_utils::diagnostics::span_lint_and_help;
use if_chain::if_chain;
use rustc_hir::intravisit::walk_expr;
use rustc_hir::intravisit::FnKind;
use rustc_hir::intravisit::Visitor;
use rustc_hir::FnRetTy;
use rustc_hir::QPath;
use rustc_hir::{Body, FnDecl, HirId, TyKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Span;

dylint_linting::declare_late_lint! {
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
    ///         let sum = value1 + value2;
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
    ///         let sum = value1 + value2;
    ///         let percentage_difference =
    ///         match 100u128.checked_mul(absolute_difference / sum) {
    ///            Some(result) => Ok(result),
    ///            None => panic!("overflow!"),
    ///         };
    ///         return Err(TradingPairErrors::Overflow);
    ///     }
    /// ```
    pub UNUSED_RETURN_ENUM,
    Warn,
    "If any of the variants (Ok/Err) is not used, the code could be simplified or it could imply a bug"
}

struct CounterVisitor {
    count_err: u32,
    count_ok: u32,
}

impl<'tcx> Visitor<'tcx> for CounterVisitor {
    fn visit_expr(&mut self, expr: &'tcx rustc_hir::Expr<'tcx>) {
        if_chain! {
            if let rustc_hir::ExprKind::Call(fun, _) = &expr.kind;
            if let rustc_hir::ExprKind::Path(path_method) = &fun.kind;
            if let QPath::Resolved(None, path) = *path_method;
            if let Some(segment) = path.segments.last();
            then {
                if segment.ident.to_string() == "Err" {
                    self.count_err += 1
                }
                if segment.ident.to_string() == "Ok" {
                    self.count_ok += 1
                }
            }
        }
        walk_expr(self, expr);
    }
}

impl<'tcx> LateLintPass<'tcx> for UnusedReturnEnum {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        decl: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        _: Span,
        _: HirId,
    ) {
        if_chain! {
            // Filter functions with that return "Result"
            if let FnRetTy::Return(ty) = decl.output;
            if let TyKind::Path(method_path) = &ty.kind;
            if let QPath::Resolved(None, path) = *method_path;
            if path.segments.len() == 1 && path.segments[0].ident.name.as_str() == "Result";
            then {
                let mut visitor = CounterVisitor {
                    count_err: 0,
                    count_ok: 0,
                };
                visitor.visit_expr(body.value);
                if (visitor.count_err < 1 || visitor.count_ok < 1) && (visitor.count_err != visitor.count_ok) {
                    span_lint_and_help(
                        cx,
                        UNUSED_RETURN_ENUM,
                        body.value.span,
                        "Ink messages can return a Result enum with a custom error type. If any of the variants (Ok/Err) is not used, the code could be simplified or it could imply a bug.",
                        None,
                        &format!("You are returning '{}' Err but '{}' Ok", visitor.count_err, visitor.count_ok),
                    );
                }
            }
        }
    }
}
