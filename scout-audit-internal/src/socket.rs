use std::net::{IpAddr, SocketAddr};

use futures::{future, prelude::*};

use std::sync::{Arc, Mutex};
use tarpc::{
    context,
    server::{self, incoming::Incoming, Channel},
    tokio_serde::formats::Json,
};
use serde::{Deserialize, Serialize};

use crate::detector::*;
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Detector {
    Soroban(SorobanDetector),
    Ink(InkDetector),
}

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
        format!("HOLA, {name}! {}", self.socket_addr)
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

    async fn push_finding(self, _: context::Context, finding: String) {
        println!("Finding: {} was added", finding);
        self.findings.lock().unwrap().push(finding);
    }

    async fn get_findings(self, _: context::Context) -> Vec<String> {
        self.findings.lock().unwrap().clone()
    }
}

async fn spawn(fut: impl Future<Output = ()> + Send + 'static) {
    tokio::spawn(fut);
}

pub async fn detector_server(
    server_addr: (IpAddr, u16),
    findings: Arc<Mutex<Vec<String>>>,
) -> anyhow::Result<()> {
    let listener = tarpc::serde_transport::tcp::listen(&server_addr, Json::default).await?;
    listener
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        .max_channels_per_key(100, |t| t.transport().peer_addr().unwrap().ip())
        .map(|channel| {
            println!(
                "Incoming connection from: {}",
                channel.transport().peer_addr().unwrap()
            );
            let server = Server {
                socket_addr: channel.transport().peer_addr().unwrap(),
                available_detectors: vec![],
                findings: findings.clone(),
            };
            channel.execute(server.serve()).for_each(spawn)
        })
        .buffer_unordered(100)
        .for_each(|_| async {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        })
        .await;

    Ok(())
}
