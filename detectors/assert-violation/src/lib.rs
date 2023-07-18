#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_span;

use clippy_utils::diagnostics::{span_lint, span_lint_and_help};
use if_chain::if_chain;
use rustc_ast::ast::LitKind;
use rustc_hir::intravisit::Visitor;
use rustc_hir::intravisit::{walk_expr, FnKind};
use rustc_hir::{Body, FnDecl, HirId};
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Span;

dylint_linting::declare_late_lint! {
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

    pub ASSERT_VIOLATION,
    Warn,
    "warning"
}
impl<'tcx> LateLintPass<'tcx> for AssertViolation {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        _: &'tcx FnDecl<'_>,
        body: &'tcx Body<'_>,
        _: Span,
        _: HirId,
    ) {
        struct AssertViolationStorage {
            span: Vec<Option<Span>>,
            uses_assert: bool,
        }

        impl<'tcx> Visitor<'tcx> for AssertViolationStorage {
            fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
                if_chain! {
                    if let ExprKind::If(_, b, _) = expr.kind;
                    if let ExprKind::Block(block, _) = b.kind;
                    if let Some(expr1) = block.expr;
                    if let ExprKind::Call(_, args) = expr1.kind;
                    if let ExprKind::Call(_, args2) = args[0].kind;
//                    if args2.len() == 2;
//                    if let ExprKind::AddrOf(_, _, expr2) = args2[0].kind;
//                    if let ExprKind::Array(args3) = expr2.kind;
//                    if let ExprKind::Lit(lit) = &args3[0].kind;
//                    if let LitKind::Str(_, _) = lit.node;
                    then {
                        dbg!(expr.span);
                        self.span.push(Some(b.span));
                        self.uses_assert = true;
                    }
                }

                walk_expr(self, expr);
            }
        }
        let mut av_storage = AssertViolationStorage {
            span: Vec::new(),
            uses_assert: false,
        };

        walk_expr(&mut av_storage, body.value);

        if av_storage.uses_assert {
            av_storage.span.iter().for_each(|span| {
                if let Some(span) = span {

                    dbg!(span);

                    span_lint_and_help(cx, ASSERT_VIOLATION, *span, "warn", None, "help");
                }
            });
        }
    }
}
