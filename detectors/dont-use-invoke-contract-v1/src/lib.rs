#![feature(rustc_private)]
#![feature(let_chains)]
extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_span;

use rustc_hir::{
    intravisit::{walk_expr, Visitor},
    Expr, ExprKind,
};
use rustc_lint::LateLintPass;
use rustc_span::{Span, Symbol};

const LINT_MESSAGE: &str =
    "This is a low level way to evaluate another smart contract. Avoid using it. But if needed, use `invoke_contract`.";

dylint_linting::declare_late_lint! {
    pub DONT_USE_INVOKE_CONTRACT_V1,
    Warn,
    LINT_MESSAGE,
    {
        name: "Dont use invoke_contract_v1",
        long_message: LINT_MESSAGE,
        severity: "Enhancement",
        help: "https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/dont-use-invoke-contract-v1",
        vulnerability_class: "Best practices",
    }
}

impl<'tcx> LateLintPass<'tcx> for DontUseInvokeContractV1 {
    fn check_fn(
        &mut self,
        cx: &rustc_lint::LateContext<'tcx>,
        _: rustc_hir::intravisit::FnKind<'tcx>,
        _: &'tcx rustc_hir::FnDecl<'tcx>,
        body: &'tcx rustc_hir::Body<'tcx>,
        _: rustc_span::Span,
        _: rustc_hir::def_id::LocalDefId,
    ) {
        struct DontUseInvokeContractV1Visitor {
            has_invoke_contract_v1_span: Vec<Option<Span>>,
        }

        impl<'tcx> Visitor<'tcx> for DontUseInvokeContractV1Visitor {
            fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
                if let ExprKind::MethodCall(path_segment, _, _, _) = &expr.kind {
                    if path_segment.ident.name == Symbol::intern("invoke_contract_v1") {
                        self.has_invoke_contract_v1_span.push(Some(expr.span));
                    }
                }
                walk_expr(self, expr);
            }
        }

        let mut visitor = DontUseInvokeContractV1Visitor {
            has_invoke_contract_v1_span: Vec::new(),
        };

        walk_expr(&mut visitor, body.value);

        visitor.has_invoke_contract_v1_span.iter().for_each(|span| {
            if let Some(span) = span {
                scout_audit_clippy_utils::diagnostics::span_lint_and_help(
                    cx,
                    DONT_USE_INVOKE_CONTRACT_V1,
                    *span,
                    LINT_MESSAGE,
                    None,
                    "Prefer to use the ink! guided and type safe approach to evaluate smart contracts than using this.",
                );
            }
        });
    }
}
