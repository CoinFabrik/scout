#![feature(rustc_private)]
#![warn(unused_extern_crates)]
#![feature(let_chains)]

extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;
extern crate rustc_ast;

use rustc_ast::ast::UintTy;
use rustc_ast::{LitKind, LitIntType};
use rustc_hir::{FnRetTy, Expr, ExprKind, intravisit::{walk_expr, Visitor}};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::mir::{Place, BasicBlocks};
use rustc_middle::mir::{BasicBlock, StatementKind, Local, TerminatorKind, Operand, interpret::{ConstValue}};
use rustc_span::Span;
use rustc_span::def_id::DefId;
use rustc_span::sym::Default;
use clippy_utils::diagnostics::span_lint;

dylint_linting::impl_late_lint! {
    pub UNRESTRICTED_TRANSFER_FROM,
    Warn,
    "description goes here",
    UnrestrictedTransferFrom::default()
}

#[derive(Default)]
pub struct UnrestrictedTransferFrom {
}
impl UnrestrictedTransferFrom {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl<'tcx> LateLintPass<'tcx> for UnrestrictedTransferFrom {
    
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _:rustc_hir::intravisit::FnKind<'tcx>,
        fn_decl: &'tcx rustc_hir::FnDecl<'tcx>,
        body: &'tcx rustc_hir::Body<'tcx>,
        _:Span,
        localdef:rustc_span::def_id::LocalDefId) {
        
        struct SetAllowReentrancyFinder<'tcx,'tcx_ref>{
            cx: &'tcx_ref LateContext<'tcx>,
            def_id: Option<rustc_span::def_id::DefId>,
            pusharg_def_id: Option<rustc_span::def_id::DefId>,
        }

        let mut sarf = SetAllowReentrancyFinder {
            cx: cx,
            def_id: None,
            pusharg_def_id: None
        };

        impl<'tcx> Visitor<'tcx> for SetAllowReentrancyFinder<'tcx, '_> {
            fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
                match expr.kind {
                    ExprKind::Call(path, args) => {
                        // Look for Selector::new([0x54, 0xb3, 0xc7, 0x6e])
                        if let ExprKind::Path(qpath) = &path.kind && 
                            let rustc_hir::QPath::TypeRelative(ty, path_segment) = qpath &&
                            path_segment.ident.name.to_string() == "new" &&
                            let rustc_hir::TyKind::Path(qpath_2) = &ty.kind && 
                            let rustc_hir::QPath::Resolved(ty_2, path_2) = qpath_2 && 
                            path_2.segments.into_iter().any(|s|s.ident.name.to_string() == "Selector") &&
                            args.len() == 1 &&
                            let ExprKind::Array(sel_arr) = args.first().unwrap().kind &&
                            sel_arr.len() == 4 {
                                let transfer_from_selector = [0x54, 0xb3, 0xc7, 0x6e];
                                let mut is_tranfer_from = true;

                                for (id, expr) in sel_arr.into_iter().enumerate() {
                                    if let ExprKind::Lit(byte) = expr.kind && 
                                        let LitKind::Int(value, int_ty) = byte.node &&
                                        let LitIntType::Unsigned(u_ty) = int_ty &&
                                        u_ty == UintTy::U8 &&
                                        value == transfer_from_selector[id]
                                        {
                                            is_tranfer_from &= true;
                                    }
                                }
                                if is_tranfer_from {
                                    self.def_id = self.cx.typeck_results().type_dependent_def_id(path.hir_id);
                                }
                        }
                    },
                    ExprKind::MethodCall(path, self_arg, ..) => {
                        if path.ident.name.to_string() == "push_arg" {
                            self.pusharg_def_id = self.cx.typeck_results().type_dependent_def_id(expr.hir_id);
                        }
                    },
                    _=>{}
                }
        
                walk_expr(self, expr);
            }
        }

        if let FnRetTy::Return(ret_ty) = fn_decl.output &&
            let rustc_hir::TyKind::Path(qpath) = &ret_ty.kind &&
            let rustc_hir::QPath::Resolved(_, path) = qpath && 
            path.segments.last()
                .map_or(false, |s| 
                    s.ident.name.to_string() == "CallBuilder" ||
                    s.ident.name.to_string() == "CreateBuilder"
            ) {    
            return;
        }

        let mir_body = cx.tcx.optimized_mir(localdef.to_def_id());

        walk_expr(&mut sarf, body.value);

        if sarf.def_id.is_none() {
            return;
        }
        //vector with function args and variables derived from those args
        let mut tainted_locals: Vec<Local> = mir_body.args_iter().collect();

        for bb in mir_body.basic_blocks.iter() {
            for statement in &bb.statements {
                match &statement.kind {
                    StatementKind::Assign(assign) => {
                        match &assign.1 {
                            rustc_middle::mir::Rvalue::Ref(_, _, origplace) |
                            rustc_middle::mir::Rvalue::AddressOf(_, origplace) |
                            rustc_middle::mir::Rvalue::Len(origplace) |
                            rustc_middle::mir::Rvalue::CopyForDeref(origplace) => {
                                if tainted_locals.clone().into_iter().any(|local|local == origplace.local) {
                                    tainted_locals.push(assign.0.local);
                                }
                            },
                            rustc_middle::mir::Rvalue::Use(operand) => {
                                match &operand {
                                    Operand::Copy(origplace) |
                                    Operand::Move(origplace) => {
                                        if tainted_locals.clone().into_iter().any(|local|local == origplace.local) {
                                            tainted_locals.push(assign.0.local);
                                        }
                                    },
                                    _ =>{}
                                }
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }
        }
        for bb in mir_body.basic_blocks.iter() {
            if let TerminatorKind::Call {
                func,
                args,
                destination,
                target,
                unwind,
                from_hir_call,
                fn_span 
            } = &bb.terminator().kind {
                if let Operand::Constant(cont) = func &&
                    let rustc_middle::mir::ConstantKind::Val(const_val, val_type) = &cont.literal &&
                    let rustc_middle::ty::TyKind::FnDef(def, subs) = val_type.kind() &&
                    sarf.def_id.is_some_and(|id|id==*def) &&
                    target.is_some() {
                        //aca el terminator es la llamada a new, el destination tiene el place con el selector
                        //a partir de aca lo que hago es buscar donde se usa el selector y donde se le pushean args dados por el usuario
                        let mut tainted_selector_places: Vec<Local> = vec![destination.local];
                        fn navigate_trough_bbs(cx: &LateContext, bb: &BasicBlock, bbs: &BasicBlocks, tainted_locals: &Vec<Local>, tainted_selector_places: &mut Vec<Local>, sarf: &SetAllowReentrancyFinder) {
                            if let TerminatorKind::Call {
                                    func,
                                    args,
                                    destination,
                                    target,
                                    unwind,
                                    from_hir_call,
                                    fn_span
                                } = &bbs[*bb].terminator().kind &&
                                let Operand::Constant(cont) = func &&
                                let rustc_middle::mir::ConstantKind::Val(const_val, val_type) = &cont.literal &&
                                let rustc_middle::ty::TyKind::FnDef(def, subs) = val_type.kind() {
                                
                                if sarf.pusharg_def_id.is_some_and(|id|id==*def) {
                                    for arg in args {
                                        if arg.place().map_or(false, |place|tainted_locals.iter().any(|l|l == &place.local)) {
                                            span_lint(cx, UNRESTRICTED_TRANSFER_FROM, *fn_span, "this argument comes from a user-supplied argument");
                                        }
                                    }
                                }
                                if target.is_some() {
                                    navigate_trough_bbs(cx,&target.unwrap(), bbs, tainted_locals, tainted_selector_places, sarf);
                                }
                            }
                        }
                        navigate_trough_bbs(cx,&target.unwrap(),&mir_body.basic_blocks, &tainted_locals, &mut tainted_selector_places, &sarf);
                }
            }
        }

        /*
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
        */
        
    }
}