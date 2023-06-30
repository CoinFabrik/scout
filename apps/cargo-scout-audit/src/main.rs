use std::{fs, path::PathBuf};

use cargo::Config;
use cargo_metadata::MetadataCommand;
use clap::{Parser, Subcommand};
use dylint::Dylint;
use utils::detectors::{get_excluded_detectors, get_filtered_detectors, list_detectors};

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
    ScoutAudit(Scout),
}

#[derive(Debug, Parser, Clone)]
#[command(author, version, about, long_about = None)]
struct Scout {
    #[clap(short, long, value_name = "path", help = "Path to Cargo.toml.")]
    manifest_path: Option<String>,

    // Exlude detectors
    #[clap(
        short,
        long,
        value_name = "detector/s",
        help = "Exclude the given detectors, separated by commas."
    )]
    exclude: Option<String>,

    // Filter by detectors
    #[clap(
        short,
        long,
        value_name = "detector/s",
        help = "Filter by the given detectors, separated by commas."
    )]
    filter: Option<String>,

    // List all the available detectors
    #[clap(short, long, help = "List all the available detectors")]
    list_detectors: bool,

    #[clap(last = true, help = "Arguments for `cargo check`")]
    args: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    match cli.subcmd {
        CargoSubCommand::ScoutAudit(opts) => run_scout(opts),
    }
}

fn run_scout(opts: Scout) {
    env_logger::init();

    if opts.filter.is_some() && opts.exclude.is_some() {
        panic!("You can't use `--exclude` and `--filter` at the same time.");
    }

    let mut metadata = MetadataCommand::new();
    if opts.manifest_path.is_some() {
        metadata.manifest_path(opts.manifest_path.clone().unwrap());
    }
    let metadata = metadata.exec().expect("Failed to get metadata");

    let cargo_config = Config::default().expect("Failed to get config");
    let detectors_config =
        detectors::get_detectors_configuration().expect("Failed to get detectors configuration");

    let detectors = Detectors::new(cargo_config, detectors_config, metadata);

    let detectors_names = detectors
        .get_detector_names()
        .expect("Failed to build detectors");
    if opts.list_detectors {
        list_detectors(detectors_names).expect("Failed to list detectors");
        return;
    }

    let used_detectors: Vec<String> = if opts.filter.is_some() {
        get_filtered_detectors(opts.clone().filter.unwrap(), detectors_names).unwrap()
    } else if opts.exclude.is_some() {
        get_excluded_detectors(opts.clone().exclude.unwrap(), detectors_names).unwrap()
    } else {
        detectors_names
    };

    let detectors_paths = detectors
        .build(used_detectors)
        .expect("Failed to build detectors");

    run_dylint(detectors_paths, opts).expect("Failed to run dylint");
}

fn run_dylint(detectors_paths: Vec<PathBuf>, opts: Scout) -> anyhow::Result<()> {
    let paths: Vec<String> = detectors_paths
        .iter()
        .map(|path| path.to_string_lossy().to_string())
        .collect();

    let options = Dylint {
        paths,
        args: opts.args,
        manifest_path: opts.manifest_path,
        ..Default::default()
    };

    // TODO: Improve this
    if opts.exclude.is_some() || opts.filter.is_some() {
        if let Some(manifest_path) = &options.manifest_path {
            // Get the directory of manifest path
            let manifest_path = PathBuf::from(manifest_path)
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            fs::remove_dir_all(format!("{manifest_path}/target/dylint"))
                .expect("Error removing directory");
        } else {
            fs::remove_dir_all("target/dylint").expect("Error removing directory");
        }
    }

    dylint::run(&options)?;

    Ok(())
}
