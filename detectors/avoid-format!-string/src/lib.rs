#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_ast;
extern crate rustc_span;

use clippy_utils::{diagnostics::span_lint_and_help, sym};
use if_chain::if_chain;
use rustc_ast::{Expr, ExprKind, Item, NodeId};
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_span::sym;

dylint_linting::impl_pre_expansion_lint! {
    /// ### What it does
    /// Detects the usage of `format!` macro.
    ///
    /// ### Why is this bad?
    /// The usage of format! is not recommended because it can panic the execution.
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
    "`format!` can panic in runtime, and this should be avoided in production code",
    AvoidFormatString::default()
}

#[derive(Default)]
pub struct AvoidFormatString {
    stack: Vec<NodeId>,
}

impl EarlyLintPass for AvoidFormatString {
    fn check_item(&mut self, _cx: &EarlyContext, item: &Item) {
        if self.in_test_item() || is_test_item(item) {
            self.stack.push(item.id);
        }
    }

    fn check_expr(&mut self, cx: &EarlyContext, expr: &Expr) {
        if_chain! {
            if !self.in_test_item();
            if let ExprKind::MacCall(mac) = &expr.kind;
            if mac.path == sym!(format);

            then {
                span_lint_and_help(
                    cx,
                    AVOID_FORMAT_STRING,
                    expr.span,
                    "The format! macro should not be used, it can panic at runtime.",
                    None,
                    "Instead, if this is returning an error, define a new error type",
                );
            }
        }
    }
}

fn is_test_item(item: &Item) -> bool {
    item.attrs.iter().any(|attr| {
        if attr.has_name(sym::test) {
            true
        } else {
            if_chain! {
                if attr.has_name(sym::cfg);
                if let Some(items) = attr.meta_item_list();
                if let [item] = items.as_slice();
                if let Some(feature_item) = item.meta_item();
                if feature_item.has_name(sym::test);
                then {
                    true
                } else {
                    false
                }
            }
        }
    })
}

impl AvoidFormatString {
    fn in_test_item(&self) -> bool {
        !self.stack.is_empty()
    }
}
