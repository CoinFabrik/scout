#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_span;

use std::fs;

use clippy_utils::diagnostics::span_lint_and_help;
use reqwest::{Client, Error};
use rustc_lint::{LateContext, LateLintPass};
use semver::*;
use serde::Deserialize;

dylint_linting::declare_late_lint! {
    /// ### What it does
    /// Checks the ink! version of the contract
    /// ### Why is this bad?
    /// Using an outdated version of ink! could lead to security vulnerabilities, bugs, and other issues.
    ///```

    pub CHECK_INK_VERSION,
    Warn,
    "Use the latest version of ink!"
}

#[derive(Deserialize)]
struct CrateResponse {
    #[serde(rename = "crate")]
    krate: Crate,
}

#[derive(Deserialize)]
struct Crate {
    max_version: String,
}

impl<'tcx> LateLintPass<'tcx> for CheckInkVersion {
    fn check_crate(&mut self, cx: &LateContext<'_>) {
        let latest_version = get_version();

        let latest_version = match latest_version {
            Ok(version) => version,
            Err(_) => return,
        };

        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

        let cargo_toml_path = std::path::Path::new(&manifest_dir).join("Cargo.toml");

        let cargo_toml = fs::read_to_string(cargo_toml_path).expect("Unable to read Cargo.toml");

        let toml: toml::Value = toml::from_str(&cargo_toml).unwrap();

        let ink_version = match toml
            .get("dependencies")
            .and_then(|d| d.get("ink").and_then(|i| i.get("version")))
        {
            Some(version) => version.to_string(),
            None => return,
        };

        let req = Version::parse(&latest_version.replace('\"', "")).unwrap();
        let ink_version = VersionReq::parse(&ink_version.replace('\"', "")).unwrap();

        if !ink_version.matches(&req) {
            span_lint_and_help(
                cx,
                CHECK_INK_VERSION,
                rustc_span::DUMMY_SP,
                &format!("The latest ink! version is {latest_version}, and your version is {ink_version}"),
                None,
                &format!("Please, use version {latest_version} of ink! in your Cargo.toml"),
            );
        }
    }
}

#[tokio::main]
async fn get_version() -> Result<String, Error> {
    let url = "https://crates.io/api/v1/crates/ink";

    let client = Client::builder().user_agent("Scout/1.0").build()?;

    let resp: CrateResponse = client.get(url).send().await?.json().await?;

    Ok(resp.krate.max_version)
}
