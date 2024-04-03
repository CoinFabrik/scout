#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_span;

use if_chain::if_chain;
use rustc_ast::{
    ptr::P,
    tokenstream::{TokenStream, TokenTree},
    AttrArgs, AttrKind, Expr, ExprKind, Item, MacCall, Stmt, StmtKind,
};
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_span::{sym, Span};
use scout_audit_clippy_utils::sym;
use scout_audit_internal::{DetectorImpl, InkDetector as Detector};

