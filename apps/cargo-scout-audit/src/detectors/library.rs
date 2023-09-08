use std::{env::consts, path::PathBuf};

use ::cargo::core::PackageId;
use anyhow::Result;
use cargo_metadata::Metadata;

use crate::utils::{cargo, env};

/// Represents a Rust library.
#[derive(Debug, Clone)]
pub struct Library {
    pub root: PathBuf,
    pub id: PackageId,
    pub lib_name: String,
    pub toolchain: String,
    pub target_dir: PathBuf,
    pub metadata: Metadata,
}

impl Library {
    /// Creates a new instance of `Library`.
    pub fn new(
        root: PathBuf,
        id: PackageId,
        lib_name: String,
        toolchain: String,
        target_dir: PathBuf,
        metadata: Metadata,
    ) -> Self {
        Self {
            root,
            id,
            lib_name,
            toolchain,
            target_dir,
            metadata,
        }
    }

    /// Builds the library and returns its path.
    pub fn build(&self, verbose: bool) -> Result<PathBuf> {
        let library_path = self.path();
        let target_dir = self.target_directory();

        cargo::build(&format!("linter `{}`", self.id.name()), !verbose)
            .sanitize_environment()
            .env_remove(env::RUSTFLAGS)
            .current_dir(&self.root)
            .args(["--release"])
            .success()?;

        if !library_path.exists() {
            anyhow::bail!("Could not determine if {library_path:?} exists");
        }

        if !target_dir.exists() {
            std::fs::create_dir_all(&target_dir)?;
        }

        let new_library_path = target_dir.join(library_path.file_name().unwrap());
        std::fs::copy(&library_path, &new_library_path)?;

        Ok(new_library_path)
    }

    pub fn target_directory(&self) -> PathBuf {
        self.target_dir
            .join("scout/libraries")
            .join(&self.toolchain)
    }

    pub fn path(&self) -> PathBuf {
        self.metadata
            .target_directory
            .clone()
            .into_std_path_buf()
            .join("release")
            .join(format!(
                "{}{}@{}{}",
                consts::DLL_PREFIX,
                self.lib_name.replace('-', "_"),
                self.toolchain,
                consts::DLL_SUFFIX
            ))
    }
}
