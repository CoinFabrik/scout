use std::path::Path;

pub fn is_special_directory(path: &Path) -> bool {
    path.ends_with(".cargo") || path.ends_with(".git") || path.ends_with("target")
}
