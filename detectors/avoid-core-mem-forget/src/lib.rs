#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_span;

use if_chain::if_chain;
use rustc_ast::{Expr, ExprKind, Item, NodeId};
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_span::sym;
use scout_audit_internal::Detector;

dylint_linting::impl_pre_expansion_lint! {
    /// ### What it does
    /// Checks for `core::mem::forget` usage.
    /// ### Why is this bad?
    /// This is a bad practice because it can lead to memory leaks, resource leaks and logic errors.
    /// ### Example
    /// ```rust
    ///    #[ink(message)]
    ///    pub fn forget_value(&mut self) {
    ///        let forgotten_value = self.value;
    ///        self.value = false;
    ///        core::mem::forget(forgotten_value);
    ///    }
    ///
    ///     ```
    /// Use instead:
    ///```rust
    ///    #[ink(message)]
    ///    pub fn forget_value(&mut self) {
    ///        let forgotten_value = self.value;
    ///        self.value = false;
    ///        let _ = forgotten_value;
    ///    }
    ///
    /// // or use drop if droppable
    ///
    ///    pub fn drop_value(&mut self) {
    ///        let forgotten_value = self.value;
    ///        self.value = false;
    ///        forget_value.drop();
    ///    }
    ///```

    pub AVOID_STD_CORE_MEM_FORGET,
    Warn,
    Detector::AvoidCoreMemForget.get_lint_message(),
    AvoidStdCoreMemForget::default()
}

#[derive(Default)]
pub struct AvoidStdCoreMemForget {
    stack: Vec<NodeId>,
}

impl EarlyLintPass for AvoidStdCoreMemForget {
    fn check_item(&mut self, _cx: &EarlyContext, item: &Item) {
        if self.in_test_item() || is_test_item(item) {
            self.stack.push(item.id);
        }
    }

    fn check_expr(&mut self, cx: &EarlyContext, expr: &Expr) {
        if_chain! {
            if !self.in_test_item();
            if let ExprKind::Call(a, _) = &expr.kind;
            if let ExprKind::Path(_, path) = &a.kind;
            if path.segments.len() == 3;
            if path.segments[0].ident.name.to_string() == "core";
            if path.segments[1].ident.name.to_string() == "mem";
            if path.segments[2].ident.name.to_string() == "forget";
            then {

                Detector::AvoidCoreMemForget.span_lint_and_help(
                    cx,
                    AVOID_STD_CORE_MEM_FORGET,
                    expr.span,
                    "Instead, use the `let _ = ...` pattern or `.drop` method to forget the value.",
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

impl AvoidStdCoreMemForget {
    fn in_test_item(&self) -> bool {
        !self.stack.is_empty()
    }
}
