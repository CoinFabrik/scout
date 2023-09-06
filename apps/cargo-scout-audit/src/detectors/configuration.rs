use std::path::PathBuf;

use anyhow::Result;
use cargo::{
    core::{Dependency, GitReference, SourceId},
    util::IntoUrl,
};

#[derive(Debug, Clone)]
pub struct DetectorConfiguration {
    pub dependency: Dependency,
    pub pattern: Option<String>,
}

pub type DetectorsConfigurationList = Vec<DetectorConfiguration>;

/// Returns list of detectors.
pub fn get_detectors_configuration() -> Result<DetectorsConfigurationList> {
    let detectors = vec![DetectorConfiguration {
        dependency: Dependency::parse(
            "library",
            None,
            SourceId::for_git(
                &"https://github.com/CoinFabrik/scout".into_url()?,
                GitReference::DefaultBranch,
            )?,
        )?,
        pattern: Some("detectors/*".into()),
    }];

    Ok(detectors)
}

/// Returns local detectors configuration from custom path.
pub fn get_local_detectors_configuration<T>(paths: T) -> Result<DetectorsConfigurationList>
where
    T: IntoIterator<Item = PathBuf>,
{
    let detectors = paths
        .into_iter()
        .map(|path| {
            Ok(DetectorConfiguration {
                dependency: Dependency::parse("library", None, SourceId::for_path(&path)?)?,
                pattern: None,
            })
        })
        .collect::<Result<DetectorsConfigurationList>>()?;

    Ok(detectors)
}
