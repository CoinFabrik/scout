use std::path::PathBuf;

use anyhow::{anyhow, bail, ensure, Result};
use cargo::{
    core::{source::MaybePackage, Dependency, Package, PackageId, QueryKind, Source},
    Config,
};

/// Downloads git repo using cargo native cache and returns its path.
pub fn download_git_repo(dependency: &Dependency, config: &Config) -> Result<PathBuf> {
    let _lock = config.acquire_package_cache_lock()?;
    let mut source = dependency.source_id().load(config, &Default::default())?;
    let package_id = sample_package_id(dependency, &mut *source)?;

    if let MaybePackage::Ready(package) = source.download(package_id)? {
        git_dependency_root_from_package(config, &*source, &package)
    } else {
        bail!(format!("`{}` is not ready", package_id.name()))
    }
}

fn sample_package_id(dep: &Dependency, source: &mut dyn Source) -> anyhow::Result<PackageId> {
    let mut package_id: Option<PackageId> = None;

    while {
        let poll = source.query(dep, QueryKind::Fuzzy, &mut |summary| {
            if package_id.is_none() {
                package_id = Some(summary.package_id());
            }
        })?;
        if poll.is_pending() {
            source.block_until_ready()?;
            package_id.is_none()
        } else {
            false
        }
    } {}

    package_id.ok_or_else(|| anyhow!("Found no packages in `{}`", dep.source_id()))
}

fn git_dependency_root_from_package<'a>(
    config: &'a Config,
    source: &(dyn Source + 'a),
    package: &Package,
) -> anyhow::Result<PathBuf> {
    let package_root = package.root();

    if source.source_id().is_git() {
        let git_path = config.git_path();
        let git_path = config.assert_package_cache_locked(&git_path);
        ensure!(
            package_root.starts_with(git_path.join("checkouts")),
            "Unexpected path: {}",
            package_root.to_string_lossy()
        );
        let n = git_path.components().count() + 3;
        Ok(package_root.components().take(n).collect())
    } else if source.source_id().is_path() {
        unreachable!()
    } else {
        unimplemented!()
    }
}
