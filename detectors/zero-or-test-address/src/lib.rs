#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_data_structures;
extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;

use clippy_utils::diagnostics::span_lint_and_help;
use if_chain::if_chain;
use rustc_data_structures::steal::Steal;
use rustc_hir::intravisit::{walk_expr, FnKind};
use rustc_hir::intravisit::{Visitor};
use rustc_hir::Expr;
use rustc_hir::{Body, FnDecl};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::thir::Thir;
use rustc_span::def_id::LocalDefId;
use rustc_span::Span;
use rustc_middle::thir::Expr as TypedExpr;
use rustc_hir::ExprKind as HirExprKind;

// use rustc_middle::query::plumbing::sealed::IntoQueryParam

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// ### Why is this bad?
    ///
    /// ### Known problems
    /// Remove if none.
    ///
    /// ### Example
    /// ```rust
    /// // example code where a warning is issued
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code that does not raise a warning
    /// ```
    pub ZERO_OR_TEST_ADDRESS,
    Warn,
    "description goes here"
}

impl<'tcx> LateLintPass<'tcx> for ZeroOrTestAddress {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        _: &'tcx FnDecl<'_>,
        body: &'tcx Body<'_>,
        _: Span,
        id: LocalDefId,
    ) {
        struct ZeroCheckStorage<'tcx> {
            span: Option<Span>,
            passes_account_id_as_arg: bool,
            ckeck_zero_addr_inside: bool,
            the_fn_id: LocalDefId,
            the_body: &'tcx Body<'tcx>,
            thir: &'tcx Steal<Thir<'tcx>>,
            seen: Vec<LocalDefId>,
        }



        fn uses_account_id_as_param(
            expr: &Expr,
            _body: &Body<'_>,
            id: LocalDefId,
            thir: &Steal<Thir<'_>>,
            seen: &mut Vec<LocalDefId>,
        ) -> Option<Span> {


            if let HirExprKind::Call(_, _) = &expr.kind {
                let thir_guard = thir.borrow();
                let thir = &*thir_guard;
                if thir.params.is_empty() {
                    return None;
                }
                for param in &thir.params {
                    let ty = param.ty.to_string();
                    if ty == "ink::ink_primitives::AccountId" {
                        if !seen.contains(&id) {
                            dbg!(thir);
                        }
                        seen.push(id);
                        return Some(expr.span);
                    }
                }
                return None;
            }
            return None;

        }

        fn _check_zero_addr_in(
            expr: &Expr,
            _body: &Body<'_>,
            _id: LocalDefId,
            _thir: &Steal<Thir<'_>>,
        ) -> bool {

            if_chain! {
                if let HirExprKind::If(cond, _, _) = &expr.kind;
                if let condition_kind = &cond.kind;
                if let HirExprKind::Binary(op, _, _) = condition_kind;
                if let op_kind = op.node;
                if op_kind == rustc_hir::BinOpKind::Ne;
                then {
                    dbg!(condition_kind);
                    return true;
                }
            }
            return false;
        }


        impl<'tcx> Visitor<'tcx> for ZeroCheckStorage<'tcx> {
/*             fn visit_stmt(&mut self, stmt: &'tcx Stmt<'tcx>) {
                // check for an statement that modifies the state
                // the state is modified if the statement is an assignment and modifies an struct
                // or if if invokes a function and the receiver is a env::balance
                if self.passes_account_id_as_arg && self.ckeck_zero_addr_inside {
                } else {
                    walk_stmt(self, stmt);
                }
            }
             */
            fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
                let function_takes_acc_id_span =
                    uses_account_id_as_param(expr, self.the_body, self.the_fn_id, self.thir, &mut self.seen);
                if let Some(span) = function_takes_acc_id_span {
                    self.passes_account_id_as_arg = true;
                    self.span = Some(span);
                }
/*                 let ckeck_zero_addr =
                    check_zero_addr_in(expr, self.the_body, self.the_fn_id, self.thir);

                if ckeck_zero_addr {
                    self.ckeck_zero_addr_inside = true;
                } */


                walk_expr(self, expr);
            }
        }

        let mut zerocheck_storage = ZeroCheckStorage {
            span: None,
            passes_account_id_as_arg: false,
            ckeck_zero_addr_inside: true,
            the_fn_id: id,
            the_body: body,
            thir: cx.tcx.thir_body(id).unwrap().0,
            seen: Vec::new(),
        };
        walk_expr(&mut zerocheck_storage, body.value);

        if zerocheck_storage.passes_account_id_as_arg
            && !zerocheck_storage.ckeck_zero_addr_inside
        {
            span_lint_and_help(
                cx,
                ZERO_OR_TEST_ADDRESS,
                // body.value.span,
                zerocheck_storage.span.unwrap(),
                "Not checking for a zero-address could lead to a locked contract",
                None,
                "This function should check if the AccountId passed is zero and revert if it is",
            );
        }
    }
}

#[test]
fn ui() {
    dylint_testing::ui_test(
        env!("CARGO_PKG_NAME"),
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("ui"),
    );
}

/* pub fn modify_admin(admin: String) -> String {
    if admin == "" {
        return "ZeroAddress".to_string();
    }

    admin
}
 */
