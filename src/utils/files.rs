use std::{
    fs::{create_dir_all, File},
    io::Read,
    path::{Path, PathBuf},
};

use simple_error::{bail, SimpleError};

pub fn get_base_dir() -> Result<PathBuf, SimpleError> {
    match dirs::home_dir() {
        Some(home_dir) => {
            let base_dir = home_dir.join(".bkmrk");
            if !base_dir.exists() {
                create_dir_all(&base_dir).unwrap();
            }
            Ok(base_dir)
        }
        None => bail!("ERROR: Couldn't get home directory."),
    }
}

pub fn get_db_path() -> Result<PathBuf, SimpleError> {
    let base_dir = match get_base_dir() {
        Ok(base_dir) => base_dir,
        Err(err) => {
            bail!("ERROR: Couldn't get base configuration directory\n{}", err)
        }
    };
    Ok(base_dir.join("data.db"))
}

pub fn read_file(file_path: &Path) -> Result<String, SimpleError> {
    match File::open(file_path) {
        Ok(mut file) => {
            let mut file_contents = String::new();
            match file.read_to_string(&mut file_contents) {
                Ok(_) => Ok(file_contents),
                Err(err) => bail!(
                    "ERROR: Couldn't read file `{}`: {}",
                    file_path.display(),
                    err
                ),
            }
        }
        Err(err) => bail!(
            "ERROR: Couldn't open file `{}`: {}",
            file_path.display(),
            err
        ),
    }
}
