#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_ast;
extern crate rustc_span;

use clippy_utils::sym;
use if_chain::if_chain;
use rustc_ast::{
    ptr::P,
    token::{LitKind, TokenKind},
    tokenstream::{TokenStream, TokenTree},
    AttrArgs, AttrKind, Expr, ExprKind, Item, MacCall, StmtKind,
};
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_span::{sym, Span};
use scout_audit_internal::Detector;

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
    Detector::PanicError.get_lint_message(),
    PanicError::default()
}

#[derive(Default)]
pub struct PanicError {
    in_test_span: Option<Span>,
}

impl EarlyLintPass for PanicError {
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

    fn check_stmt(&mut self, cx: &EarlyContext<'_>, stmt: &rustc_ast::Stmt) {
        if_chain! {
            if !self.in_test_item();
            if let StmtKind::MacCall(mac) = &stmt.kind;
            then {
                check_macro_call(cx, stmt.span, &mac.mac)
            }
        }
    }

    fn check_expr(&mut self, cx: &EarlyContext, expr: &Expr) {
        if_chain! {
            if !self.in_test_item();
            if let ExprKind::MacCall(mac) = &expr.kind;
            then {
                check_macro_call(cx, expr.span, mac)
            }
        }
    }
}

fn check_macro_call(cx: &EarlyContext, span: Span, mac: &P<MacCall>) {
    if_chain! {
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
            Detector::PanicError.span_lint_and_help(
                cx,
                PANIC_ERROR,
                span,
                &format!("You could use instead an Error enum and then 'return Err(Error::{})'", capitalize_err_msg(lit.symbol.as_str()).replace(' ', "")),
            );
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

impl PanicError {
    fn in_test_item(&self) -> bool {
        self.in_test_span.is_some()
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

fn is_test_token_present(token_stream: &TokenStream) -> bool {
    token_stream.trees().any(|tree| match tree {
        TokenTree::Token(token, _) => token.is_ident_named(sym::test),
        TokenTree::Delimited(_, _, token_stream) => is_test_token_present(token_stream),
    })
}
