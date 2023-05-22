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

pub type DetectorsConfigurationList = [DetectorConfiguration; 1];

/// Returns list of detectors
pub fn get_detectors_configuration() -> Result<DetectorsConfigurationList> {
    // TODO: Make detectors configuration static
    let detectors = [DetectorConfiguration {
        dependency: Dependency::parse(
            "library",
            None,
            SourceId::for_git(
                &"https://github.com/CoinFabrik/web3-grant".into_url()?,
                GitReference::DefaultBranch,
            )?,
        )?,
        pattern: Some("detectors/dylint/smart-contract-linters/*".into()),
    }];

    Ok(detectors)
}
