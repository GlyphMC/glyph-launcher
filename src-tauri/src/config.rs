use std::{fs, path::PathBuf};

use anyhow::{anyhow, Error, Result};
use dirs::config_dir;
use serde::{Deserialize, Serialize};

use crate::{auth::account::Account, java::structs::JavaConfig};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub accounts: Vec<Account>,
    pub rich_presence: bool,
    pub java: JavaConfig,
}

fn create_config_file(config: &Config) -> Result<(), Error> {
    let config_path = get_config_path()?;

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| anyhow!("Failed to create config directory: {}", e))?;
    }

    let config_data = serde_json::to_string_pretty(config)?;
    fs::write(config_path, config_data)
        .map_err(|e| anyhow!("Failed to write config file: {}", e))?;
    Ok(())
}

pub fn get_config_dir() -> Result<PathBuf, Error> {
    let folder_name = if cfg!(debug_assertions) {
        ".glyph-launcher-dev"
    } else {
        ".glyph-launcher"
    };

    config_dir()
        .map(|path| path.join(folder_name))
        .ok_or_else(|| anyhow!("Failed to get config directory"))
}

fn get_config_path() -> Result<PathBuf, Error> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join("config.json"))
}

pub fn config_file_exists() -> Result<bool, Error> {
    let config_path = get_config_path()?;
    Ok(config_path.exists())
}

pub fn create_default_config_file() -> Result<(), Error> {
    let default_account = Account::default();

    let default_config = Config {
        accounts: vec![default_account],
        rich_presence: true,
        java: JavaConfig::default(),
    };

    create_config_file(&default_config)
}

pub fn get_config() -> Result<Config, Error> {
    let config_path = get_config_path()?;
    let config_data = fs::read_to_string(config_path)
        .map_err(|e| anyhow!("Failed to read config file: {}", e))?;
    let config = serde_json::from_str::<Config>(&config_data)
        .map_err(|e| anyhow!("Failed to parse config file: {}", e))?;
    Ok(config)
}

pub fn save_config(config: &Config) -> Result<(), Error> {
    create_config_file(config)
}
