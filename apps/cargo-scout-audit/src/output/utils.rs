use std::io::Write;
use std::{
    fs::{rename, File},
    path::PathBuf,
};

// Writes data to a file at the specified path.
pub fn write_to_file(path: &PathBuf, data: &[u8]) -> Result<(), std::io::Error> {
    // Write to a temporary file first
    let temp_path = path.with_extension("tmp");
    let mut temp_file = File::create(&temp_path)?;
    temp_file.write_all(data)?;

    // Rename temporary file to the target path
    rename(temp_path, path)?;

    Ok(())
}
