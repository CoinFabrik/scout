#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_span;

use clippy_utils::sym;
use if_chain::if_chain;
use rustc_ast::{
    ptr::P,
    tokenstream::{TokenStream, TokenTree},
    AttrArgs, AttrKind, Expr, ExprKind, Item, MacCall, Stmt, StmtKind,
};
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_span::{sym, Span};
use scout_audit_internal::Detector;

dylint_linting::impl_pre_expansion_lint! {
    /// ### What it does
    /// Checks for `assert!` usage.
    /// ### Why is this bad?
    /// `assert!` causes a panic, and panicking it's not a good practice. Instead, use proper error handling.
    /// ### Example
    /// ```rust
    ///    #[ink(message)]
    ///    pub fn assert_if_greater_than_10(&self, value: u128) -> bool {
    ///        assert!(value <= 10, "value should be less than 10");
    ///        true
    ///    }
    ///     ```
    /// Use instead:
    ///```rust
    ///     #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    ///     #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    ///     pub enum Error {
    ///         GreaterThan10,
    ///     }
    ///
    ///    #[ink(message)]
    ///    pub fn revert_if_greater_than_10(&self, value: u128) -> Result<bool, Error> {
    ///        if value <= 10 {
    ///            return Ok(true)
    ///        } else {
    ///            return Err(Error::GreaterThan10)
    ///        }
    ///    }
    ///```

    pub ASSERT_VIOLATION,
    Warn,
    Detector::AssertViolation.get_lint_message(),
    AssertViolation::default()
}

#[derive(Default)]
pub struct AssertViolation {
    in_test_span: Option<Span>,
}

impl AssertViolation {
    fn in_test_item(&self) -> bool {
        self.in_test_span.is_some()
    }
}

impl EarlyLintPass for AssertViolation {
    fn check_item(&mut self, _cx: &EarlyContext, item: &Item) {
        match (is_test_item(item), self.in_test_span) {
            (true, None) => self.in_test_span = Some(item.span),
            (true, Some(test_span)) => {
                if !test_span.contains(item.span) {
                    self.in_test_span = Some(item.span);
                }
            }
            (false, None) => {}
            (false, Some(test_span)) => {
                if !test_span.contains(item.span) {
                    self.in_test_span = None;
                }
            }
        };
    }

    fn check_stmt(&mut self, cx: &EarlyContext, stmt: &Stmt) {
        if self.in_test_item() {
            return;
        }

        if let StmtKind::MacCall(mac) = &stmt.kind {
            check_macro_call(cx, stmt.span, &mac.mac)
        }
    }
    fn check_expr(&mut self, cx: &EarlyContext, expr: &Expr) {
        if self.in_test_item() {
            return;
        }

        if let ExprKind::MacCall(mac) = &expr.kind {
            check_macro_call(cx, expr.span, mac)
        }
    }
}

fn check_macro_call(cx: &EarlyContext, span: Span, mac: &P<MacCall>) {
    if vec![
        sym!(assert),
        sym!(assert_eq),
        sym!(assert_ne),
        sym!(debug_assert),
        sym!(debug_assert_eq),
        sym!(debug_assert_ne),
    ]
    .iter()
    .any(|sym| &mac.path == sym)
    {
        Detector::AssertViolation.span_lint_and_help(
            cx,
            ASSERT_VIOLATION,
            span,
            "You could use instead an Error enum.",
        );
    }
}

fn is_test_item(item: &Item) -> bool {
    item.attrs.iter().any(|attr| {
        // Find #[cfg(all(test, feature = "e2e-tests"))]
        if_chain!(
            if let AttrKind::Normal(normal) = &attr.kind;
            if let AttrArgs::Delimited(delim_args) = &normal.item.args;
            if is_test_token_present(&delim_args.tokens);
            then {
                return true;
            }
        );

        // Find unit or integration tests
        if attr.has_name(sym::test) {
            return true;
        }

        if_chain! {
            if attr.has_name(sym::cfg);
            if let Some(items) = attr.meta_item_list();
            if let [item] = items.as_slice();
            if let Some(feature_item) = item.meta_item();
            if feature_item.has_name(sym::test);
            then {
                return true;
            }
        }

        false
    })
}

fn is_test_token_present(token_stream: &TokenStream) -> bool {
    token_stream.trees().any(|tree| match tree {
        TokenTree::Token(token, _) => token.is_ident_named(sym::test),
        TokenTree::Delimited(_, _, token_stream) => is_test_token_present(token_stream),
    })
}
