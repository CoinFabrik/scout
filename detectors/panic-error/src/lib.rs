#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_ast;
extern crate rustc_span;

use clippy_utils::{diagnostics::span_lint_and_help, sym};
use if_chain::if_chain;
use rustc_ast::{
    token::{LitKind, TokenKind},
    tokenstream::TokenTree,
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
    pub PANIC_ERROR,
    Warn,
    "`panic!` is useful for testing and prototyping, but should be avoided in production code",
    PanicError::default()
}

#[derive(Default)]
pub struct PanicError {
    stack: Vec<NodeId>,
}

impl EarlyLintPass for PanicError {
    fn check_item(&mut self, _cx: &EarlyContext, item: &Item) {
        if self.in_test_item() || is_test_item(item) {
            self.stack.push(item.id);
        }
    }

    fn check_expr(&mut self, cx: &EarlyContext, expr: &Expr) {
        if_chain! {
            if !self.in_test_item();
            if let ExprKind::MacCall(mac) = &expr.kind;
            if mac.path == sym!(panic);
            if let [TokenTree::Token(token, _)] = mac
                .args
                .tokens
                .clone()
                .into_trees()
                .collect::<Vec<_>>()
                .as_slice();
            if let TokenKind::Literal(lit) = token.kind;
            if lit.kind == LitKind::Str;
            then {
                span_lint_and_help(
                    cx,
                    PANIC_ERROR,
                    expr.span,
                    "The panic! macro is used to stop execution when a condition is not met. This is useful for testing and prototyping, but should be avoided in production code",
                    None,
                    &format!("You could use instead an Error enum and then 'return Err(Error::{})'", capitalize_err_msg(lit.symbol.as_str()).replace(' ', "")),
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

impl PanicError {
    fn in_test_item(&self) -> bool {
        !self.stack.is_empty()
    }
}

fn capitalize_err_msg(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
