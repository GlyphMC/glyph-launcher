use std::path::PathBuf;

use anyhow::{Error, Result};
use log::info;

use crate::config::{self, Config};

use super::structs::JavaConfig;

fn handle_path(path: PathBuf) -> PathBuf {
    if cfg!(target_os = "windows") && path.ends_with("java.exe") {
        path.with_file_name("javaw.exe")
    } else {
        path
    }
}

fn get_java_path(path: PathBuf, automatic: bool) -> PathBuf {
    if !automatic {
        return path;
    }

    if cfg!(target_os = "windows") {
        path.join("bin").join("javaw.exe")
    } else {
        path.join("bin").join("java")
    }
}

pub fn save_java_to_config(
    paths: (PathBuf, PathBuf, PathBuf),
    automatic: bool,
) -> Result<(), Error> {
    let (java_8_path, java_17_path, java_21_path) = paths;

    let processed_paths = (
        handle_path(get_java_path(java_8_path, automatic)),
        handle_path(get_java_path(java_17_path, automatic)),
        handle_path(get_java_path(java_21_path, automatic)),
    );

    info!("Java 8 path: {:?}", processed_paths.0);
    info!("Java 17 path: {:?}", processed_paths.1);
    info!("Java 21 path: {:?}", processed_paths.2);

    let config = config::get_config()?;
    let new_config = Config {
        accounts: config.accounts,
        rich_presence: config.rich_presence,
        java: JavaConfig {
            java_8_path: processed_paths.0.to_string_lossy().to_string(),
            java_17_path: processed_paths.1.to_string_lossy().to_string(),
            java_21_path: processed_paths.2.to_string_lossy().to_string(),
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
