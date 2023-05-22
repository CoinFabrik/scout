use std::path::PathBuf;

use cargo::Config;
use cargo_metadata::MetadataCommand;
use dylint::Dylint;

use crate::detectors::Detectors;

mod detectors;
mod utils;

fn main() {
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
