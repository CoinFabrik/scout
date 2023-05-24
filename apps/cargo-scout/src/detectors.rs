use std::path::PathBuf;

use anyhow::Result;
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
pub use configuration::get_detectors_configuration;

pub struct Detectors {
    cargo_config: Config,
    detectors_configs: DetectorsConfigurationList,
    metadata: Metadata,
}

impl Detectors {
    /// Creates a new instance of `Detectors`
    pub fn new(
        cargo_config: Config,
        detectors_configs: DetectorsConfigurationList,
        metadata: Metadata,
    ) -> Self {
        Self {
            cargo_config,
            detectors_configs,
            metadata,
        }
    }

    /// Builds detectors and returns the paths to the built libraries
    pub fn build(self) -> Result<Vec<PathBuf>> {
        let detectors_paths = self
            .detectors_configs
            .iter()
            .map(|detectors_config| self.build_detectors(detectors_config.clone()))
            .flatten_ok()
            .collect::<Result<Vec<_>>>()?;

        Ok(detectors_paths)
    }

    fn build_detectors(&self, detectors_config: DetectorConfiguration) -> Result<Vec<PathBuf>> {
        let builder =
            DetectorBuilder::new(&self.cargo_config, detectors_config, self.metadata.clone());
        builder.build()
    }
}
