#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_ast;
extern crate rustc_span;

use clippy_utils::sym;
use if_chain::if_chain;
use rustc_ast::{
    tokenstream::{TokenStream, TokenTree},
    AttrArgs, AttrKind, Expr, ExprKind, Item,
};
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_span::{sym, Span};
use scout_audit_internal::Detector;

dylint_linting::impl_pre_expansion_lint! {
    /// ### What it does
    /// Detects the usage of `format!` macro.
    ///
    /// ### Why is this bad?
    /// The usage of format! is not recommended.
    /// ### Example
    /// ```rust
    ///    #[ink(message)]
    ///    pub fn crash(&self) -> Result<(), Error> {
    ///        Err(Error::FormatError {
    ///            msg: (format!("{}", self.value)),
    ///        })
    ///    }
    ///
    /// ```
    /// Use instead:
    /// ```rust
    ///    pub enum Error {
    ///        FormatError { msg: String },
    ///        CrashError
    ///    }
    ///
    ///    #[ink(message)]
    ///    pub fn crash(&self) -> Result<(), Error> {
    ///        Err(Error::FormatError { msg: self.value.to_string() })
    ///    }
    ///
    /// ```
    pub AVOID_FORMAT_STRING,
    Warn,
    Detector::AvoidFormatString.get_lint_message(),
    AvoidFormatString::default()
}

#[derive(Default)]
pub struct AvoidFormatString {
    in_test_span: Option<Span>,
}

impl AvoidFormatString {
    fn in_test_item(&self) -> bool {
        self.in_test_span.is_some()
    }
}

impl EarlyLintPass for AvoidFormatString {
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

    fn check_expr(&mut self, cx: &EarlyContext, expr: &Expr) {
        if_chain! {
            if !self.in_test_item();
            if let ExprKind::MacCall(mac) = &expr.kind;
            if mac.path == sym!(format);

            then {
                Detector::AvoidFormatString.span_lint_and_help(
                    cx,
                    AVOID_FORMAT_STRING,
                    expr.span,
                    "Instead, if this is returning an error, define a new error type",
                );
            }
        }
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
