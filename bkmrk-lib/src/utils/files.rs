use std::{
    fs::{create_dir_all, File},
    io::Read,
    path::{Path, PathBuf},
};

use eyre::{eyre, Result, WrapErr};

pub fn get_base_dir() -> Result<PathBuf> {
    match dirs::home_dir() {
        Some(home_dir) => {
            let base_dir = home_dir.join(".bkmrk");
            if !base_dir.exists() {
                create_dir_all(&base_dir).unwrap();
            }
            Ok(base_dir)
        }
        None => Err(eyre!("ERROR: Couldn't get home directory.")),
    }
}

pub fn get_db_path() -> Result<PathBuf> {
    let base_dir = get_base_dir()?;
    Ok(base_dir.join("data.db"))
}

pub fn read_file(file_path: &Path) -> Result<String> {
    let mut file = File::open(file_path)
        .wrap_err_with(|| format!("ERROR: Couldn't open file {}", file_path.display()))?;
    let mut file_contents = String::new();

    file.read_to_string(&mut file_contents)
        .wrap_err_with(|| format!("ERROR: Couldn't read file {}", file_path.display()))?;
    Ok(file_contents)
}
