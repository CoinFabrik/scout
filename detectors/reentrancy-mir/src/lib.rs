#![feature(rustc_private)]
#![warn(unused_extern_crates)]
#![feature(let_chains)]

extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;
extern crate log;

use if_chain::if_chain;
use log::debug;
use rustc_hir::intravisit::{walk_expr, FnKind};
use rustc_hir::intravisit::{walk_stmt, Visitor};
use rustc_hir::{Body, FnDecl, HirId, Stmt};
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_middle::mir::{Place, BasicBlockData, BasicBlocks};
use rustc_middle::mir::{BasicBlock, LocalDecl, StatementKind, Local, TerminatorKind, Operand, terminator, Terminator, ConstantKind, interpret::{ConstValue}};
use rustc_middle::ty::TyKind;
use rustc_span::Span;
use rustc_span::def_id::DefId;
use rustc_span::sym::Default;
use clippy_utils::diagnostics::span_lint;

dylint_linting::impl_late_lint! {
    pub REENTRANCY,
    Warn,
    "description goes here",
    Reentrancy::default()
}

#[derive(Default)]
pub struct Reentrancy {
}
impl Reentrancy {
    pub fn new() -> Self {
        Self {
        }
    }
}

pub fn navigateTroughBasicBlocks<'tcx>(bb: &BasicBlock, bbs: &BasicBlocks<'tcx>, tainted_places_param: &Vec<&Place<'tcx>>, stop_on_func: DefId, cx: &LateContext<'tcx>) {
    let mut tainted_places: Vec<&Place<'tcx>> = vec![];
    for place in tainted_places_param {
        tainted_places.push(place);
    }
    dbg!(&bbs[*bb]);
    for statement in &bbs[*bb].statements {
        match &statement.kind {
            StatementKind::StorageDead(local) => {
                tainted_places.retain(|place| place.local != *local);
            },
            StatementKind::Assign(assign) => {
                match assign.1 {
                    rustc_middle::mir::Rvalue::Ref(_, _, origplace) => {
                        //dbg!("Ref", &tainted_places, origplace);
                        if &tainted_places.clone().into_iter().filter(|place| place.local == origplace.local).count() > &0 {
                            tainted_places.push(&assign.0)
                        }
                    },
                    rustc_middle::mir::Rvalue::AddressOf(_, origplace) => {
                        //dbg!("AddressOf", &tainted_places, origplace);
                        if &tainted_places.clone().into_iter().filter(|place| place.local == origplace.local).count() > &0 {
                            tainted_places.push(&assign.0)
                        }
                    },
                    rustc_middle::mir::Rvalue::Len(origplace) => {
                        //dbg!("Len", &tainted_places, origplace);
                        if &tainted_places.clone().into_iter().filter(|place| place.local == origplace.local).count() > &0 {
                            tainted_places.push(&assign.0)
                        }
                    },
                    rustc_middle::mir::Rvalue::CopyForDeref(origplace) => {
                        //dbg!("CopyForDeref", &tainted_places, origplace);
                        if &tainted_places.clone().into_iter().filter(|place| place.local == origplace.local).count() > &0 {
                            tainted_places.push(&assign.0)
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
    if tainted_places.len() == 0 {
        return;
    }
    match &bbs[*bb].terminator().kind {
        TerminatorKind::Goto { target } => {
            navigateTroughBasicBlocks(target, bbs, &tainted_places, stop_on_func, cx);
        },
        TerminatorKind::SwitchInt { discr, targets } => {
            for target in targets.all_targets() {
                navigateTroughBasicBlocks(target, bbs, &tainted_places, stop_on_func, cx);
            }
        },
        TerminatorKind::Drop { place, target, unwind } => {
            navigateTroughBasicBlocks(target, bbs, &tainted_places, stop_on_func, cx);
        },
        TerminatorKind::Call { func, args, destination, target, unwind, from_hir_call, fn_span } => {
            dbg!(&tainted_places);
            let mut tainted_in_args = false;
            for arg in args {
                if arg.place().map_or(false, |v| tainted_places.contains(&&v)) {
                    tainted_places.push(destination);
                    tainted_in_args = true;
                }
            }
            if tainted_in_args && let Operand::Constant(cont) = func {
                if let rustc_middle::mir::ConstantKind::Val(const_val, val_type) = &cont.literal {
                    if let rustc_middle::ty::TyKind::FnDef(def, subs) = val_type.kind(){
                        if *def == stop_on_func {
                            span_lint(cx, REENTRANCY, *fn_span, "ACA SE LLAMA CON ARGS TAINTEADOS");
                            dbg!(tainted_in_args);
                        }
                    }
                }
            }
            if target.is_some() {
                navigateTroughBasicBlocks(&target.unwrap(), bbs, &tainted_places, stop_on_func, cx);
            }
        },
        TerminatorKind::FalseEdge { real_target, imaginary_target } => {
            navigateTroughBasicBlocks(real_target, bbs, &tainted_places, stop_on_func, cx);
        },
        TerminatorKind::FalseUnwind { real_target, unwind } => {
            navigateTroughBasicBlocks(real_target, bbs, &tainted_places, stop_on_func, cx);
        },
        TerminatorKind::InlineAsm { template, operands, options, line_spans, destination, unwind } => {
            if destination.is_some() {
                navigateTroughBasicBlocks(&destination.unwrap(), bbs, &tainted_places, stop_on_func, cx);
            }
        },
        _=>{}
    }
}

impl<'tcx> LateLintPass<'tcx> for Reentrancy {
    
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _:rustc_hir::intravisit::FnKind<'tcx>,
        _: &'tcx rustc_hir::FnDecl<'tcx>,
        body: &'tcx rustc_hir::Body<'tcx>,
        _:Span,
        localdef:rustc_span::def_id::LocalDefId) {
        
        struct SetAllowReentrancyFinder<'tcx,'tcx_ref>{
            cx: &'tcx_ref LateContext<'tcx>,
            def_id: Option<rustc_span::def_id::DefId>,
            invoke_def_id: Option<rustc_span::def_id::DefId>,
        }

        let mut sarf = SetAllowReentrancyFinder {
            cx: cx,
            def_id: None,
            invoke_def_id: None
        };

        impl<'tcx> Visitor<'tcx> for SetAllowReentrancyFinder<'tcx, '_> {
            fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
                if let rustc_hir::ExprKind::MethodCall(path, expr2, ..) = expr.kind {
                    if path.ident.name.to_string() == "set_allow_reentry" {
                        self.def_id = self.cx.typeck_results().type_dependent_def_id(expr.hir_id);
                    } else if path.ident.name.to_string() == "invoke_contract" {
                        self.invoke_def_id = self.cx.typeck_results().type_dependent_def_id(expr.hir_id);
                    }
                }
        
                walk_expr(self, expr);
            }
        }
        walk_expr(&mut sarf, body.value);

        let mir_body = cx.tcx.optimized_mir(localdef.to_def_id());
        
        let mut allow_reentry_true_places: Vec<&Place> = vec![];
        for bb in mir_body.basic_blocks.into_iter() {
            if let TerminatorKind::Call {
                func,
                args,
                destination,
                target,
                unwind,
                from_hir_call,
                fn_span 
            } = &bb.terminator().kind {
                if let Operand::Constant(cont) = func {
                    if let rustc_middle::mir::ConstantKind::Val(const_val, val_type) = &cont.literal {
                        if let rustc_middle::ty::TyKind::FnDef(def, subs) = val_type.kind(){
                            if sarf.def_id.is_some_and(|id|id==*def) {
                                //Check that second argument is true in set_allow_reentry [set_allow_reentry(self, true)]
                                if args.last().is_some() && 
                                    let Operand::Constant(arg) = args.last().unwrap() &&
                                    let ConstantKind::Val(const_val, val_type) = arg.literal &&
                                    *val_type.kind() == TyKind::Bool &&
                                    let ConstValue::Scalar(scalar) = const_val &&
                                    scalar.to_bool().map_or_else(|e| false, |f| f) {
                                        navigateTroughBasicBlocks(&target.unwrap(), &mir_body.basic_blocks, &vec![destination], sarf.invoke_def_id.unwrap(), cx)
                                }
                            }
                        }
                    }
                } else if let Operand::Copy(cpy) = func {
                    dbg!(cpy);
                } else if let Operand::Move(mov) = func {
                    dbg!(mov);
                }
            }
        }
        

        /*let local_decls = &mir_body.local_decls;
        
        let mut local_callflags: Vec<Local> = vec![];
        for (local, local_decl) in local_decls.iter_enumerated() {
            if local_decl.ty.sort_string(cx.tcx).contains("ink::ink_env::CallFlags") {
                local_callflags.push(local);
                dbg!(local.index());
            }
        }

        for bb in mir_body.basic_blocks.into_iter() {
            let mut there_is_callflag = false;
            for statement in bb.statements.iter() {
                if let StatementKind::StorageLive(local) = statement.kind {
                    for local_callflag in local_callflags.iter() {
                        if local_callflag == &local {
                            dbg!(local);
                            there_is_callflag = true;
                        }
                    }
                }
            }
            if there_is_callflag {
                if let TerminatorKind::Call { 
                    func, 
                    args, 
                    destination, 
                    target, 
                    unwind, 
                    from_hir_call, 
                    fn_span 
                } = &bb.terminator().kind {
                    dbg!(func);
                }
            }
        }*/

        /*if local_callflags.len() > 0 {
            for bb in mir_body.basic_blocks.into_iter() {
                let mut there_is_callflag: bool = false;
                for statement in bb.statements.iter() {
                    if let StatementKind::StorageLive(local) = statement.kind {
                        for local_callflag in local_callflags.iter() {
                            if local_callflag == &local {
                                dbg!(local);
                                there_is_callflag = true;
                            }
                        }
                    }
                }
                if there_is_callflag {
                    dbg!(bb);
                    if let TerminatorKind::Call { 
                        func, 
                        args, 
                        destination, 
                        target, 
                        unwind, 
                        from_hir_call, 
                        fn_span 
                    } = &bb.terminator().kind {
                        let target_bb = &mir_body.basic_blocks[target.unwrap()];
                        dbg!(target_bb);
                        if let TerminatorKind::Call { 
                            func, 
                            args, 
                            destination, 
                            target, 
                            unwind, 
                            from_hir_call, 
                            fn_span 
                        } = &target_bb.terminator().kind {
                            if let Operand::Constant(cont) = func {
                                if let rustc_middle::mir::ConstantKind::Val(const_val, val_type) = &cont.literal {
                                    if let rustc_middle::ty::TyKind::FnDef(def, subs) = val_type.kind(){
                                        dbg!(def);
                                        dbg!(def.index);
                                        dbg!(cx.tcx.fn_sig(def).0.);
                                    }
                                }
                            } else if let Operand::Copy(cpy) = func {
                                dbg!(cpy);
                            } else if let Operand::Move(mov) = func {
                                dbg!(mov);
                            }
                            /*dbg!(func, 
                            args, 
                            destination, 
                            target, 
                            unwind, 
                            from_hir_call, 
                            fn_span);*/
                            /*let target_bb2 = &mir_body.basic_blocks[target.unwrap()];
                            
                            dbg!(target_bb2);*/
                        }
                    }
                    //dbg!(&bb.terminator().kind);
                }
            }
        }*/
        
        
        // let statements = &mir_body.basic_blocks[BasicBlock::from_u32(0)].statements;
        
        //dbg!(statements.first());


        
        
    }
}