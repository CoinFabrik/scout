#![feature(rustc_private)]
#![warn(unused_extern_crates)]
#![feature(let_chains)]
extern crate rustc_ast;
extern crate rustc_span;
use clippy_wrappers::span_lint_and_help;
use rustc_ast::{
    tokenstream::{TokenStream, TokenTree},
    visit::{walk_block, walk_expr, Visitor},
    AssocItem, AssocItemKind, AttrArgs, AttrKind, Attribute, DelimArgs, ExprKind, MethodCall,
};
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_span::Span;

const LINT_MESSAGE: &str =
    "Using `transferred_value` without #[ink(payable)] will always return 0.";

scout_audit_dylint_linting::impl_pre_expansion_lint! {
    pub NON_PAYABLE_TRANSFERRED_VALUE,
    Warn,
    LINT_MESSAGE,
    NonPayableTransferredValue::default(),
    {
        name: "Non-payable transferred_value",
        long_message: "",
        severity: "Enhancement",
        help: "https://coinfabrik.github.io/scout/docs/vulnerabilities/non-payable-transferred-value",
        vulnerability_class: "Best practices",
    }
}

#[derive(Default)]
struct TransferredValueSearcher {
    span: Vec<Span>,
}

#[derive(Default)]
pub struct NonPayableTransferredValue {}

impl<'tcx> Visitor<'tcx> for TransferredValueSearcher {
    fn visit_expr(&mut self, expr: &'tcx rustc_ast::Expr) {
        if let ExprKind::MethodCall(met) = &expr.kind
            && is_self_env_transferred_value(met.as_ref())
        {
            self.span.push(expr.span);
        }

        walk_expr(self, expr)
    }
}

impl EarlyLintPass for NonPayableTransferredValue {
    fn check_impl_item(&mut self, cx: &EarlyContext<'_>, item: &AssocItem) {
        if let AssocItemKind::Fn(it) = &item.kind
            && !attr_is_present(&item.attrs, "payable")
            && attr_is_present(&item.attrs, "message")
            && it.body.is_some()
        {
            let mut visitor = TransferredValueSearcher::default();

            walk_block(&mut visitor, it.body.as_ref().unwrap());

            visitor.span.iter().for_each(|span| {
                span_lint_and_help(
                    cx,
                    NON_PAYABLE_TRANSFERRED_VALUE,
                    *span,
                    LINT_MESSAGE,
                    None,
                    "Consider adding #[ink(payable)] to the function to allow it to receive a value.",
                )
            });
        }
    }
}

fn is_self_env_transferred_value(met: &MethodCall) -> bool {
    if met.seg.ident.name.to_string() == "transferred_value"
        && let ExprKind::MethodCall(maybe_env) = &met.receiver.kind
        && maybe_env.seg.ident.name.to_string() == "env"
        && let ExprKind::Path(None, path) = &maybe_env.receiver.kind
        && path.segments.len() == 1
        && path.segments[0].ident.name.to_string() == "self"
    {
        return true;
    }
    false
}

fn attr_is_present(attrs: &[Attribute], find: &str) -> bool {
    for attr in attrs {
        if let AttrKind::Normal(nattr) = &attr.kind
            && nattr.item.path.segments.len() == 1
            && nattr.item.path.segments[0].ident.name.to_string() == "ink"
            && let AttrArgs::Delimited(DelimArgs { tokens, .. }) = &nattr.item.args
            && is_token_present(tokens, find)
        {
            return true;
        }
    }
    false
}

fn is_token_present(token_stream: &TokenStream, find: &str) -> bool {
    token_stream.trees().any(|tree| match tree {
        TokenTree::Token(token, _) => token
            .ident()
            .map_or(false, |ident| ident.0.name.to_string() == find),
        TokenTree::Delimited(_, _, _, token_stream) => is_token_present(token_stream, find),
    })
}
