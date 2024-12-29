use std::path::PathBuf;

use anyhow::{Error, Result};

use crate::config::{self, Config};

use super::structs::JavaConfig;

pub fn save_java_to_config(paths: (PathBuf, PathBuf, PathBuf)) -> Result<(), Error> {
    let (java_8_path, java_17_path, java_21_path) = paths;
    let config = config::get_config()?;

    let new_config = Config {
        accounts: config.accounts,
        rich_presence: config.rich_presence,
        java: JavaConfig {
            java_8_path: java_8_path.to_string_lossy().to_string(),
            java_17_path: java_17_path.to_string_lossy().to_string(),
            java_21_path: java_21_path.to_string_lossy().to_string(),
        },
    };

    config::save_config(&new_config)?;

    Ok(())
}
