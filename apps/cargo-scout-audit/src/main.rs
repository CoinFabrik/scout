use core::panic;
use std::{fs, path::PathBuf};

use cargo::Config;
use cargo_metadata::MetadataCommand;
use clap::{Parser, Subcommand};
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

    #[clap(short, long, value_name = "Type", help = "Sets the output type")]
    output: Option<String>,

    #[clap(long, value_name = "path", help = "Path to the stdout file.")]
    stdout_path: Option<String>,

    #[clap(long, value_name = "path", help = "Path to the stderr file.")]
    stderr_path: Option<String>,
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

    if opts.output.is_some() {
        let valid_formats = ["json", "html"];
        let format = opts.output.clone().unwrap();

        if valid_formats.contains(&format.as_str()) {
            run_dylint(detectors_paths, opts, Some(format)).expect("Failed to run dylint")
        }
    } else {
        run_dylint(detectors_paths, opts, None).expect("Failed to run dylint");
    }
}

fn run_dylint(
    detectors_paths: Vec<PathBuf>,
    opts: Scout,
    format: Option<String>,
) -> anyhow::Result<()> {
    let paths: Vec<String> = detectors_paths
        .iter()
        .map(|path| path.to_string_lossy().to_string())
        .collect();

    let mut options = Dylint {
        paths,
        args: opts.args,
        manifest_path: opts.manifest_path,
        pipe_stdout: opts.stdout_path,
        pipe_stderr: opts.stderr_path,
        ..Default::default()
    };

    let stderr_temp_file = tempfile::NamedTempFile::new().unwrap();
    let stdout_temp_file = tempfile::NamedTempFile::new().unwrap();

    if format.is_some() {
        options.pipe_stderr = Some(stderr_temp_file.path().to_str().unwrap().to_string());
        options.pipe_stdout = Some(stdout_temp_file.path().to_str().unwrap().to_string());
    }

    // If there is a need to exclude or filter by detector, the dylint tool needs to be recompiled.
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

    if let Some(format) = format {
        let stderr_file = fs::File::open(stderr_temp_file.path()).unwrap();
        let _stdout_file = fs::File::open(stdout_temp_file.path()).unwrap();

        match format.as_str() {
            "json" => {
                let mut html_file = fs::File::create("report.json").unwrap();
                std::io::Write::write_all(
                    &mut html_file,
                    format_into_json(stderr_file)
                        .expect("Failed to format into json")
                        .as_bytes(),
                )
                .expect("Failed to write into json file");
            }
            "html" => {
                let mut html_file = fs::File::create("report.html").unwrap();
                std::io::Write::write_all(
                    &mut html_file,
                    format_into_html(stderr_file)
                        .expect("Failed to format into html")
                        .as_bytes(),
                )
                .expect("Failed to write into html file");
            }
            _ => todo!(),
        }

        stderr_temp_file.close()?;
        stdout_temp_file.close()?;
    }

    Ok(())
}
