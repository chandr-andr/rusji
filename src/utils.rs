use crate::constance::*;
use home::home_dir;
use std::io::{Error, ErrorKind, Result};

pub fn build_full_app_path() -> Result<String> {
    let home_dir = home_dir();
    match home_dir {
        Some(path) => Ok(format!("{}/{}", path.display(), APP_DIRECTORY,)),
        None => Err(Error::new(
            ErrorKind::NotFound,
            "Can't find home directory!",
        )),
    }
}

pub fn build_app_config_path() -> Result<String> {
    match build_full_app_path() {
        Ok(path) => Ok(format!("{}/{}", path, APP_CONFIG,)),
        Err(err) => Err(err),
    }
}
