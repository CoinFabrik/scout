#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;
extern crate rustc_span;

use if_chain::if_chain;
use rustc_hir::def::Res;
use rustc_hir::def_id::LocalDefId;
use rustc_hir::intravisit::Visitor;
use rustc_hir::intravisit::{walk_expr, FnKind};
use rustc_hir::{Body, FnDecl};
use rustc_hir::{Expr, ExprKind, PatKind, QPath};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Span;
use scout_audit_internal::Detector;
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

    pub DELEGATE_CALL,
    Warn,
    Detector::DelegateCall.get_lint_message()
}
impl<'tcx> LateLintPass<'tcx> for DelegateCall {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        _: &'tcx FnDecl<'_>,
        body: &'tcx Body<'_>,
        _: Span,
        _: LocalDefId,
    ) {
        struct DelegateCallStorage<'tcx> {
            span: Option<Span>,
            has_vulnerable_delegate: bool,
            the_body: &'tcx Body<'tcx>,
        }

        fn check_delegate_call(expr: &Expr, body: &Body<'_>) -> Option<Span> {
            if_chain! {
                if let ExprKind::MethodCall(func, _, arguments, _) = &expr.kind;
                if let function_name = func.ident.name.to_string();
                if function_name == "delegate";
                then {
                    let mut param_hir_ids = Vec::new();
                    let mut arg_hir_ids = Vec::new();

                    for i in 0..body.params.len() {
                        if let PatKind::Binding(_, hir_id, _, _) = body.params[i].pat.kind {
                            param_hir_ids.push(hir_id);
                        }
                    }

                    for i in 0..arguments.len() {
                        arg_hir_ids.push(arguments[i].hir_id);

                        if let ExprKind::Path(qpath) = &arguments[i].kind {
                            match qpath {
                                QPath::Resolved(_, path) => {
                                    if let Res::Local(hir_id) = path.res {
                                        arg_hir_ids.push(hir_id);
                                    }
                                    for j in 0..path.segments.len() {
                                        arg_hir_ids.push(path.segments[j].hir_id);
                                    }
                                }
                                QPath::LangItem(_, _, Some(lang_item_hir_id)) => {
                                    arg_hir_ids.push(*lang_item_hir_id);
                                }
                                _ => (),
                            }
                        }
                    }

                    for param_id in param_hir_ids {
                        if arg_hir_ids.contains(&param_id) {
                            return Some(expr.span);
                        }
                    }

                    return None;
                }
            }
            None
        }

        impl<'tcx> Visitor<'tcx> for DelegateCallStorage<'_> {
            fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
                if let Some(delegate_call_span) = check_delegate_call(expr, self.the_body) {
                    self.has_vulnerable_delegate = true;
                    self.span = Some(delegate_call_span);
                };

                walk_expr(self, expr);
            }
        }

        let mut delegate_storage = DelegateCallStorage {
            span: None,
            has_vulnerable_delegate: false,
            the_body: body,
        };

        walk_expr(&mut delegate_storage, body.value);

        if delegate_storage.has_vulnerable_delegate {
            Detector::DelegateCall.span_lint_and_help(
                cx,
                DELEGATE_CALL,
                delegate_storage.span.unwrap(),
                "Consider using a memory value (self.target) as the target of the delegate call.",
            );
        }
    }
}
