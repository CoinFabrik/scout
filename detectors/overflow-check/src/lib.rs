#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_span;

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
    str::from_utf8,
};

use rustc_ast::Crate;
use rustc_lint::{EarlyContext, EarlyLintPass, LintContext};
use rustc_span::DUMMY_SP;
use scout_audit_clippy_utils::diagnostics::span_lint_and_help;
use toml::Value;

const LINT_MESSAGE: &str = "Use `overflow-checks = true` in Cargo.toml profile";

dylint_linting::declare_early_lint! {
    /// ### What it does
    /// Checks that overflow-checks is enabled in Cargo.toml.
    ///
    /// ### Why is this bad?
    /// Integer overflow will trigger a panic in debug builds or will wrap in
    /// release mode. Division by zero will cause a panic in either mode. In some applications one
    /// wants explicitly checked, wrapping or saturating arithmetic.
    pub OVERFLOW_CHECK,
    Warn,
    LINT_MESSAGE,
    {
        name: "Overflow Check",
        long_message: "An overflow/underflow is typically caught and generates an error. When it is not caught, the operation will result in an inexact result which could lead to serious problems.",
        severity: "Critical",
        help: "TODO: ADD LINK",
        vulnerability_class: "Arithmetic",
    }
}

impl OverflowCheck {
    fn workspace_dir(&self) -> Result<PathBuf, String> {
        // Locate the project workspace or package root
        let output = Command::new(env!("CARGO"))
            .arg("locate-project")
            .arg("--workspace")
            .arg("--message-format=plain")
            .output()
            .map_err(|e| format!("Failed to execute cargo command: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "Cargo command failed with exit code: {}",
                output.status
            ));
        }

        // Convert the output to a string and parse it as a path
        let path_str = from_utf8(&output.stdout)
            .map_err(|_| "Output from cargo is not valid UTF-8".to_string())?
            .trim();

        // Find the parent directory, which should be the workspace directory
        let cargo_path = Path::new(path_str);
        cargo_path
            .parent()
            .ok_or_else(|| "Failed to find parent directory of the project".to_string())
            .map(|path| path.to_path_buf())
    }
}

impl EarlyLintPass for OverflowCheck {
    fn check_crate(&mut self, cx: &EarlyContext<'_>, _: &Crate) {
        // Attempt to get the workspace directory
        let workspace_dir = match self.workspace_dir() {
            Ok(dir) => dir,
            Err(err) => {
                cx.sess()
                    .struct_warn(format!("Failed to locate workspace directory: {}", err))
                    .emit();
                return;
            }
        };

        // Attempt to read Cargo.toml
        let cargo_toml_path = workspace_dir.join("Cargo.toml");
        let contents = match fs::read_to_string(&cargo_toml_path) {
            Ok(content) => content,
            Err(e) => {
                cx.sess()
                    .struct_warn(format!(
                        "Failed to read Cargo.toml from {:?}: {}",
                        cargo_toml_path, e
                    ))
                    .emit();
                return;
            }
        };

        // Attempt to parse Cargo.toml
        let toml = match contents.parse::<Value>() {
            Ok(parsed) => parsed,
            Err(e) => {
                cx.sess()
                    .struct_warn(format!("Failed to parse Cargo.toml: {}", e))
                    .emit();
                return;
            }
        };

        // Check if the profile.release.overflow-checks is enabled
        let overflow_checks = toml
            .get("profile")
            .and_then(|p| p.get("release"))
            .and_then(|r| r.get("overflow-checks"));

        // Check if overflow-checks is enabled
        match overflow_checks {
            Some(Value::Boolean(true)) => (), // All good
            Some(_) | None => {
                span_lint_and_help(
                    cx,
                    OVERFLOW_CHECK,
                    DUMMY_SP,
                    LINT_MESSAGE,
                    None,
                    "Enable overflow-checks on the release profile",
                );
            }
        }
    }
}
