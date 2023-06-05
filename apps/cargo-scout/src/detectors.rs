use std::{path::PathBuf, rc::Rc};

use anyhow::{bail, Ok, Result};
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

#[derive(Debug, Clone)]
pub struct Detectors {
    cargo_config: Rc<Config>,
    detectors_configs: DetectorsConfigurationList,
    metadata: Metadata,
}

impl Detectors {
    /// Creates a new instance of `Detectors`
    pub fn new(
        cargo_config: Rc<Config>,
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

    pub fn get_detectors_names(self) -> Result<Vec<String>> {
        let detectors_names = self
            .detectors_configs
            .iter()
            .map(|detectors_config| {
                let builder = DetectorBuilder::new(
                    &self.cargo_config,
                    detectors_config.clone(),
                    self.metadata.clone(),
                );
                builder.get_detectors_names()
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
        let builder =
            DetectorBuilder::new(&self.cargo_config, detectors_config, self.metadata.clone());
        builder.build(used_detectors)
    }

    pub fn get_filtered_detectors(
        filter: String,
        detectors_names: Vec<String>,
    ) -> Result<Vec<String>> {
        let mut used_detectors: Vec<String> = Vec::new();
        let parsed_detectors = filter
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
        Ok(used_detectors)
    }

    pub fn get_excluded_detectors(
        excluded: String,
        detectors_names: Vec<String>,
    ) -> Result<Vec<String>> {
        let mut used_detectors = detectors_names.clone();
        let parsed_detectors = excluded
            .to_lowercase()
            .trim()
            .replace('_', "-")
            .split(',')
            .map(|detector| detector.trim().to_string())
            .collect::<Vec<String>>();
        for detector in parsed_detectors {
            if detectors_names.contains(&detector.to_string()) {
                let index = used_detectors.iter().position(|x| x == &detector).unwrap();
                used_detectors.remove(index);
            } else {
                bail!("The detector '{}' doesn't exist", detector);
            }
        }
        Ok(used_detectors)
    }

    pub fn list_detectors(detectors_names: Vec<String>) -> Result<()> {
        let separator = "─".repeat(48);
        let upper_border = format!("┌{}┐", separator);
        let lower_border = format!("└{}┘", separator);
        let empty_line = format!("│{:48}│", "");

        println!("{}", upper_border);
        println!("│{:^47}│", "🔍 Available detectors:");
        println!("{}", empty_line);

        for (index, detector_name) in detectors_names.iter().enumerate() {
            println!("│ {:<1}. {:<44}│", index + 1, detector_name);
        }

        println!("{}", empty_line);
        println!("{}", lower_border);
        Ok(())
    }
}
