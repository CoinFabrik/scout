use std::{fs, path::PathBuf};

use anyhow::{bail, Result};
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
        CargoSubCommand::Scout(opts) => run_scout(opts),
    }
}

fn run_scout(opts: Scout) {
    env_logger::init();

    let mut metadata = MetadataCommand::new();
    if opts.manifest_path.is_some() {
        metadata.manifest_path(opts.manifest_path.clone().unwrap());
    }
    let metadata = metadata.exec().expect("Failed to get metadata");

    let cargo_config = Config::default().expect("Failed to get config");
    let detectors_config =
        detectors::get_detectors_configuration().expect("Failed to get detectors configuration");

    if opts.filter.is_some() && opts.exclude.is_some() {
        println!("You can't use `--exclude` and `--filter` at the same time.");
        return;
    }

    let detectors = Detectors::new(cargo_config.into(), detectors_config, metadata);

    let detectors_names = detectors
        .clone()
        .get_detectors_names()
        .expect("Failed to build detectors");
    if opts.list_detectors {
        println!("--------------------");
        println!("Available detectors:\n\n");
        let mut index = 1;
        for detector_name in detectors_names {
            println!("{} -> {}", index, detector_name);
            index += 1;
        }
        return;
    }

    let used_detectors = get_used_detectors(opts.clone(), detectors_names).unwrap();

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
        all: true,
        quiet: true,
        paths,
        args: opts.args,
        manifest_path: opts.manifest_path,
        ..Default::default()
    };

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

    dylint::run(&options)?;

    Ok(())
}

fn get_used_detectors(opts: Scout, detectors_names: Vec<String>) -> Result<Vec<String>> {
    let mut used_detectors = Vec::new();
    if opts.filter.is_some() {
        let parsed_detectors = opts
            .filter
            .unwrap()
            .to_lowercase()
            .trim()
            .replace('_', "-")
            .split(',')
            .map(|detector| detector.trim().to_string())
            .collect::<Vec<String>>();
        for detector in parsed_detectors {
            if detectors_names.contains(&detector.to_string()) {
                used_detectors.push(detector.to_string());
            } else {
                bail!("The detector '{}' doesn't exist", detector);
            }
        }
    } else if opts.exclude.is_some() {
        let parsed_detectors = opts
            .exclude
            .unwrap()
            .to_lowercase()
            .trim()
            .replace('_', "-")
            .split(',')
            .map(|detector| detector.trim().to_string())
            .collect::<Vec<String>>();
        used_detectors = detectors_names.clone();
        for detector in parsed_detectors {
            if detectors_names.contains(&detector.to_string()) {
                let index = used_detectors.iter().position(|x| x == &detector).unwrap();
                used_detectors.remove(index);
            } else {
                bail!("The detector '{}' doesn't exist", detector);
            }
        }
    } else {
        used_detectors = detectors_names;
    }

    Ok(used_detectors)
}
