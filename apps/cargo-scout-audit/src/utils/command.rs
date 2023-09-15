use std::{
    ffi::OsStr,
    path::Path,
    process::{Command as StdCommand, Output, Stdio},
};

use anyhow::{ensure, Context, Result};

use super::env;

pub struct Command {
    command: StdCommand,
}

impl Command {
    pub fn new<S: AsRef<OsStr>>(program: S) -> Self {
        Self {
            command: StdCommand::new(program),
        }
    }

    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.command.args(args);
        self
    }

    #[allow(dead_code)]
    pub fn envs<I, K, V>(&mut self, vars: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.command.envs(vars);
        self
    }

    pub fn env_remove<K: AsRef<OsStr>>(&mut self, key: K) -> &mut Self {
        self.command.env_remove(key);
        self
    }

    pub fn current_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Self {
        self.command.current_dir(dir);
        self
    }

    #[allow(dead_code)]
    pub fn stdout<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Self {
        self.command.stdout(cfg);
        self
    }

    pub fn stderr<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Self {
        self.command.stderr(cfg);
        self
    }

    pub fn sanitize_environment(&mut self) -> &mut Self {
        self.env_remove(env::RUSTC);
        self.env_remove(env::RUSTUP_TOOLCHAIN);
        self
    }

    pub fn output(&mut self) -> Result<Output> {
        log::debug!("{:?}", self.command.get_envs().collect::<Vec<_>>());
        log::debug!("{:?}", self.command.get_current_dir());
        log::debug!("{:?}", self.command);

        let output = self
            .command
            .output()
            .with_context(|| format!("Could not get output of `{:?}`", self.command))?;

        ensure!(
            output.status.success(),
            "command failed: {:?}\nstdout: {:?}\nstderr: {:?}",
            self.command,
            std::str::from_utf8(&output.stdout).unwrap_or_default(),
            std::str::from_utf8(&output.stderr).unwrap_or_default()
        );

        Ok(output)
    }

    pub fn success(&mut self) -> Result<()> {
        log::debug!("{:?}", self.command.get_envs().collect::<Vec<_>>());
        log::debug!("{:?}", self.command.get_current_dir());
        log::debug!("{:?}", self.command);

        let status = self
            .command
            .status()
            .with_context(|| format!("Could not get status of `{:?}`", self.command))?;

        ensure!(status.success(), "command failed: {:?}", self.command);

        Ok(())
    }
}

#[allow(unused_variables)]
#[allow(dead_code)]
pub fn driver(toolchain: &str, driver: &Path) -> Result<Command> {
    #[allow(unused_mut)]
    let mut command = Command::new(driver);
    #[cfg(windows)]
    {
        // MinerSebas: To succesfully determine the dylint driver Version on Windows,
        // it is neccesary to add some Libraries to the Path.
        let rustup_home = std::env::var("RUSTUP_HOME")?;
        let old_path = std::env::var("PATH")?;
        let new_path = std::env::join_paths(
            std::iter::once(
                Path::new(&rustup_home)
                    .join("toolchains")
                    .join(toolchain)
                    .join("bin"),
            )
            .chain(std::env::split_paths(&old_path)),
        )?;
        command.envs(vec![("PATH", new_path)]);
    }
    Ok(command)
}
