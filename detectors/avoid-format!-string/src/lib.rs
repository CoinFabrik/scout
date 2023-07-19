#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_ast;
extern crate rustc_span;

use clippy_utils::{diagnostics::span_lint_and_help, sym};
use if_chain::if_chain;
use rustc_ast::{
    Expr, ExprKind, Item, NodeId,
};
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_span::sym;

dylint_linting::impl_pre_expansion_lint! {
    /// ### What it does
    /// The panic! macro is used to stop execution when a condition is not met.
    /// This is useful for testing and prototyping, but should be avoided in production code
    ///
    /// ### Why is this bad?
    /// The usage of panic! is not recommended because it will stop the execution of the caller contract.
    ///
    /// ### Known problems
    /// While this linter detects explicit calls to panic!, there are some ways to raise a panic such as unwrap() or expect().
    ///
    /// ### Example
    /// ```rust
    /// pub fn add(&mut self, value: u32)   {
    ///    match self.value.checked_add(value) {
    ///        Some(v) => self.value = v,
    ///        None => panic!("Overflow error"),
    ///    };
    /// }
    /// ```
    /// Use instead:
    /// ```rust
    /// pub fn add(&mut self, value: u32) -> Result<(), Error>  {
    ///     match self.value.checked_add(value) {
    ///         Some(v) => self.value = v,
    ///         None => return Err(Error::OverflowError),
    ///     };
    ///     Ok(())
    /// }
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
                    &format!("Instead, if this is returning an error, define a new error type"),
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
