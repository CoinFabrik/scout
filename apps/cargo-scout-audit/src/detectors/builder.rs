use std::path::PathBuf;

use anyhow::{bail, ensure, Context, Ok, Result};
use cargo::Config;
use cargo_metadata::Metadata;
use itertools::Itertools;

use super::{configuration::DetectorConfiguration, library::Library, source::download_git_repo};
use crate::utils::{cargo_package, rustup};

pub struct DetectorBuilder<'a> {
    cargo_config: &'a Config,
    detectors_config: DetectorConfiguration,
    root_metadata: Metadata,
    verbose: bool,
}

impl<'a> DetectorBuilder<'a> {
    /// Creates a new instance of `DetectorsBuilder`.
    pub fn new(
        cargo_config: &'a Config,
        detectors_config: DetectorConfiguration,
        root_metadata: Metadata,
        verbose: bool,
    ) -> Self {
        Self {
            cargo_config,
            detectors_config,
            root_metadata,
            verbose,
        }
    }

    /// Compiles detector library and returns its path.
    pub fn build(self, used_detectors: Vec<String>) -> Result<Vec<PathBuf>> {
        let detector_root = self.download_detector()?;
        let workspace_path = self.parse_library_path(&detector_root)?;
        let library = self.get_library(workspace_path)?;
        let library_paths = self.build_detectors(library)?;
        let filtered_paths = self.filter_detectors(library_paths, used_detectors)?;

        Ok(filtered_paths)
    }

    /// Returns list of detector names.
    pub fn get_detector_names(self) -> Result<Vec<String>> {
        let detector_root = self.download_detector()?;
        let workspace_path = self.parse_library_path(&detector_root)?;
        let library = self.get_library(workspace_path)?;
        let detector_names = library
            .metadata
            .packages
            .into_iter()
            .map(|p| p.name)
            .collect_vec();
        Ok(detector_names)
    }

    /// Downloads and returns detector root from supported sources.
    fn download_detector(&self) -> Result<PathBuf> {
        if self.detectors_config.dependency.source_id().is_git() {
            download_git_repo(&self.detectors_config.dependency, self.cargo_config)
        } else if self.detectors_config.dependency.source_id().is_path() {
            if let Some(path) = self.detectors_config.dependency.source_id().local_path() {
                Ok(path)
            } else {
                bail!(
                    "Path source should have a local path: {}",
                    self.detectors_config.dependency.source_id()
                )
            }
        } else {
            bail!(format!(
                "Unsupported source id: {}",
                self.detectors_config.dependency.source_id()
            ));
        }
    }

    /// Parse dependency root with given library path.
    fn parse_library_path(&self, dependency_root: &PathBuf) -> Result<PathBuf> {
        let path = match &self.detectors_config.path {
            Some(path) => dependency_root.join(path),
            None => dependency_root.clone(),
        };
        let path = dunce::canonicalize(&path)
            .with_context(|| format!("Could not canonicalize {path:?}"))?;
        let dependency_root = dunce::canonicalize(dependency_root)
            .with_context(|| format!("Could not canonicalize {dependency_root:?}"))?;
        ensure!(
            path.starts_with(&dependency_root),
            "Path could refer to `{}`, which is outside of `{}`",
            path.to_string_lossy(),
            dependency_root.to_string_lossy()
        );
        Ok(path)
    }

    /// Parse workspace path into library.
    fn get_library(&self, workspace_path: PathBuf) -> Result<Library> {
        // Dylint annotation
        // smoelius: Collecting the package ids before building reveals missing/unparsable `Cargo.toml`
        // files sooner.

        // smoelius: Why are we doing this complicated dance at all? Because we want to leverage Cargo's
        // download cache. But we also want to support git repositories with libraries that use
        // different compiler versions. And we have to work around the fact that "all projects within a
        // workspace are intended to be built with the same version of the compiler"
        // (https://github.com/rust-lang/rustup/issues/1399#issuecomment-383376082).
        ensure!(
            workspace_path.is_dir(),
            "Not a directory: {}",
            workspace_path.to_string_lossy()
        );

        let package_metadata = cargo_package::package_metadata(&workspace_path)?;
        let toolchain = rustup::active_toolchain(&workspace_path)?;
        let library = Library::new(
            workspace_path,
            toolchain,
            self.root_metadata
                .target_directory
                .clone()
                .into_std_path_buf(),
            package_metadata,
        );
        Ok(library)
    }

    /// Builds detectors returning their compiled paths.
    fn build_detectors(&self, library: Library) -> Result<Vec<PathBuf>> {
        let library_paths = library.build(self.verbose)?;
        Ok(library_paths)
    }

    fn filter_detectors(
        &self,
        detector_paths: Vec<PathBuf>,
        used_detectors: Vec<String>,
    ) -> Result<Vec<PathBuf>> {
        let mut filtered_paths = Vec::new();

        for path in detector_paths {
            let detector_name = path.file_name().unwrap().to_str().unwrap().to_string();

            #[cfg(not(windows))]
            let detector_name = detector_name.split("lib").collect::<Vec<_>>()[1];

            let detector_name = detector_name.split('@').collect::<Vec<_>>()[0]
                .to_string()
                .replace('_', "-");
            if used_detectors.contains(&detector_name) {
                filtered_paths.push(path)
            } else {
                std::fs::remove_file(path)?;
            }
        }

        Ok(filtered_paths)
    }
}
