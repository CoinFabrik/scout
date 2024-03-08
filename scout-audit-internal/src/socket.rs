// Copyright 2018 Google LLC
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

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
use std::sync::{Arc, Mutex};

use crate::detector::Detector;
#[tarpc::service]
pub trait DetectorSocket {
    async fn hello(name: String) -> String;
    async fn is_up() -> bool;
    async fn set_available_detectors(detectors: Vec<Detector>);
    async fn is_detector_available(detector: Detector) -> bool;
    async fn push_finding(finding: String);
    async fn get_findings() -> Vec<String>;
}

#[derive(Clone)]
struct Server {
    socket_addr: SocketAddr,
    available_detectors: Vec<Detector>,
    findings: Arc<Mutex<Vec<String>>>,

}

impl DetectorSocket for Server {
    async fn hello(self, _: context::Context, name: String) -> String {
        format!("Hello, {name}! You are connected from {}", self.socket_addr)
    }
    async fn is_up(self, _: context::Context) -> bool {
        true
    }
    async fn set_available_detectors(mut self, _: context::Context, detectors: Vec<Detector>) {
        self.available_detectors = detectors;
    }

    async fn is_detector_available(self, _: context::Context, detector: Detector) -> bool {
        self.available_detectors.contains(&detector)
    }

    async fn push_finding(mut self, _: context::Context, finding: String) {
        self.findings.lock().unwrap().push(finding);
    }

    async fn get_findings(self, _: context::Context) -> Vec<String> {
        self.findings.lock().unwrap().clone()
    }
}

async fn spawn(fut: impl Future<Output = ()> + Send + 'static) {
    tokio::spawn(fut);
}

pub async fn detector_server(findings: Arc<Mutex<Vec<String>>>
) -> anyhow::Result<()> {
    let server_addr = (IpAddr::V6(Ipv6Addr::LOCALHOST), 1177);

    let mut listener = tarpc::serde_transport::tcp::listen(&server_addr, Json::default).await?;
    listener.config_mut().max_frame_length(usize::MAX);
    listener
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        .max_channels_per_key(1, |t| t.transport().peer_addr().unwrap().ip())
        .map(|channel| {
            let server = Server {
                socket_addr: channel.transport().peer_addr().unwrap(),
                available_detectors: vec![],
                findings: findings.clone(),
            };
            channel.execute(server.serve()).for_each(spawn)
        })
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;

    Ok(())
}
