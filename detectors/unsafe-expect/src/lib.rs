#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_span;

use rustc_hir::{
    intravisit::{walk_expr, Visitor},
    Expr, ExprKind,
};
use rustc_lint::LateLintPass;
use rustc_span::{Span, Symbol};

const LINT_MESSAGE: &str = "Unsafe usage of `expect`";

dylint_linting::declare_late_lint! {
    /// ### What it does
    /// Checks for usage of `expect`
    ///
    /// ### Why is this bad?
    /// `expect` might panic if the result value is an error or `None`.
    ///
    /// ### Example
    /// ```rust
    /// // example code where a warning is issued
    /// fn main() {
    ///    let result = result_fn().expect("error");
    /// }
    ///
    /// fn result_fn() -> Result<u8, Error> {
    ///     Err(Error::new(ErrorKind::Other, "error"))
    /// }
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code that does not raise a warning
    /// fn main() {
    ///    let result = if let Ok(result) = result_fn() {
    ///       result
    ///   }
    /// }
    ///
    /// fn result_fn() -> Result<u8, Error> {
    ///     Err(Error::new(ErrorKind::Other, "error"))
    /// }
    /// ```
    pub UNSAFE_EXPECT,
    Warn,
    LINT_MESSAGE,
    {
        name: "Unsafe Expect",
        long_message: "In Rust, the expect method is commonly used for error handling. It retrieves the value from a Result or Option and panics with a specified error message if an error occurs. However, using expect can lead to unexpected program crashes.    ",
        severity: "Medium",
        help: "https://coinfabrik.github.io/scout/docs/vulnerabilities/unsafe-expect",
        vulnerability_class: "Validations and error handling",
    }
}

impl<'tcx> LateLintPass<'tcx> for UnsafeExpect {
    fn check_fn(
        &mut self,
        cx: &rustc_lint::LateContext<'tcx>,
        _: rustc_hir::intravisit::FnKind<'tcx>,
        _: &'tcx rustc_hir::FnDecl<'tcx>,
        body: &'tcx rustc_hir::Body<'tcx>,
        _: rustc_span::Span,
        _: rustc_hir::def_id::LocalDefId,
    ) {
        struct UnsafeExpectVisitor {
            has_expect: bool,
            has_expect_span: Vec<Option<Span>>,
        }

        impl<'tcx> Visitor<'tcx> for UnsafeExpectVisitor {
            fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
                if let ExprKind::MethodCall(path_segment, _, _, _) = &expr.kind {
                    if path_segment.ident.name == Symbol::intern("expect") {
                        self.has_expect = true;
                        self.has_expect_span.push(Some(expr.span));
                    }
                }
                walk_expr(self, expr);
            }
        }

        let mut visitor = UnsafeExpectVisitor {
            has_expect: false,
            has_expect_span: Vec::new(),
        };

        walk_expr(&mut visitor, body.value);

        if visitor.has_expect {
            visitor.has_expect_span.iter().for_each(|span| {
                if let Some(span) = span {
                    scout_audit_clippy_utils::diagnostics::span_lint_and_help(
                        cx,
                        UNSAFE_EXPECT,
                        *span,
                        LINT_MESSAGE,
                        None,
                        "Please, use a custom error instead of `expect`",
                    );
                }
            });
        }
    }
}
