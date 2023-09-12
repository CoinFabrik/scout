#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_span;

use std::fs;

use rustc_lint::EarlyLintPass;
use scout_audit_internal::Detector;
use semver::*;

dylint_linting::declare_early_lint! {
    /// ### What it does
    /// Checks the ink! version of the contract
    /// ### Why is this bad?
    /// Using an outdated version of ink! could lead to security vulnerabilities, bugs, and other issues.
    ///```
    pub CHECK_INK_VERSION,
    Warn,
    Detector::InkVersion.get_lint_message()
}

impl EarlyLintPass for CheckInkVersion {
    fn check_crate(&mut self, cx: &rustc_lint::EarlyContext<'_>, _: &rustc_ast::Crate) {
        let latest_version = get_version();

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
            Detector::InkVersion.span_lint_and_help(
                cx,
                CHECK_INK_VERSION,
                rustc_span::DUMMY_SP,
                &format!("The latest ink! version is {latest_version}, and your version is {ink_version}"),
            );
        }
    }
}

fn get_version() -> String {
    let resp: serde_json::Value = ureq::get("https://crates.io/api/v1/crates/ink")
        .set("User-Agent", "Scout/1.0")
        .call()
        .expect("Failed to get ink! version from crates.io")
        .into_json()
        .expect("Failed to parse ink! version from crates.io");
    let version = resp
        .get("crate")
        .unwrap()
        .get("max_stable_version")
        .unwrap()
        .to_string();
    version
}
