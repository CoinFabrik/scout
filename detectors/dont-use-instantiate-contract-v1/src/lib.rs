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
    "This is a low level way to instantiate another smart contract, calling the legacy `instantiate_v1` host function.";

scout_audit_dylint_linting::declare_late_lint! {
    pub DONT_USE_INSTANTIATE_CONTRACT_V1,
    Warn,
    LINT_MESSAGE,
    {
        name: "Dont use instantiate_contract_v1",
        long_message: LINT_MESSAGE,
        severity: "Enhancement",
        help: "https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/dont-use-instantiate-contract-v1",
        vulnerability_class: "Best practices",
    }
}

impl<'tcx> LateLintPass<'tcx> for DontUseInstantiateContractV1 {
    fn check_fn(
        &mut self,
        cx: &rustc_lint::LateContext<'tcx>,
        _: rustc_hir::intravisit::FnKind<'tcx>,
        _: &'tcx rustc_hir::FnDecl<'tcx>,
        body: &'tcx rustc_hir::Body<'tcx>,
        _: rustc_span::Span,
        _: rustc_hir::def_id::LocalDefId,
    ) {
        struct DontUseInstantiateContractV1Visitor {
            has_instantiate_contract_v1_span: Vec<Option<Span>>,
        }

        impl<'tcx> Visitor<'tcx> for DontUseInstantiateContractV1Visitor {
            fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
                if let ExprKind::MethodCall(path_segment, _, _, _) = &expr.kind {
                    if path_segment.ident.name == Symbol::intern("instantiate_contract_v1") {
                        self.has_instantiate_contract_v1_span.push(Some(expr.span));
                    }
                }
                walk_expr(self, expr);
            }
        }

        let mut visitor = DontUseInstantiateContractV1Visitor {
            has_instantiate_contract_v1_span: Vec::new(),
        };

        walk_expr(&mut visitor, body.value);

        visitor.has_instantiate_contract_v1_span.iter().for_each(|span| {
            if let Some(span) = span {
                clippy_utils::diagnostics::span_lint_and_help(
                    cx,
                    DONT_USE_INSTANTIATE_CONTRACT_V1,
                    *span,
                    LINT_MESSAGE,
                    None,
                    "Prefer to use methods on a `ContractRef` or the `CreateBuilder` through `build_create` instead",
                );
            }
        });
    }
}
