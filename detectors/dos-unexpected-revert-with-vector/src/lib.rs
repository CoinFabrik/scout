#![feature(rustc_private)]
#![warn(unused_extern_crates)]
#![feature(let_chains)]

extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;

use std::collections::HashSet;

use rustc_hir::intravisit::walk_expr;
use rustc_hir::intravisit::Visitor;
use rustc_hir::QPath;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::mir::{
    BasicBlock, BasicBlockData, BasicBlocks, ConstantKind, Operand, Place, StatementKind,
    TerminatorKind,
};
use rustc_middle::ty::TyKind;
use rustc_span::def_id::DefId;
use rustc_span::Span;
use scout_audit_internal::Detector;

dylint_linting::impl_late_lint! {
    /// ### What it does
    /// Checks for array pushes without access control.
    /// ### Why is this bad?
    /// Arrays have a maximum size according to the storage cell. If the array is full, the push will revert. This can be used to prevent the execution of a function.
    /// ### Known problems
    /// If the owner validation is performed in an auxiliary function, the warning will be shown, resulting in a false positive.
    /// ### Example
    /// ```rust
    /// if self.votes.contains(candidate) {
    ///     Err(Errors::CandidateAlreadyAdded)
    /// } else {
    ///     self.candidates.push(candidate);
    ///     self.votes.insert(candidate, &0);
    ///     Ok(())
    /// }
    /// ```
    /// Use instead:
    /// ```rust
    /// if self.votes.contains(candidate) {
    ///     Err(Errors::CandidateAlreadyAdded)
    /// } else {
    ///     self.candidates.insert(self.total_candidates, &candidate);
    ///     self.total_candidates += 1;
    ///     self.votes.insert(candidate, &0);
    ///     Ok(())
    /// }
    /// ```
    pub UNEXPECTED_REVERT_WARN,
    Warn,
    Detector::DosUnexpectedRevertWithVector.get_lint_message(),
    UnexpectedRevertWarn::default()
}

#[derive(Default)]
pub struct UnexpectedRevertWarn {}
impl UnexpectedRevertWarn {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'tcx> LateLintPass<'tcx> for UnexpectedRevertWarn {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: rustc_hir::intravisit::FnKind<'tcx>,
        _: &'tcx rustc_hir::FnDecl<'tcx>,
        body: &'tcx rustc_hir::Body<'tcx>,
        _: Span,
        localdef: rustc_span::def_id::LocalDefId,
    ) {
        struct UnprotectedVectorFinder<'tcx, 'tcx_ref> {
            cx: &'tcx_ref LateContext<'tcx>,
            callers_def_id: HashSet<DefId>,
            push_def_id: Option<DefId>,
        }
        impl<'tcx> Visitor<'tcx> for UnprotectedVectorFinder<'tcx, '_> {
            fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
                if let ExprKind::MethodCall(path, receiver, ..) = expr.kind {
                    let defid = self.cx.typeck_results().type_dependent_def_id(expr.hir_id);
                    let ty = self.cx.tcx.mk_foreign(defid.unwrap());
                    if ty.to_string().contains("std::vec::Vec") {
                        if path.ident.name.to_string() == "push" {
                            self.push_def_id = defid;
                        }
                    } else if let ExprKind::MethodCall(rec_path, receiver2, ..) = receiver.kind
                        && rec_path.ident.name.to_string() == "env"
                        && let ExprKind::Path(rec2_qpath) = &receiver2.kind
                        && let QPath::Resolved(qualifier, rec2_path) = rec2_qpath
                        && rec2_path.segments.first().map_or(false, |seg| {
                            seg.ident.to_string() == "self" && qualifier.is_none()
                        })
                        && path.ident.name.to_string() == "caller"
                    {
                        if self
                            .cx
                            .typeck_results()
                            .type_dependent_def_id(expr.hir_id)
                            .is_some()
                        {
                            self.callers_def_id.insert(
                                self.cx
                                    .typeck_results()
                                    .type_dependent_def_id(expr.hir_id)
                                    .unwrap(),
                            );
                        }
                    } else if let ExprKind::Call(receiver2, ..) = receiver.kind
                        && let ExprKind::Path(rec2_qpath) = &receiver2.kind
                        && let QPath::TypeRelative(ty2, rec2_path) = rec2_qpath
                        && rec2_path.ident.name.to_string() == "env"
                        && let rustc_hir::TyKind::Path(rec3_qpath) = &ty2.kind
                        && let QPath::Resolved(_, rec3_path) = rec3_qpath
                        && rec3_path.segments[0].ident.to_string() == "Self"
                        && self
                            .cx
                            .typeck_results()
                            .type_dependent_def_id(expr.hir_id)
                            .is_some()
                    {
                        self.callers_def_id.insert(
                            self.cx
                                .typeck_results()
                                .type_dependent_def_id(expr.hir_id)
                                .unwrap(),
                        );
                    }
                }
                walk_expr(self, expr);
            }
        }

        let mut uvf_storage = UnprotectedVectorFinder {
            cx,
            callers_def_id: HashSet::default(),
            push_def_id: None,
        };

        walk_expr(&mut uvf_storage, body.value);

        let mir_body = cx.tcx.optimized_mir(localdef);

        struct CallersAndVecOps<'tcx> {
            callers: Vec<(&'tcx BasicBlockData<'tcx>, BasicBlock)>,
            vec_ops: Vec<(&'tcx BasicBlockData<'tcx>, BasicBlock)>,
        }

        fn find_caller_and_vec_ops_in_mir<'tcx>(
            bbs: &'tcx BasicBlocks<'tcx>,
            callers_def_id: HashSet<DefId>,
            push_def_id: Option<DefId>,
        ) -> CallersAndVecOps {
            let mut callers_vec = CallersAndVecOps {
                callers: vec![],
                vec_ops: vec![],
            };
            for (bb, bb_data) in bbs.iter().enumerate() {
                if bb_data.terminator.as_ref().is_none() {
                    continue;
                }
                let terminator = bb_data.terminator.clone().unwrap();
                if let TerminatorKind::Call { func, .. } = terminator.kind {
                    if let Operand::Constant(fn_const) = func
                        && let ConstantKind::Val(_const_val, ty) = fn_const.literal
                        && let TyKind::FnDef(def, _subs) = ty.kind()
                    {
                        if !callers_def_id.is_empty() {
                            for caller in &callers_def_id {
                                if caller == def {
                                    callers_vec
                                        .callers
                                        .push((bb_data, BasicBlock::from_usize(bb)));
                                }
                            }
                        } else {
                            for op in &push_def_id {
                                if op == def {
                                    callers_vec
                                        .vec_ops
                                        .push((bb_data, BasicBlock::from_usize(bb)));
                                }
                            }
                        }
                    }
                }
            }
            callers_vec
        }

        let caller_and_vec_ops = find_caller_and_vec_ops_in_mir(
            &mir_body.basic_blocks,
            uvf_storage.callers_def_id,
            uvf_storage.push_def_id,
        );

        if !caller_and_vec_ops.vec_ops.is_empty() {
            let unchecked_places = navigate_trough_basicblocks(
                &mir_body.basic_blocks,
                BasicBlock::from_u32(0),
                &caller_and_vec_ops,
                false,
                &mut vec![],
                &mut HashSet::<BasicBlock>::default(),
            );
            for place in unchecked_places {
                Detector::DosUnexpectedRevertWithVector.span_lint(
                    cx,
                    UNEXPECTED_REVERT_WARN,
                    place.1,
                );
            }
        }

        fn navigate_trough_basicblocks<'tcx>(
            bbs: &'tcx BasicBlocks<'tcx>,
            bb: BasicBlock,
            caller_and_vec_ops: &CallersAndVecOps<'tcx>,
            after_comparison: bool,
            tainted_places: &mut Vec<Place<'tcx>>,
            visited_bbs: &mut HashSet<BasicBlock>,
        ) -> Vec<(Place<'tcx>, Span)> {
            let mut ret_vec = Vec::<(Place, Span)>::new();
            if visited_bbs.contains(&bb) {
                return ret_vec;
            } else {
                visited_bbs.insert(bb);
            }
            if bbs[bb].terminator.is_none() {
                return ret_vec;
            }
            for statement in &bbs[bb].statements {
                if let StatementKind::Assign(assign) = &statement.kind {
                    match &assign.1 {
                        rustc_middle::mir::Rvalue::Ref(_, _, origplace)
                        | rustc_middle::mir::Rvalue::AddressOf(_, origplace)
                        | rustc_middle::mir::Rvalue::Len(origplace)
                        | rustc_middle::mir::Rvalue::CopyForDeref(origplace) => {
                            if tainted_places
                                .clone()
                                .into_iter()
                                .any(|place| place == *origplace)
                            {
                                tainted_places.push(assign.0);
                            }
                        }
                        rustc_middle::mir::Rvalue::Use(operand) => match &operand {
                            Operand::Copy(origplace) | Operand::Move(origplace) => {
                                if tainted_places
                                    .clone()
                                    .into_iter()
                                    .any(|place| place == *origplace)
                                {
                                    tainted_places.push(assign.0);
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
            match &bbs[bb].terminator().kind {
                TerminatorKind::SwitchInt { discr, targets } => {
                    let comparison_with_caller = match discr {
                        Operand::Copy(place) | Operand::Move(place) => {
                            tainted_places
                                .iter()
                                .any(|tainted_place| tainted_place == place)
                                || after_comparison
                        }
                        Operand::Constant(_cons) => after_comparison,
                    };
                    for target in targets.all_targets() {
                        ret_vec.append(&mut navigate_trough_basicblocks(
                            bbs,
                            *target,
                            caller_and_vec_ops,
                            comparison_with_caller,
                            tainted_places,
                            visited_bbs,
                        ));
                    }
                    return ret_vec;
                }
                TerminatorKind::Call {
                    destination,
                    args,
                    target,
                    fn_span,
                    ..
                } => {
                    for arg in args {
                        match arg {
                            Operand::Copy(origplace) | Operand::Move(origplace) => {
                                if tainted_places
                                    .clone()
                                    .into_iter()
                                    .any(|place| place == *origplace)
                                {
                                    tainted_places.push(*destination);
                                }
                            }
                            Operand::Constant(_) => {}
                        }
                    }
                    for caller in &caller_and_vec_ops.callers {
                        if caller.1 == bb {
                            tainted_places.push(*destination);
                        }
                    }
                    for map_op in &caller_and_vec_ops.vec_ops {
                        if map_op.1 == bb
                            && !after_comparison
                            && args.get(1).map_or(true, |f| {
                                f.place().is_some_and(|f| !tainted_places.contains(&f))
                            })
                        {
                            ret_vec.push((*destination, *fn_span))
                        }
                    }
                    if target.is_some() {
                        ret_vec.append(&mut navigate_trough_basicblocks(
                            bbs,
                            target.unwrap(),
                            caller_and_vec_ops,
                            after_comparison,
                            tainted_places,
                            visited_bbs,
                        ));
                    }
                }
                TerminatorKind::Assert { target, .. }
                | TerminatorKind::Goto { target, .. }
                | TerminatorKind::Drop { target, .. } => {
                    ret_vec.append(&mut navigate_trough_basicblocks(
                        bbs,
                        *target,
                        caller_and_vec_ops,
                        after_comparison,
                        tainted_places,
                        visited_bbs,
                    ));
                }
                TerminatorKind::Yield { resume, .. } => {
                    ret_vec.append(&mut navigate_trough_basicblocks(
                        bbs,
                        *resume,
                        caller_and_vec_ops,
                        after_comparison,
                        tainted_places,
                        visited_bbs,
                    ));
                }
                TerminatorKind::FalseEdge { real_target, .. }
                | TerminatorKind::FalseUnwind { real_target, .. } => {
                    ret_vec.append(&mut navigate_trough_basicblocks(
                        bbs,
                        *real_target,
                        caller_and_vec_ops,
                        after_comparison,
                        tainted_places,
                        visited_bbs,
                    ));
                }
                TerminatorKind::InlineAsm { destination, .. } => {
                    if destination.is_some() {
                        ret_vec.append(&mut navigate_trough_basicblocks(
                            bbs,
                            destination.unwrap(),
                            caller_and_vec_ops,
                            after_comparison,
                            tainted_places,
                            visited_bbs,
                        ));
                    }
                }
                TerminatorKind::Resume
                | TerminatorKind::Terminate
                | TerminatorKind::Return
                | TerminatorKind::Unreachable
                | TerminatorKind::GeneratorDrop => {}
            }
            ret_vec
        }
    }
}
