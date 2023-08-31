use core::panic;
use std::{fs, path::PathBuf};

use cargo::Config;
use cargo_metadata::MetadataCommand;
use clap::{Parser, Subcommand, ValueEnum};
use dylint::Dylint;
use utils::detectors::{get_excluded_detectors, get_filtered_detectors, list_detectors};
use utils::output::{format_into_html, format_into_json};

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
#[derive(Debug, Clone, ValueEnum, PartialEq)]
enum OutputFormat {
    Text,
    Json,
    Html,
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

    #[clap(
        short,
        long,
        value_name = "type",
        help = "Sets the output type",
        default_value = "text"
    )]
    output_format: OutputFormat,

    #[clap(long, value_name = "path", help = "Path to the output file.")]
    output_path: Option<String>,
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

    let mut options = Dylint {
        paths,
        args: opts.args,
        manifest_path: opts.manifest_path,
        pipe_stdout: opts.output_path.clone(),
        pipe_stderr: opts.output_path.clone(),
        ..Default::default()
    };

    let stderr_temp_file = tempfile::NamedTempFile::new()?;
    let stdout_temp_file = tempfile::NamedTempFile::new()?;

    if let Some(out_path) = &opts.output_path {
        let path = PathBuf::from(&out_path);
        if path.is_dir() {
            panic!("The output path can't be a directory.");
        }
    }

    if opts.output_path.is_some() || opts.output_format != OutputFormat::Text {
        options.pipe_stderr = Some(stderr_temp_file.path().to_str().unwrap().to_string());
        options.pipe_stdout = Some(stdout_temp_file.path().to_str().unwrap().to_string());
    }

    // If there is a need to exclude or filter by detector, the dylint tool needs to be recompiled.
    // TODO: improve detector system so that doing this isn't necessary.
    if opts.exclude.is_some() || opts.filter.is_some() {
        let target_dylint_path = match &options.manifest_path {
            Some(manifest_path) => {
                let manifest_path = PathBuf::from(manifest_path);
                let manifest_path_parent = manifest_path
                    .parent()
                    .expect("Error getting manifest path parent");
                manifest_path_parent.join("target").join("dylint")
            }
            None => std::env::current_dir()
                .expect("Failed to get current dir")
                .join("target")
                .join("dylint"),
        };
        if target_dylint_path.exists() {
            fs::remove_dir_all(target_dylint_path).expect("Error removing target/dylint directory");
        }
    }

    dylint::run(&options)?;

    let mut stderr_file = fs::File::open(stderr_temp_file.path())?;
    let mut _stdout_file = fs::File::open(stdout_temp_file.path())?;

    match opts.output_format {
        OutputFormat::Json => {
            let mut json_file = match &opts.output_path {
                Some(path) => fs::File::create(path)?,
                None => fs::File::create("report.json")?,
            };
            std::io::Write::write_all(&mut json_file, format_into_json(stderr_file)?.as_bytes())?;
        }
        OutputFormat::Html => {
            let mut html_file = match &opts.output_path {
                Some(path) => fs::File::create(path)?,
                None => fs::File::create("report.html")?,
            };
            std::io::Write::write_all(&mut html_file, format_into_html(stderr_file)?.as_bytes())?;
        }
        OutputFormat::Text => {
            let mut txt_file = match &opts.output_path {
                Some(path) => fs::File::create(path)?,
                None => fs::File::create("report.txt")?,
            };
            std::io::copy(&mut stderr_file, &mut txt_file)?;
        }
    }

    stderr_temp_file.close()?;
    stdout_temp_file.close()?;

    Ok(())
}
