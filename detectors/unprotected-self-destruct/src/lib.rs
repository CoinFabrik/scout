#![feature(rustc_private)]
#![feature(let_chains)]

extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;

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
    pub UNPROTECTED_SELF_DESTRUCT,
    Warn,
    Detector::UnprotectedSelfDestruct.get_lint_message(),
    UnprotectedSelfDestruct::default()
}

#[derive(Default)]
pub struct UnprotectedSelfDestruct {}
impl UnprotectedSelfDestruct {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'tcx> LateLintPass<'tcx> for UnprotectedSelfDestruct {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: rustc_hir::intravisit::FnKind<'tcx>,
        _: &'tcx rustc_hir::FnDecl<'tcx>,
        body: &'tcx rustc_hir::Body<'tcx>,
        _: Span,
        localdef: rustc_span::def_id::LocalDefId,
    ) {
        struct UnprotectedSelfDestructFinder<'tcx, 'tcx_ref> {
            cx: &'tcx_ref LateContext<'tcx>,
            terminate_contract_span: Option<Span>,
            terminate_contract_def_id: Option<DefId>,
            caller_def_id: Option<DefId>,
        }

        impl<'tcx> Visitor<'tcx> for UnprotectedSelfDestructFinder<'tcx, '_> {
            fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
                if let ExprKind::MethodCall(path, receiver, ..) = expr.kind
                    && let ExprKind::MethodCall(rec_path, reciever2, ..) = receiver.kind
                    && rec_path.ident.name.to_string() == "env"
                    && let ExprKind::Path(rec2_qpath) = &reciever2.kind
                    && let QPath::Resolved(qualifier, rec2_path) = rec2_qpath
                    && rec2_path.segments.first().map_or_else(
                        || false,
                        |seg| seg.ident.to_string() == "self" && qualifier.is_none(),
                    )
                {
                    if path.ident.name.to_string() == "terminate_contract" {
                        self.terminate_contract_span = Some(expr.span);
                        self.terminate_contract_def_id =
                            self.cx.typeck_results().type_dependent_def_id(expr.hir_id);
                    } else if path.ident.name.to_string() == "caller" {
                        self.caller_def_id =
                            self.cx.typeck_results().type_dependent_def_id(expr.hir_id);
                    }
                }

                walk_expr(self, expr);
            }
        }

        let mut usd_storage = UnprotectedSelfDestructFinder {
            cx,
            terminate_contract_def_id: None,
            terminate_contract_span: None,
            caller_def_id: None,
        };

        walk_expr(&mut usd_storage, body.value);
        let mir_body = cx.tcx.optimized_mir(localdef);

        struct CallersAndTerminates<'tcx> {
            callers: Vec<(&'tcx BasicBlockData<'tcx>, BasicBlock)>,
            terminates: Vec<(&'tcx BasicBlockData<'tcx>, BasicBlock)>,
        }

        fn find_caller_and_terminate_in_mir<'tcx>(
            bbs: &'tcx BasicBlocks<'tcx>,
            caller_def_id: Option<DefId>,
            terminate_def_id: Option<DefId>,
        ) -> CallersAndTerminates {
            let mut callers_vec = CallersAndTerminates {
                callers: vec![],
                terminates: vec![],
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
                        if caller_def_id.is_some_and(|d| d == *def) {
                            callers_vec
                                .callers
                                .push((bb_data, BasicBlock::from_usize(bb)));
                        } else if terminate_def_id.is_some_and(|d| d == *def) {
                            callers_vec
                                .terminates
                                .push((bb_data, BasicBlock::from_usize(bb)));
                        }
                    }
                }
            }
            callers_vec
        }

        let caller_and_terminate = find_caller_and_terminate_in_mir(
            &mir_body.basic_blocks,
            usd_storage.caller_def_id,
            usd_storage.terminate_contract_def_id,
        );

        if !caller_and_terminate.terminates.is_empty() {
            if caller_and_terminate.callers.is_empty() {
                for terminate in caller_and_terminate.terminates {
                    if let TerminatorKind::Call { fn_span, .. } = terminate.0.terminator().kind {
                        Detector::UnprotectedSelfDestruct.span_lint(
                            cx,
                            UNPROTECTED_SELF_DESTRUCT,
                            fn_span,
                        );
                    }
                }
            } else {
                let unchecked_places = navigate_trough_basicblocks(
                    &mir_body.basic_blocks,
                    BasicBlock::from_u32(0),
                    &caller_and_terminate,
                    false,
                    &mut vec![],
                );
                for place in unchecked_places {
                    Detector::UnprotectedSelfDestruct.span_lint(
                        cx,
                        UNPROTECTED_SELF_DESTRUCT,
                        place.1,
                    );
                }
            }
        }

        fn navigate_trough_basicblocks<'tcx>(
            bbs: &'tcx BasicBlocks<'tcx>,
            bb: BasicBlock,
            caller_and_terminate: &CallersAndTerminates<'tcx>,
            after_comparison: bool,
            tainted_places: &mut Vec<Place<'tcx>>,
        ) -> Vec<(Place<'tcx>, Span)> {
            let mut ret_vec = Vec::<(Place, Span)>::new();
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
                            caller_and_terminate,
                            comparison_with_caller,
                            tainted_places,
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
                    for caller in &caller_and_terminate.callers {
                        if caller.1 == bb {
                            tainted_places.push(*destination);
                        }
                    }
                    for terminate in &caller_and_terminate.terminates {
                        if terminate.1 == bb && !after_comparison {
                            ret_vec.push((*destination, *fn_span))
                        }
                    }
                    if target.is_some() {
                        ret_vec.append(&mut navigate_trough_basicblocks(
                            bbs,
                            target.unwrap(),
                            caller_and_terminate,
                            after_comparison,
                            tainted_places,
                        ));
                    }
                }
                TerminatorKind::Assert { target, .. }
                | TerminatorKind::Goto { target, .. }
                | TerminatorKind::Drop { target, .. } => {
                    ret_vec.append(&mut navigate_trough_basicblocks(
                        bbs,
                        *target,
                        caller_and_terminate,
                        after_comparison,
                        tainted_places,
                    ));
                }
                TerminatorKind::Yield { resume, .. } => {
                    ret_vec.append(&mut navigate_trough_basicblocks(
                        bbs,
                        *resume,
                        caller_and_terminate,
                        after_comparison,
                        tainted_places,
                    ));
                }
                TerminatorKind::FalseEdge { real_target, .. } => {
                    ret_vec.append(&mut navigate_trough_basicblocks(
                        bbs,
                        *real_target,
                        caller_and_terminate,
                        after_comparison,
                        tainted_places,
                    ));
                }
                TerminatorKind::FalseUnwind { real_target, .. } => {
                    ret_vec.append(&mut navigate_trough_basicblocks(
                        bbs,
                        *real_target,
                        caller_and_terminate,
                        after_comparison,
                        tainted_places,
                    ));
                }
                TerminatorKind::InlineAsm { destination, .. } => {
                    if destination.is_some() {
                        ret_vec.append(&mut navigate_trough_basicblocks(
                            bbs,
                            destination.unwrap(),
                            caller_and_terminate,
                            after_comparison,
                            tainted_places,
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
