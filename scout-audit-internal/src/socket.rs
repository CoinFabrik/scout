//extern crate rustc_lint;
//extern crate rustc_span;
use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    time::Duration,
};

use futures::{future, prelude::*};
use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};
use tarpc::{
    context,
    server::{self, incoming::Incoming, Channel},
    tokio_serde::formats::Json,
};
use tokio::time;

//use rustc_lint::{Lint, LintContext};
//use rustc_span::Span;
use crate::detector::Detector;

#[tarpc::service]
pub trait Findings {
    async fn hello(name: String) -> String;
}
#[derive(Clone)]
pub struct FindingsServer(SocketAddr);

#[tarpc::server]
impl Findings for FindingsServer {
    async fn hello(self, _: context::Context, msg: String) -> String {
        msg
    }
}
