use std::{
    env::{self, VarError},
    fs::{self, File, OpenOptions},
    io::{ErrorKind, Read, Write},
    path::PathBuf,
};

fn create_default_config(file_path: PathBuf) -> anyhow::Result<File> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;
    let default_config: toml::Table = toml::toml! {
        [developer]
        [developer.soroban]
        enabled = true
        [developer.ink]
        enabled = true

        [auditor]
        [auditor.soroban]
        enabled = true
        [auditor.ink]
        enabled = true
    };
    let str = toml::to_string_pretty::<toml::Table>(&default_config)?;
    file.write_all(str.as_bytes())?;
    Ok(file)
}

fn read_config() -> anyhow::Result<()> {
    let base_path = match std::env::consts::OS {
        "windows" => env::var("USERPROFILE")? + "/scout/",
        _ => "~/.config/scout/".to_string(),
    };
    let path = PathBuf::from(base_path);
    if let Err(_metadata) = fs::metadata(&path) {
        fs::create_dir_all(&path)?;
    }
    let file_path = path.as_path().join("config.toml");
    let res_file = File::open(&file_path);
    let mut file = if res_file
        .as_ref()
        .is_err_and(|f| f.kind() == ErrorKind::NotFound)
    {
        create_default_config(file_path)?
    } else {
        res_file?
    };
    let mut toml_str = String::new();
    file.read_to_string(&mut toml_str)?;
    let config: toml::Table = toml::from_str(&toml_str)?;

    Ok(())
}
