use std::path::PathBuf;

use anyhow::{Ok, Result};
use cargo::Config;
use itertools::Itertools;

use self::{
    builder::DetectorBuilder,
    configuration::{DetectorConfiguration, DetectorsConfigurationList},
};

mod builder;
mod configuration;
mod library;
mod source;

use cargo_metadata::Metadata;
pub use configuration::{get_detectors_configuration, get_local_detectors_configuration};

#[derive(Debug)]
pub struct Detectors {
    cargo_config: Config,
    detectors_configs: DetectorsConfigurationList,
    metadata: Metadata,
    verbose: bool,
}

impl Detectors {
    /// Creates a new instance of `Detectors`
    pub fn new(
        cargo_config: Config,
        detectors_configs: DetectorsConfigurationList,
        metadata: Metadata,
        verbose: bool,
    ) -> Self {
        Self {
            cargo_config,
            detectors_configs,
            metadata,
            verbose,
        }
    }

    /// Builds detectors and returns the paths to the built libraries
    pub fn build(self, used_detectors: Vec<String>) -> Result<Vec<PathBuf>> {
        let detectors_paths = self
            .detectors_configs
            .iter()
            .map(|detectors_config| {
                self.build_detectors(detectors_config.clone(), used_detectors.clone())
            })
            .flatten_ok()
            .collect::<Result<Vec<_>>>()?;

        Ok(detectors_paths)
    }

    pub fn get_detector_names(&self) -> Result<Vec<String>> {
        let detectors_names = self
            .detectors_configs
            .iter()
            .map(|detectors_config| {
                let builder = DetectorBuilder::new(
                    &self.cargo_config,
                    detectors_config.clone(),
                    self.metadata.clone(),
                    self.verbose,
                );
                builder.get_detector_names()
            })
            .flatten_ok()
            .collect::<Result<Vec<_>>>()?;

        Ok(detectors_names)
    }

    fn build_detectors(
        &self,
        detectors_config: DetectorConfiguration,
        used_detectors: Vec<String>,
    ) -> Result<Vec<PathBuf>> {
        let builder = DetectorBuilder::new(
            &self.cargo_config,
            detectors_config,
            self.metadata.clone(),
            self.verbose,
        );
        builder.build(used_detectors)
    }
}
