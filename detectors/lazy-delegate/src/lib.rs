#![feature(rustc_private)]
#![warn(unused_extern_crates)]
#![feature(let_chains)]
extern crate rustc_ast;
extern crate rustc_span;

use clippy_utils::diagnostics::span_lint_and_help;
use if_chain::if_chain;
use rustc_ast::ast::GenericArgs;
use rustc_ast::{
    tokenstream::{TokenStream, TokenTree},
    AngleBracketedArgs, AttrArgs, AttrKind, Item, ItemKind, TyKind,
};
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_span::Span;

/*
dylint_linting::impl_late_lint! {
    /// ### What it does
    /// Checks for delegated calls to contracts passed as arguments.
    /// ### Why is this bad?
    ///Delegated calls to contracts passed as arguments can be used to change the expected behavior of the contract. If you need to change the target of a delegated call, you should use a storage variable, and make a function with proper access control to change it.
    /// ### Known problems
    /// Remove if none.
    ///
    /// ### Example
    /// ```rust
    ///pub fn delegateCall(&mut self, target: Hash, argument: Balance) {
    ///    let selector_bytes = [0x0, 0x0, 0x0, 0x0];
    ///    let result: T  = build_call::<DefaultEnvironment>()
    ///        .delegate(target)
    ///        .exec_input(
    ///            ExecutionInput::new(Selector::new(selector_bytes))
    ///                .push_arg(argument)
    ///     )
    ///        .returns::<T>()
    ///     .invoke();
    ///}
    ///     ```
    /// Use instead:
    ///```rust
    ///pub fn delegate_call(&mut self, argument: Balance) {
    ///    let selector_bytes = [0x0, 0x0, 0x0, 0x0];
    ///    let result: T  = build_call::<DefaultEnvironment>()
    ///        .delegate(self.target)
    ///        .exec_input(
    ///            ExecutionInput::new(Selector::new(selector_bytes))
    ///                .push_arg(argument)
    ///        )
    ///        .returns::<T>()
    ///        .invoke();
    ///}
    ///
    ///pub fn set_target(&mut self, new_target: Hash) -> Result<(), &'static str> {
    ///   if self.admin != self.env().caller() {
    ///        Err("Only admin can set target")
    ///    } else {
    ///        self.target = new_target;
    ///        Ok(())
     ///   }
    ///}

    pub DELEGATE_CALL,
    Warn,
    "Passing arguments to the target of a delegate call is not safe, as it allows the caller to set a malicious hash as the target.",
    DelegateCall::default()
}
 */
dylint_linting::impl_pre_expansion_lint! {
    /// ### What it does
    /// Checks for non-lazy storage when using delegate calls.
    /// ### Why is this bad?
    /// ink! has a bug that makes delegated calls not modify the storage of the caller.
    /// ### Example
    /// ```rust
    ///    #[ink(storage)]
    ///    pub struct Contract {
    ///        admin: AccountId,
    ///    }
    ///     ```
    /// Use instead:
    ///```rust
    ///    #[ink(storage)]
    ///    pub struct DelegateCall {
    ///        admin: Lazy<AccountId, ManualKey<12345>>,
    ///    }
    ///
    /// ### More info
    /// - https://github.com/paritytech/ink/issues/1825
    /// - https://github.com/paritytech/ink/issues/1826
    ///```

    pub DELEGATE_CALL,
    Warn,
    "Use of delegate call with non-lazy, non-mapping storage won't modify the storage of the contract.",
    DelegateCall::default()
}

#[derive(Default)]
pub struct DelegateCall {
    non_lazy_manual_storage_spans: Vec<Span>,
    delegate_uses: Vec<Span>,
}

impl EarlyLintPass for DelegateCall {
    fn check_item(&mut self, _: &EarlyContext<'_>, item: &Item) {
        if is_storage_item(item)
        && let ItemKind::Struct(strt, _) = &item.kind
        {
            for field in strt.fields() {
                if let Some(_) = field.ident
                && let TyKind::Path(_, path) = &field.ty.kind
                && path.segments.len() == 1
                && path.segments[0].ident.name.to_string() == "Lazy".to_string()
                && let Some(arg) = &path.segments[0].args
                && let GenericArgs::AngleBracketed(AngleBracketedArgs { args, .. }) = arg.clone().into_inner()
                && args.len() > 1 {} else {
                    if !self.non_lazy_manual_storage_spans.contains(&item.span) {
                        self.non_lazy_manual_storage_spans.push(item.span);
                    }
                }
            }
        }
    }

    fn check_ident(&mut self, cx: &EarlyContext<'_>, id: rustc_span::symbol::Ident) {
        if id.name.to_string() == "delegate" {
            dbg!(id.span);
            self.delegate_uses.push(id.span);
        }

        if !self.delegate_uses.is_empty() {
            span_lint_and_help(
                cx,
                DELEGATE_CALL,
                id.span,
                "Delegate call with non-lazy, non-mapping storage",
                None,
                "Use lazy storage with manual keys",
            );

            for span in &self.non_lazy_manual_storage_spans {
                span_lint_and_help(
                    cx,
                    DELEGATE_CALL,
                    *span,
                    "Non-lazy non-mapping storage",
                    None,
                    "Use lazy storage with manual keys. \nMore info in https://github.com/paritytech/ink/issues/1826 and https://github.com/paritytech/ink/issues/1825",
                );
            }

            self.delegate_uses.clear();
        }
    }
}

fn is_storage_item(item: &Item) -> bool {
    item.attrs.iter().any(|attr| {
        if_chain!(
            if let AttrKind::Normal(normal) = &attr.kind;
            if let AttrArgs::Delimited(delim_args) = &normal.item.args;
            if is_storage_present(&delim_args.tokens);
            then {
                return true
            }
        );
        return false;
    })
}

fn is_storage_present(token_stream: &TokenStream) -> bool {
    token_stream.trees().any(|tree| match tree {
        TokenTree::Token(token, _) => {
            if let Some(ident) = token.ident() {
                return ident.0.name.to_ident_string().contains("storage");
            } else {
                false
            }
        }
        TokenTree::Delimited(_, _, token_stream) => is_storage_present(token_stream),
    })
}
