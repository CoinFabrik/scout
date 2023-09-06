use std::path::PathBuf;

use cargo_metadata::MetadataCommand;

pub fn get_cargo_scout_audit_path() -> cargo_metadata::Result<PathBuf> {
    Ok(MetadataCommand::new().exec()?.workspace_root.into())
}

pub fn get_repository_root_path() -> cargo_metadata::Result<PathBuf> {
    let mut path = get_cargo_scout_audit_path()?;
    path.pop();
    path.pop();
    Ok(path)
}
