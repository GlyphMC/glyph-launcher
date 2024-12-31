use std::path::PathBuf;

use anyhow::{Error, Result};

use crate::config::{self, Config};

use super::structs::JavaConfig;

pub fn save_java_to_config(paths: (PathBuf, PathBuf, PathBuf)) -> Result<(), Error> {
    let (java_8_path, java_17_path, java_21_path) = paths;
    let config = config::get_config()?;

    let os_suffix = if cfg!(target_os = "windows") {
        "\\bin\\javaw.exe"
    } else {
        "/bin/java"
    };

    let new_config = Config {
        accounts: config.accounts,
        rich_presence: config.rich_presence,
        java: JavaConfig {
            java_8_path: java_8_path.to_string_lossy().to_string() + os_suffix,
            java_17_path: java_17_path.to_string_lossy().to_string() + os_suffix,
            java_21_path: java_21_path.to_string_lossy().to_string() + os_suffix,
        },
		completed_onboarding: config.completed_onboarding,
    };

    config::save_config(&new_config)?;

    Ok(())
}

pub fn get_java_from_config() -> Result<JavaConfig, Error> {
    let config = config::get_config()?;

    Ok(config.java)
}
