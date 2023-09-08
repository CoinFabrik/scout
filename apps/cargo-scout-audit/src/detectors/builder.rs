use std::path::PathBuf;

use anyhow::{bail, ensure, Context, Ok, Result};
use cargo::Config;
use cargo_metadata::Metadata;
use glob::glob;
use itertools::Itertools;

use super::{configuration::DetectorConfiguration, library::Library, source::download_git_repo};
use crate::utils::{cargo_package, path::is_special_directory, rustup};

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
        let all_paths = self.parse_library_patterns(&detector_root)?;
        let filtered_paths = self.filter_library_paths(all_paths, used_detectors)?;
        let packages = self.get_packages(filtered_paths)?;
        let library_paths = self.build_packages(packages)?;

        Ok(library_paths)
    }

    pub fn get_detector_names(self) -> Result<Vec<String>> {
        let detector_root = self.download_detector()?;
        let paths = self.parse_library_patterns(&detector_root)?;
        let detector_names = paths
            .into_iter()
            .map(|path| {
                path.file_name()
                    .and_then(|file_name| file_name.to_str())
                    .expect("Error getting path")
                    .to_string()
            })
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

    /// Parse dependency root with given library pattern.
    fn parse_library_patterns(&self, dependency_root: &PathBuf) -> Result<Vec<PathBuf>> {
        let pattern = match &self.detectors_config.pattern {
            Some(pattern) => dependency_root.join(pattern),
            None => dependency_root.clone(),
        };

        let entries = glob(&pattern.to_string_lossy())?;

        let paths = entries
            .map(|entry| {
                entry.map_err(Into::into).and_then(|path| {
                    if let Some(pattern) = &self.detectors_config.pattern {
                        let path_buf = path
                            .canonicalize()
                            .with_context(|| format!("Could not canonicalize {path:?}"))?;
                        // Dylint annotation
                        // smoelius: On Windows, the dependency root must be canonicalized to ensure it
                        // has a path prefix.
                        let dependency_root =
                            dependency_root.canonicalize().with_context(|| {
                                format!("Could not canonicalize {dependency_root:?}")
                            })?;
                        ensure!(
                            path_buf.starts_with(&dependency_root),
                            "Pattern `{pattern}` could refer to `{}`, which is outside of `{}`",
                            path_buf.to_string_lossy(),
                            dependency_root.to_string_lossy()
                        );
                    }
                    Ok(path)
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(paths)
    }

    /// Parse library paths into packages.
    fn get_packages(&self, paths: Vec<PathBuf>) -> Result<Vec<Library>> {
        // Dylint annotation
        // smoelius: Collecting the package ids before building reveals missing/unparsable `Cargo.toml`
        // files sooner.

        // smoelius: Why are we doing this complicated dance at all? Because we want to leverage Cargo's
        // download cache. But we also want to support git repositories with libraries that use
        // different compiler versions. And we have to work around the fact that "all projects within a
        // workspace are intended to be built with the same version of the compiler"
        // (https://github.com/rust-lang/rustup/issues/1399#issuecomment-383376082).
        let packages = paths
            .into_iter()
            .map(|path| {
                if !path.is_dir() || is_special_directory(&path) {
                    return Ok(None);
                }

                let package_metadata = cargo_package::package_metadata(&path)?;
                let package_id = cargo_package::package_id(
                    self.detectors_config.dependency.source_id(),
                    &package_metadata,
                    &path,
                )?;
                let lib_name = cargo_package::package_library_name(&package_metadata, &path)?;
                let toolchain = rustup::active_toolchain(&path)?;
                Ok(Some(Library::new(
                    path,
                    package_id,
                    lib_name,
                    toolchain,
                    self.root_metadata
                        .target_directory
                        .clone()
                        .into_std_path_buf(),
                    package_metadata,
                )))
            })
            .collect::<Result<Vec<_>>>()?;
        let packages: Vec<Library> = packages.into_iter().flatten().collect();
        Ok(packages)
    }

    /// Builds packages returning library paths
    fn build_packages(&self, libraries: Vec<Library>) -> Result<Vec<PathBuf>> {
        let library_paths = libraries
            .into_iter()
            .map(|library| library.build(self.verbose))
            .collect::<Result<Vec<_>>>()?;

        Ok(library_paths)
    }

    fn filter_library_paths(
        &self,
        mut all_paths: Vec<PathBuf>,
        used_detectors: Vec<String>,
    ) -> Result<Vec<PathBuf>> {
        all_paths.retain(|path| {
            let file_name = path
                .file_name()
                .and_then(|file_name| file_name.to_str())
                .expect("Error getting path");
            used_detectors.contains(&file_name.to_string())
        });
        Ok(all_paths)
    }
}
