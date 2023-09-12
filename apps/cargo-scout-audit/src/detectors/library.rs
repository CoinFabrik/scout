use std::{env::consts, path::PathBuf};

use anyhow::Result;
use cargo_metadata::Metadata;
use itertools::Itertools;

use crate::utils::{cargo, env};

/// Represents a Rust library.
#[derive(Debug, Clone)]
pub struct Library {
    pub root: PathBuf,
    pub toolchain: String,
    pub target_dir: PathBuf,
    pub metadata: Metadata,
}

impl Library {
    /// Creates a new instance of `Library`.
    pub fn new(root: PathBuf, toolchain: String, target_dir: PathBuf, metadata: Metadata) -> Self {
        Self {
            root,
            toolchain,
            target_dir,
            metadata,
        }
    }

    /// Builds the library and returns its path.
    pub fn build(&self, verbose: bool) -> Result<Vec<PathBuf>> {
        // Build entire workspace
        cargo::build("detectors", !verbose)
            .sanitize_environment()
            .env_remove(env::RUSTFLAGS)
            .current_dir(&self.root)
            .args(["--release"])
            .success()?;

        // Verify all libraries were built
        let compiled_library_paths = self
            .metadata
            .packages
            .clone()
            .into_iter()
            .map(|p| self.path(p.name))
            .collect_vec();

        let unexistant_libraries = compiled_library_paths
            .clone()
            .into_iter()
            .filter(|p| !p.exists())
            .collect_vec();
        if !unexistant_libraries.is_empty() {
            anyhow::bail!("Could not determine if {:?} exist", unexistant_libraries);
        }

        // Copy libraries to target directory
        let target_dir = self.target_directory();
        if !target_dir.exists() {
            std::fs::create_dir_all(&target_dir)?;
        }

        let target_compiled_library_paths = compiled_library_paths
            .into_iter()
            .map(|p| {
                let target_path = target_dir.join(p.file_name().unwrap());
                std::fs::copy(&p, &target_path)?;
                Ok(target_path)
            })
            .collect::<Result<Vec<PathBuf>>>()?;

        Ok(target_compiled_library_paths)
    }

    pub fn target_directory(&self) -> PathBuf {
        self.target_dir
            .join("scout/libraries")
            .join(&self.toolchain)
    }

    pub fn path(&self, library_name: String) -> PathBuf {
        self.metadata
            .target_directory
            .clone()
            .into_std_path_buf()
            .join("release")
            .join(format!(
                "{}{}@{}{}",
                consts::DLL_PREFIX,
                library_name.replace('-', "_"),
                self.toolchain,
                consts::DLL_SUFFIX
            ))
    }
}
