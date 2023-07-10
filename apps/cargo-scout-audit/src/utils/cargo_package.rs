use std::path::Path;

use anyhow::{anyhow, ensure, Result};
use cargo::core::{PackageId, SourceId};
use cargo_metadata::{Metadata, MetadataCommand, Package as CargoPackage};

pub fn package_metadata(package_root: &Path) -> Result<Metadata, cargo_metadata::Error> {
    MetadataCommand::new()
        .current_dir(package_root)
        .no_deps()
        .exec()
}

pub fn package_id(
    source_id: SourceId,
    metadata: &Metadata,
    package_root: &Path,
) -> Result<PackageId> {
    let package = package_with_root(metadata, package_root)?;
    PackageId::new(&package.name, &package.version, source_id)
}

pub fn package_with_root(metadata: &Metadata, package_root: &Path) -> Result<CargoPackage> {
    let packages = metadata
        .packages
        .iter()
        .map(|package| {
            let path = package
                .manifest_path
                .parent()
                .ok_or_else(|| anyhow!("Could not get parent directory"))?;
            Ok(if path == package_root {
                Some(package)
            } else {
                None
            })
        })
        .filter_map(Result::transpose)
        .collect::<Result<Vec<_>>>()?;

    ensure!(
        packages.len() <= 1,
        "Found multiple packages in `{}`",
        package_root.to_string_lossy()
    );

    packages
        .into_iter()
        .next()
        .cloned()
        .ok_or_else(|| anyhow!("Foun no packages in `{}`", package_root.to_string_lossy()))
}

pub fn package_library_name(metadata: &Metadata, package_root: &Path) -> Result<String> {
    let package = package_with_root(metadata, package_root)?;

    package
        .targets
        .iter()
        .find_map(|target| {
            if target.kind.iter().any(|kind| kind == "cdylib") {
                Some(target.name.clone())
            } else {
                None
            }
        })
        .ok_or_else(|| {
            anyhow!(
                "Could not find `cdylib` target for package `{}`",
                package.id
            )
        })
}
