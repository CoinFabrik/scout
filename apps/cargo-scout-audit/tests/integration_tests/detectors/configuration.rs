use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail};
use itertools::Itertools;
use scout_audit_internal::{Detector, IntoEnumIterator};

use super::utils;

#[derive(Debug)]
pub struct Configuration {
    pub detectors: Vec<DetectorConfiguration>,
}

#[derive(Debug)]
pub struct DetectorConfiguration {
    pub detector: Detector,
    pub testcases: Vec<Testcase>,
}

#[derive(Debug)]
pub struct Testcase {
    pub vulnerable_path: Option<String>,
    pub remediated_path: Option<String>,
}

impl Configuration {
    pub fn build() -> anyhow::Result<Self> {
        // Get all testcases folders
        let cargo_scout_audit_path = utils::get_cargo_scout_audit_path()?;
        let testcases_root_path = cargo_scout_audit_path
            .parent()
            .ok_or(anyhow!("Failed to find testcases path"))?
            .parent()
            .ok_or(anyhow!("Failed to find testcases path"))?
            .join("test-cases");
        let testcases_paths: Vec<PathBuf> = std::fs::read_dir(&testcases_root_path)?
            .into_iter()
            .filter_map(|r| r.ok().map(|f| f.path()))
            .filter(|r| r.is_dir())
            .collect();

        Self::validate_all_detectors_found(testcases_paths)?;

        // Find all testcases for each detector
        let mut detectors_config = Vec::new();
        for detector in Detector::iter() {
            let detector_name = detector.to_string();
            let testcases_root_path = testcases_root_path.join(detector_name);
            let testcases_paths: Vec<PathBuf> = std::fs::read_dir(testcases_root_path)?
                .into_iter()
                .filter_map(|r| r.ok().map(|f| f.path()))
                .filter(|r| r.is_dir())
                .collect();

            let mut testcases = Vec::new();
            for testcase_path in testcases_paths {
                let vulnerable_path = testcase_path.join("vulnerable-example");
                let remediated_path = testcase_path.join("remediated-example");

                let testcase = Testcase {
                    vulnerable_path: if vulnerable_path.exists() {
                        Some(
                            vulnerable_path
                                .join("Cargo.toml")
                                .to_string_lossy()
                                .to_string(),
                        )
                    } else {
                        None
                    },
                    remediated_path: if Path::new(&remediated_path).exists() {
                        Some(
                            remediated_path
                                .join("Cargo.toml")
                                .to_string_lossy()
                                .to_string(),
                        )
                    } else {
                        None
                    },
                };
                testcases.push(testcase);
            }

            detectors_config.push(DetectorConfiguration {
                detector,
                testcases,
            });
        }

        Ok(Configuration {
            detectors: detectors_config,
        })
    }

    fn validate_all_detectors_found<T>(testcases_paths: T) -> anyhow::Result<()>
    where
        T: IntoIterator<Item = PathBuf>,
    {
        let count = testcases_paths
            .into_iter()
            .sorted()
            .zip(Detector::iter().map(|d| d.to_string()).sorted())
            .filter(|(p, d)| p.file_name().unwrap().to_string_lossy() != d.to_string())
            .count();

        if count > 0 {
            bail!("Testcases don't match detectors defined in scout-audit-internal.")
        }

        Ok(())
    }
}
