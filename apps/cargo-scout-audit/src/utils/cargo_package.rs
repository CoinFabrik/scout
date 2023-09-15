use std::path::Path;

use cargo_metadata::{Metadata, MetadataCommand};

pub fn package_metadata(package_root: &Path) -> Result<Metadata, cargo_metadata::Error> {
    MetadataCommand::new()
        .current_dir(package_root)
        .no_deps()
        .exec()
}
