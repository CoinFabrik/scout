#![feature(rustc_private)]
#![feature(let_chains)]

extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;

use std::collections::HashSet;

use rustc_hir::QPath;
use rustc_hir::{
    intravisit::{walk_expr, Visitor},
    Expr, ExprKind,
};
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
    pub UNPROTECTED_MAPPING_OPERATION,
    Warn,
    Detector::UnprotectedMappingOperation.get_lint_message(),
    UnprotectedMappingOperation::default()
}

#[derive(Default)]
pub struct UnprotectedMappingOperation {}
impl UnprotectedMappingOperation {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'tcx> LateLintPass<'tcx> for UnprotectedMappingOperation {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: rustc_hir::intravisit::FnKind<'tcx>,
        _: &'tcx rustc_hir::FnDecl<'tcx>,
        body: &'tcx rustc_hir::Body<'tcx>,
        _: Span,
        localdef: rustc_span::def_id::LocalDefId,
    ) {
        struct UnprotectedMappingOperationFinder<'tcx, 'tcx_ref> {
            cx: &'tcx_ref LateContext<'tcx>,
            caller_def_id: Option<DefId>,
            insert_def_id: Option<DefId>,
            remove_def_id: Option<DefId>,
            take_def_id: Option<DefId>,
        }
        impl<'tcx> Visitor<'tcx> for UnprotectedMappingOperationFinder<'tcx, '_> {
            fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
                if let ExprKind::MethodCall(path, receiver, ..) = expr.kind {
                    let defid = self.cx.typeck_results().type_dependent_def_id(expr.hir_id);

                    let mapping_type = self.cx.typeck_results().expr_ty_adjusted(receiver);

                    if mapping_type
                        .to_string()
                        .contains("ink::storage::Mapping<ink::ink_primitives::AccountId")
                    {
                        if path.ident.name.to_string() == "insert" {
                            self.insert_def_id = defid;
                        } else if path.ident.name.to_string() == "remove" {
                            self.remove_def_id = defid;
                        } else if path.ident.name.to_string() == "take" {
                            self.take_def_id = defid;
                        }
                    } else if let ExprKind::MethodCall(rec_path, reciever2, ..) = receiver.kind
                        && rec_path.ident.name.to_string() == "env"
                        && let ExprKind::Path(rec2_qpath) = &reciever2.kind
                        && let QPath::Resolved(qualifier, rec2_path) = rec2_qpath
                        && rec2_path.segments.first().map_or(false, |seg| {
                            seg.ident.to_string() == "self" && qualifier.is_none()
                        })
                        && path.ident.name.to_string() == "caller"
                    {
                        self.caller_def_id =
                            self.cx.typeck_results().type_dependent_def_id(expr.hir_id);
                    }
                }
                walk_expr(self, expr);
            }
        }

        let mut umrf_storage = UnprotectedMappingOperationFinder {
            cx,
            caller_def_id: None,
            insert_def_id: None,
            remove_def_id: None,
            take_def_id: None,
        };

        walk_expr(&mut umrf_storage, body.value);
        let mir_body = cx.tcx.optimized_mir(localdef);

        struct CallersAndMapOps<'tcx> {
            callers: Vec<(&'tcx BasicBlockData<'tcx>, BasicBlock)>,
            map_ops: Vec<(&'tcx BasicBlockData<'tcx>, BasicBlock)>,
        }

        fn find_caller_and_map_ops_in_mir<'tcx>(
            bbs: &'tcx BasicBlocks<'tcx>,
            caller_def_id: Option<DefId>,
            map_ops: Vec<Option<DefId>>,
        ) -> CallersAndMapOps {
            let mut callers_vec = CallersAndMapOps {
                callers: vec![],
                map_ops: vec![],
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
                        if caller_def_id.is_some_and(|d: DefId| d == *def) {
                            callers_vec
                                .callers
                                .push((bb_data, BasicBlock::from_usize(bb)));
                        } else {
                            for op in &map_ops {
                                if op.is_some_and(|d| d == *def) {
                                    callers_vec
                                        .map_ops
                                        .push((bb_data, BasicBlock::from_usize(bb)));
                                }
                            }
                        }
                    }
                }
            }
            callers_vec
        }

        let caller_and_map_ops = find_caller_and_map_ops_in_mir(
            &mir_body.basic_blocks,
            umrf_storage.caller_def_id,
            vec![
                umrf_storage.insert_def_id,
                umrf_storage.remove_def_id,
                umrf_storage.take_def_id,
            ],
        );

        if !caller_and_map_ops.map_ops.is_empty() {
            let unchecked_places = navigate_trough_basicblocks(
                &mir_body.basic_blocks,
                BasicBlock::from_u32(0),
                &caller_and_map_ops,
                false,
                &mut vec![],
                &mut HashSet::<BasicBlock>::default(),
            );
            for place in unchecked_places {
                Detector::UnprotectedMappingOperation.span_lint(
                    cx,
                    UNPROTECTED_MAPPING_OPERATION,
                    place.1,
                );
            }
        }

        fn navigate_trough_basicblocks<'tcx>(
            bbs: &'tcx BasicBlocks<'tcx>,
            bb: BasicBlock,
            caller_and_map_ops: &CallersAndMapOps<'tcx>,
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
                            caller_and_map_ops,
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
                    for caller in &caller_and_map_ops.callers {
                        if caller.1 == bb {
                            tainted_places.push(*destination);
                        }
                    }
                    for map_op in &caller_and_map_ops.map_ops {
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
                            caller_and_map_ops,
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
                        caller_and_map_ops,
                        after_comparison,
                        tainted_places,
                        visited_bbs,
                    ));
                }
                TerminatorKind::Yield { resume, .. } => {
                    ret_vec.append(&mut navigate_trough_basicblocks(
                        bbs,
                        *resume,
                        caller_and_map_ops,
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
                        caller_and_map_ops,
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
                            caller_and_map_ops,
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
