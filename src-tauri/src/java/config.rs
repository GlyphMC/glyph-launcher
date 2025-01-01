use std::path::PathBuf;

use anyhow::{Error, Result};

use crate::config::{self, Config};

use super::structs::JavaConfig;

fn handle_path(path: PathBuf) -> PathBuf {
    if cfg!(target_os = "windows") && path.ends_with("java.exe") {
        PathBuf::from(path.to_string_lossy().replace("java.exe", "javaw.exe"))
    } else {
        path
    }
}

pub fn save_java_to_config(paths: (PathBuf, PathBuf, PathBuf)) -> Result<(), Error> {
    let (java_8_path, java_17_path, java_21_path) = paths;
    let config = config::get_config()?;

    println!("java_8_path: {:?}", java_8_path);
    println!("java_17_path: {:?}", java_17_path);
    println!("java_21_path: {:?}", java_21_path);

    let processed_paths = (
        if !java_8_path.as_os_str().is_empty() {
            handle_path(java_8_path)
        } else {
            java_8_path
        },
        if !java_17_path.as_os_str().is_empty() {
            handle_path(java_17_path)
        } else {
            java_17_path
        },
        if !java_21_path.as_os_str().is_empty() {
            handle_path(java_21_path)
        } else {
            java_21_path
        },
    );

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
