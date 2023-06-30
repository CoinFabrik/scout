use std::path::Path;

use anyhow::{anyhow, Result};

use super::command::Command;

/// Returns `rustup` active toolchain
///
/// Dylint annotation
/// smoelius: Consider carefully whether you need to call this function! In most cases, the toolchain
/// you want is not the one returned by rustup.
pub fn active_toolchain(path: &Path) -> Result<String> {
    let output = Command::new("rustup")
        .sanitize_environment()
        .current_dir(path)
        .args(["show", "active-toolchain"])
        .output()?;
    let stdout = std::str::from_utf8(&output.stdout)?;
    stdout
        .split_once(' ')
        .map(|(s, _)| s.to_owned())
        .ok_or_else(|| anyhow!("Could not determine active toolchain"))
}
