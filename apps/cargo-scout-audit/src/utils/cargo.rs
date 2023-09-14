#[cfg(windows)]
use std::path::Path;
use std::{
    io::{IsTerminal, Write},
    process::Stdio,
};

use ansi_term::Style;

use super::command::Command;

#[must_use]
pub fn build(description: &str, quiet: bool) -> Command {
    cargo("build", "Building", description, quiet)
}
fn cargo(subcommand: &str, verb: &str, description: &str, quiet: bool) -> Command {
    if !quiet {
        // smoelius: Writing directly to `stderr` avoids capture by `libtest`.
        let message = format!("{verb} {description}");
        std::io::stderr()
            .write_fmt(format_args!(
                "{}\n",
                if std::io::stdout().is_terminal() {
                    Style::new().bold()
                } else {
                    Style::new()
                }
                .paint(message)
            ))
            .expect("Could not write to stderr");
    }
    let mut command = Command::new("cargo");
    #[cfg(windows)]
    {
        // Dylint annotation
        // smoelius: Work around: https://github.com/rust-lang/rustup/pull/2978
        let cargo_home = home::cargo_home().unwrap();
        let old_path = std::env::var("PATH").unwrap();
        let new_path = std::env::join_paths(
            std::iter::once(Path::new(&cargo_home).join("bin"))
                .chain(std::env::split_paths(&old_path)),
        )
        .unwrap();
        command.envs(vec![("PATH", new_path)]);
    }
    command.args([subcommand]);
    if quiet {
        command.stderr(Stdio::null());
    }
    command
}
