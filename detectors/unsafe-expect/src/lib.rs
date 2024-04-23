#![feature(rustc_private)]
#![allow(clippy::enum_variant_names)]

extern crate rustc_hir;
extern crate rustc_span;

use std::{collections::HashSet, hash::Hash};

use if_chain::if_chain;
use rustc_hir::{
    def::Res,
    def_id::LocalDefId,
    intravisit::{walk_expr, FnKind, Visitor},
    BinOpKind, Body, Expr, ExprKind, FnDecl, HirId, Local, PathSegment, QPath, UnOp,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::{sym, Span, Symbol};
use scout_audit_clippy_utils::{diagnostics::span_lint_and_help, higher};

const LINT_MESSAGE: &str = "Unsafe usage of `expect`";
const PANIC_INDUCING_FUNCTIONS: [&str; 2] = ["panic", "bail"];

dylint_linting::declare_late_lint! {
    /// ### What it does
    /// Checks for usage of `expect`
    ///
    /// ### Why is this bad?
    /// `expect` might panic if the result value is an error or `None`.
    ///
    /// ### Example
    /// ```rust
    /// // example code where a warning is issued
    /// fn main() {
    ///    let result = result_fn().expect("error");
    /// }
    ///
    /// fn result_fn() -> Result<u8, Error> {
    ///     Err(Error::new(ErrorKind::Other, "error"))
    /// }
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code that does not raise a warning
    /// fn main() {
    ///    let result = if let Ok(result) = result_fn() {
    ///       result
    ///   }
    /// }
    ///
    /// fn result_fn() -> Result<u8, Error> {
    ///     Err(Error::new(ErrorKind::Other, "error"))
    /// }
    /// ```
    pub UNSAFE_EXPECT,
    Warn,
    LINT_MESSAGE,
    {
        name: "Unsafe Expect",
        long_message: "In Rust, the expect method is commonly used for error handling. It retrieves the value from a Result or Option and panics with a specified error message if an error occurs. However, using expect can lead to unexpected program crashes.    ",
        severity: "Medium",
        help: "https://coinfabrik.github.io/scout/docs/vulnerabilities/unsafe-expect",
        vulnerability_class: "Validations and error handling",
    }
}

/// Represents the type of check performed on method call expressions to determine their safety or behavior.
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum CheckType {
    IsSome,
    IsNone,
    IsOk,
    IsErr,
}

impl CheckType {
    fn from_method_name(name: Symbol) -> Option<Self> {
        match name.as_str() {
            "is_some" => Some(Self::IsSome),
            "is_none" => Some(Self::IsNone),
            "is_ok" => Some(Self::IsOk),
            "is_err" => Some(Self::IsErr),
            _ => None,
        }
    }

    fn inverse(self) -> Self {
        match self {
            Self::IsSome => Self::IsNone,
            Self::IsNone => Self::IsSome,
            Self::IsOk => Self::IsErr,
            Self::IsErr => Self::IsOk,
        }
    }

    /// Determines if the check type implies execution should halt, such as in error conditions.
    fn should_halt_execution(self) -> bool {
        matches!(self, Self::IsNone | Self::IsErr)
    }

    /// Determines if it is safe to expect the value without further checks, i.e., the value is present.
    fn is_safe_to_expect(self) -> bool {
        matches!(self, Self::IsSome | Self::IsOk)
    }
}

/// Represents a conditional checker that is used to analyze `if` or `if let` expressions.
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct ConditionalChecker {
    check_type: CheckType,
    checked_expr_hir_id: HirId,
}

impl ConditionalChecker {
    /// Handle te condition of the `if` or `if let` expression.
    fn handle_condition(condition: &Expr<'_>, inverse: bool) -> HashSet<Self> {
        if_chain! {
            if let ExprKind::MethodCall(path_segment, receiver, _, _) = condition.kind;
            if let Some(check_type) = CheckType::from_method_name(path_segment.ident.name);
            if let ExprKind::Path(QPath::Resolved(_, checked_expr_path)) = receiver.kind;
            if let Res::Local(checked_expr_hir_id) = checked_expr_path.res;
            then {
                let check_type = if inverse { check_type.inverse() } else { check_type };
                return std::iter::once(Self { check_type, checked_expr_hir_id }).collect();
            }
        }
        HashSet::new()
    }

    /// Constructs a ConditionalChecker from an expression if it matches a method call with a valid CheckType.
    fn from_expression(condition: &Expr<'_>) -> HashSet<Self> {
        match condition.kind {
            // Single `not` expressions are supported
            ExprKind::Unary(op, condition) => Self::handle_condition(condition, op == UnOp::Not),
            // Multiple `or` expressions are supported
            ExprKind::Binary(op, left_condition, right_condition) if op.node == BinOpKind::Or => {
                let mut result = Self::from_expression(left_condition);
                result.extend(Self::from_expression(right_condition));
                result
            }
            ExprKind::MethodCall(..) => Self::handle_condition(condition, false),
            _ => HashSet::new(),
        }
    }
}

/// Main unsafe-expect visitor
struct UnsafeExpectVisitor<'a, 'tcx> {
    cx: &'a LateContext<'tcx>,
    conditional_checker: HashSet<ConditionalChecker>,
    checked_exprs: HashSet<HirId>,
}

impl UnsafeExpectVisitor<'_, '_> {
    fn is_panic_inducing_call(&self, func: &Expr<'_>) -> bool {
        if let ExprKind::Path(QPath::Resolved(_, path)) = &func.kind {
            return PANIC_INDUCING_FUNCTIONS.iter().any(|&func| {
                path.segments
                    .iter()
                    .any(|segment| segment.ident.name.as_str().contains(func))
            });
        }
        false
    }

    fn get_expect_info(&self, receiver: &Expr<'_>) -> Option<HirId> {
        if_chain! {
            if let ExprKind::Path(QPath::Resolved(_, path)) = &receiver.kind;
            if let Res::Local(hir_id) = path.res;
            then {
                return Some(hir_id);
            }
        }
        None
    }

    fn set_conditional_checker(&mut self, conditional_checkers: &HashSet<ConditionalChecker>) {
        for checker in conditional_checkers {
            self.conditional_checker.insert(*checker);
            if checker.check_type.is_safe_to_expect() {
                self.checked_exprs.insert(checker.checked_expr_hir_id);
            }
        }
    }

    fn reset_conditional_checker(&mut self, conditional_checkers: HashSet<ConditionalChecker>) {
        for checker in conditional_checkers {
            if checker.check_type.is_safe_to_expect() {
                self.checked_exprs.remove(&checker.checked_expr_hir_id);
            }
            self.conditional_checker.remove(&checker);
        }
    }

    /// Process conditional expressions to determine if they should halt execution.
    fn handle_if_expressions(&mut self) {
        self.conditional_checker.iter().for_each(|checker| {
            if checker.check_type.should_halt_execution() {
                self.checked_exprs.insert(checker.checked_expr_hir_id);
            }
        });
    }

    fn is_literal_or_composed_of_literals(&self, expr: &Expr<'_>) -> bool {
        let mut stack = vec![expr];

        while let Some(current_expr) = stack.pop() {
            match current_expr.kind {
                ExprKind::Lit(_) => continue, // A literal is fine, continue processing.
                ExprKind::Tup(elements) | ExprKind::Array(elements) => {
                    stack.extend(elements);
                }
                ExprKind::Struct(_, fields, _) => {
                    for field in fields {
                        stack.push(field.expr);
                    }
                }
                ExprKind::Repeat(element, _) => {
                    stack.push(element);
                }
                _ => return false, // If any element is not a literal or a compound of literals, return false.
            }
        }

        true // If the stack is emptied without finding a non-literal, all elements are literals.
    }

    fn is_method_call_unsafe(
        &self,
        path_segment: &PathSegment,
        receiver: &Expr,
        args: &[Expr],
    ) -> bool {
        if path_segment.ident.name == sym::expect {
            return self
                .get_expect_info(receiver)
                .map_or(true, |id| !self.checked_exprs.contains(&id));
        }

        args.iter().any(|arg| self.contains_unsafe_method_call(arg))
            || self.contains_unsafe_method_call(receiver)
    }

    fn contains_unsafe_method_call(&self, expr: &Expr) -> bool {
        match &expr.kind {
            ExprKind::MethodCall(path_segment, receiver, args, _) => {
                self.is_method_call_unsafe(path_segment, receiver, args)
            }
            _ => false,
        }
    }
}

impl<'a, 'tcx> Visitor<'tcx> for UnsafeExpectVisitor<'a, 'tcx> {
    fn visit_local(&mut self, local: &'tcx Local<'tcx>) {
        if let Some(init) = local.init {
            match init.kind {
                ExprKind::MethodCall(path_segment, receiver, args, _) => {
                    if self.is_method_call_unsafe(path_segment, receiver, args) {
                        span_lint_and_help(
                            self.cx,
                            UNSAFE_EXPECT,
                            local.span,
                            LINT_MESSAGE,
                            None,
                            "Please, use a custom error instead of `expect`",
                        );
                    }
                }
                ExprKind::Call(func, args) => {
                    if let ExprKind::Path(QPath::Resolved(_, path)) = func.kind {
                        let is_some_or_ok = path
                            .segments
                            .iter()
                            .any(|segment| matches!(segment.ident.name, sym::Some | sym::Ok));
                        let all_literals = args
                            .iter()
                            .all(|arg| self.is_literal_or_composed_of_literals(arg));
                        if is_some_or_ok && all_literals {
                            self.checked_exprs.insert(local.pat.hir_id);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
        // If we are inside an `if` or `if let` expression, we analyze its body
        if !self.conditional_checker.is_empty() {
            match &expr.kind {
                ExprKind::Ret(..) => self.handle_if_expressions(),
                ExprKind::Call(func, _) if self.is_panic_inducing_call(func) => {
                    self.handle_if_expressions()
                }
                _ => {}
            }
        }

        // Find `if` or `if let` expressions
        if let Some(higher::IfOrIfLet {
            cond,
            then: if_expr,
            r#else: _,
        }) = higher::IfOrIfLet::hir(expr)
        {
            // If we are interested in the condition (if it is a CheckType) we traverse the body.
            let conditional_checker = ConditionalChecker::from_expression(cond);
            self.set_conditional_checker(&conditional_checker);
            walk_expr(self, if_expr);
            self.reset_conditional_checker(conditional_checker);
            return;
        }

        // If we find an unsafe `expect`, we raise a warning
        if_chain! {
            if let ExprKind::MethodCall(path_segment, receiver, _, _) = &expr.kind;
            if path_segment.ident.name == sym::expect;
            then {
                let receiver_hir_id = self.get_expect_info(receiver);
                // If the receiver is `None`, then we asume that the `expect` is unsafe
                let is_checked_safe = receiver_hir_id.map_or(false, |id| self.checked_exprs.contains(&id));
                if !is_checked_safe {
                    span_lint_and_help(
                        self.cx,
                        UNSAFE_EXPECT,
                        expr.span,
                        LINT_MESSAGE,
                        None,
                        "Please, use a custom error instead of `expect`",
                    );
                }
             }
        }

        walk_expr(self, expr);
    }
}

impl<'tcx> LateLintPass<'tcx> for UnsafeExpect {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        _: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        span: Span,
        _: LocalDefId,
    ) {
        // If the function comes from a macro expansion, we don't want to analyze it.
        if span.from_expansion() {
            return;
        }

        let mut visitor = UnsafeExpectVisitor {
            cx,
            checked_exprs: HashSet::new(),
            conditional_checker: HashSet::new(),
        };

        walk_expr(&mut visitor, body.value);
    }
}