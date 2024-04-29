#![feature(rustc_private)]
#![recursion_limit = "256"]

extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;

use std::collections::HashMap;

use rustc_hir::def_id::DefId;
use rustc_hir::intravisit::{walk_expr, FnKind, Visitor};
use rustc_hir::{Body, FnDecl};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::mir::traversal::preorder;
use rustc_middle::mir::{Local, Operand, Rvalue, TerminatorKind};
use rustc_middle::ty::TyKind;
use rustc_span::def_id::LocalDefId;
use rustc_span::Span;
use scout_audit_clippy_utils::match_def_path;

const LINT_MESSAGE: &str = "Lazy value was gotten here but never set afterwards";

dylint_linting::impl_late_lint! {
    pub LAZY_VALUES_NOT_SET,
    Warn,
    LINT_MESSAGE,
    LazyValuesNotSet::default(),
    {
        name: "Lazy values get and not set",
        long_message: "When a get is performed, a copy of the value is received; if that copy is modified, the new value must be set afterwards.",
        severity: "Critical",
        help: "https://coinfabrik.github.io/scout/docs/vulnerabilities/lazy-values-not-set",
        vulnerability_class: "Known Bugs",
    }
}

#[derive(Default)]
pub struct LazyValuesNotSet {
    lazy_set_defid: Option<DefId>,
    lazy_get_defid: Option<DefId>,
    mapping_insert_defid: Option<DefId>,
    mapping_get_defid: Option<DefId>,
}

struct FunFinderVisitor<'a, 'tcx: 'a> {
    cx: &'a LateContext<'tcx>,
    lazy_set_defid: Option<DefId>,
    lazy_get_defid: Option<DefId>,
    mapping_insert_defid: Option<DefId>,
    mapping_get_defid: Option<DefId>,
}

impl<'a, 'tcx> Visitor<'tcx> for FunFinderVisitor<'a, 'tcx> {
    fn visit_expr(&mut self, expr: &'tcx rustc_hir::Expr<'tcx>) {
        if let rustc_hir::ExprKind::MethodCall(path, receiver, _, _) = expr.kind {
            if path.ident.to_string().contains("get")
                || path.ident.to_string().contains("set")
                || path.ident.to_string().contains("insert")
            {
                let defid = self.cx.typeck_results().type_dependent_def_id(expr.hir_id);

                let receiver_type = self.cx.typeck_results().expr_ty(receiver);
                if let TyKind::Adt(def, _) = receiver_type.kind() {
                    if match_def_path(self.cx, def.did(), &["ink_storage", "lazy", "Lazy"]) {
                        if path.ident.to_string().contains("get") {
                            self.lazy_get_defid = defid;
                        } else {
                            self.lazy_set_defid = defid;
                        }
                    } else if match_def_path(
                        self.cx,
                        def.did(),
                        &["ink_storage", "lazy", "mapping", "Mapping"],
                    ) {
                        if path.ident.to_string().contains("get") {
                            self.mapping_get_defid = defid;
                        } else {
                            self.mapping_insert_defid = defid;
                        }
                    }
                }
            }
        }
        walk_expr(self, expr);
    }
}

impl<'tcx> LateLintPass<'tcx> for LazyValuesNotSet {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        _: &'tcx FnDecl<'_>,
        body: &'tcx Body<'_>,
        _: Span,
        id: LocalDefId,
    ) {
        //search for the defids of the different functions
        let mut visitor = FunFinderVisitor {
            cx,
            lazy_set_defid: None,
            lazy_get_defid: None,
            mapping_insert_defid: None,
            mapping_get_defid: None,
        };
        visitor.visit_expr(body.value);
        if visitor.lazy_set_defid.is_some() {
            self.lazy_set_defid = visitor.lazy_set_defid;
        }
        if visitor.lazy_get_defid.is_some() {
            self.lazy_get_defid = visitor.lazy_get_defid;
        }
        if visitor.mapping_insert_defid.is_some() {
            self.mapping_insert_defid = visitor.mapping_insert_defid;
        }
        if visitor.mapping_get_defid.is_some() {
            self.mapping_get_defid = visitor.mapping_get_defid;
        }

        let (_, hm) = self.get_func_info(cx, id.to_def_id(), &[], &[], &mut vec![]);
        for val in hm.values() {
            scout_audit_clippy_utils::diagnostics::span_lint(
                cx,
                LAZY_VALUES_NOT_SET,
                *val,
                LINT_MESSAGE,
            );
        }
    }
}

fn clean_local_upwards(local: Local, hm: &HashMap<Local, Vec<Local>>) -> Vec<Local> {
    let val = hm.get(&local);
    let mut ret_vec: Vec<Local> = vec![];
    if let Some(locals_vec) = val {
        ret_vec.extend(locals_vec);
        for local in locals_vec {
            ret_vec.extend(clean_local_upwards(*local, hm))
        }
    }
    ret_vec.dedup();
    ret_vec
}

impl LazyValuesNotSet {
    fn get_func_info(
        &mut self,
        cx: &LateContext,
        func_defid: DefId,
        tainted_get_map: &[Local],
        tainted_get_lazy: &[Local],
        visited_funs: &mut Vec<DefId>,
    ) -> (Vec<Local>, HashMap<Local, Span>) {
        if visited_funs.contains(&func_defid) {
            return (vec![], HashMap::new());
        }
        visited_funs.push(func_defid);
        let mir = cx.tcx.optimized_mir(func_defid);
        let mir_preorder = preorder(mir);
        let mut mapping_get_tainted_args: Vec<Local> = tainted_get_map.to_owned();
        let mut lazy_get_tainted_args: Vec<Local> = tainted_get_lazy.to_owned();
        let mut span_local: HashMap<Local, Span> = HashMap::new();
        let mut locals_dependencies: HashMap<Local, Vec<Local>> = HashMap::new();
        let mut locals_to_clean: Vec<Local> = vec![];
        for basicblock in mir_preorder {
            for stmt in basicblock.1.statements.iter().rev() {
                if let rustc_middle::mir::StatementKind::Assign(box_) = &stmt.kind {
                    let locals = get_locals_in_rvalue(&box_.1);
                    locals_dependencies.insert(box_.0.local, locals.clone());
                    for local in locals {
                        if mapping_get_tainted_args.contains(&local) {
                            mapping_get_tainted_args.push(box_.0.local);
                        }
                        if lazy_get_tainted_args.contains(&local) {
                            lazy_get_tainted_args.push(box_.0.local);
                        }
                    }
                }
            }
            if let Some(terminator) = &basicblock.1.terminator {
                if let TerminatorKind::Call {
                    func,
                    args,
                    destination,
                    fn_span,
                    ..
                } = &terminator.kind
                {
                    match func {
                        rustc_middle::mir::Operand::Copy(_)
                        | rustc_middle::mir::Operand::Move(_) => {}
                        rustc_middle::mir::Operand::Constant(b) => {
                            if let TyKind::FnDef(defid, _args) = b.ty().kind() {
                                //if the function is set or insert taint destinations local
                                if self.lazy_get_defid.is_some_and(|did| did == *defid) {
                                    lazy_get_tainted_args.push(destination.local);
                                    span_local.insert(destination.local, *fn_span);
                                } else if self.mapping_get_defid.is_some_and(|did| did == *defid) {
                                    mapping_get_tainted_args.push(destination.local);
                                    span_local.insert(destination.local, *fn_span);
                                }
                                //if the function is defined in the local crate call get_func_info recursively
                                else if defid.is_local() {
                                    //translate from my locals to the locals into the call
                                    let mut mapping_args: Vec<Local> = vec![];
                                    let mut lazy_args: Vec<Local> = vec![];
                                    for arg in args.iter().enumerate() {
                                        match arg.1 {
                                            Operand::Copy(a) | Operand::Move(a) => {
                                                if mapping_get_tainted_args.contains(&a.local) {
                                                    mapping_args.push(Local::from_usize(arg.0 + 1));
                                                }
                                                if lazy_get_tainted_args.contains(&a.local) {
                                                    lazy_args.push(Local::from_usize(arg.0 + 1));
                                                }
                                            }
                                            Operand::Constant(_) => {}
                                        }
                                    }
                                    let cleaned_taints = self.get_func_info(
                                        cx,
                                        *defid,
                                        &mapping_args,
                                        &lazy_args,
                                        visited_funs,
                                    );
                                    //get the locals translated from call locals
                                    for local in cleaned_taints.0 {
                                        let op_arg = args.get(local.as_usize() - 1);
                                        if let Some(arg) = op_arg {
                                            match arg {
                                                Operand::Copy(a) | Operand::Move(a) => {
                                                    //clean the taints
                                                    mapping_get_tainted_args
                                                        .retain(|i| a.local != *i);
                                                    lazy_get_tainted_args.retain(|i| a.local != *i);
                                                    //push locals to be cleaned before
                                                    locals_to_clean.push(a.local)
                                                }
                                                Operand::Constant(_) => {}
                                            }
                                        }
                                    }
                                }
                                //if is an insert call clean the taints upwards
                                else if self.lazy_set_defid.is_some_and(|did| did == *defid) {
                                    for arg in args {
                                        match arg {
                                            Operand::Copy(a) | Operand::Move(a) => {
                                                locals_to_clean.push(a.local);
                                            }
                                            Operand::Constant(_) => {}
                                        }
                                    }
                                } else if self.mapping_insert_defid.is_some_and(|did| did == *defid)
                                {
                                    for arg in args {
                                        match arg {
                                            Operand::Copy(a) | Operand::Move(a) => {
                                                locals_to_clean.push(a.local);
                                            }
                                            Operand::Constant(_) => {}
                                        }
                                    }
                                } else {
                                    let mut args_locals = vec![];
                                    for arg in args {
                                        match arg {
                                            Operand::Copy(a) | Operand::Move(a) => {
                                                args_locals.push(a.local);
                                            }
                                            Operand::Constant(_) => {}
                                        }
                                    }
                                    locals_dependencies.insert(destination.local, args_locals);
                                }
                            }
                        }
                    }
                }
            }
        }
        for local in locals_to_clean.clone() {
            locals_to_clean.extend(clean_local_upwards(local, &locals_dependencies));
        }
        locals_to_clean.dedup();
        for local in &locals_to_clean {
            span_local.remove(local);
        }
        (
            locals_to_clean
                .clone()
                .into_iter()
                .filter(|l| l.as_usize() <= mir.arg_count)
                .collect::<Vec<Local>>(),
            span_local,
        )
    }
}

fn get_locals_in_rvalue(rvalue: &Rvalue) -> Vec<Local> {
    fn op_local(op: &Operand) -> Vec<Local> {
        match op {
            rustc_middle::mir::Operand::Copy(p) | rustc_middle::mir::Operand::Move(p) => {
                vec![p.local]
            }
            rustc_middle::mir::Operand::Constant(_) => vec![],
        }
    }
    match rvalue {
        rustc_middle::mir::Rvalue::Use(op)
        | rustc_middle::mir::Rvalue::Repeat(op, _)
        | rustc_middle::mir::Rvalue::Cast(_, op, _)
        | rustc_middle::mir::Rvalue::UnaryOp(_, op) => op_local(op),
        rustc_middle::mir::Rvalue::Ref(_, _, p)
        | rustc_middle::mir::Rvalue::AddressOf(_, p)
        | rustc_middle::mir::Rvalue::Len(p)
        | rustc_middle::mir::Rvalue::CopyForDeref(p) => {
            vec![p.local]
        }
        rustc_middle::mir::Rvalue::BinaryOp(_, ops)
        | rustc_middle::mir::Rvalue::CheckedBinaryOp(_, ops) => {
            let mut v = op_local(&ops.0);
            v.extend(op_local(&ops.1));
            v
        }
        _ => vec![],
    }
}
