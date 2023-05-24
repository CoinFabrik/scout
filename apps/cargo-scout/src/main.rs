use std::path::PathBuf;

use cargo::Config;
use cargo_metadata::MetadataCommand;
use clap::{Parser, Subcommand};
use dylint::Dylint;

use crate::detectors::Detectors;

mod detectors;
mod utils;

#[derive(Debug, Parser)]
#[clap(display_name = "cargo")]
struct Cli {
    #[clap(subcommand)]
    subcmd: CargoSubCommand,
}

#[derive(Debug, Subcommand)]
enum CargoSubCommand {
    Scout(Scout),
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Scout {}

fn main() {
    let cli = Cli::parse();
    match cli.subcmd {
        CargoSubCommand::Scout(opts) => run_scout(opts),
    }
}

fn run_scout(_opts: Scout) {
    env_logger::init();

    let metadata = MetadataCommand::new()
        .exec()
        .expect("Failed to get metadata");
    let cargo_config = Config::default().expect("Failed to get config");
    let detectors_config =
        detectors::get_detectors_configuration().expect("Failed to get detectors configuration");

    let detectors = Detectors::new(cargo_config, detectors_config, metadata);
    let detectors_paths = detectors.build().expect("Failed to build detectors");

    run_dylint(detectors_paths).expect("Failed to run dylint");
}

fn run_dylint(detectors_paths: Vec<PathBuf>) -> anyhow::Result<()> {
    let paths = detectors_paths
        .iter()
        .map(|path| path.to_string_lossy().to_string())
        .collect();

    let options = Dylint {
        paths,
        ..Default::default()
    };
    dylint::run(&options)?;

    Ok(())
}
