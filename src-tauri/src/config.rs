use std::{fs, path::PathBuf};

use anyhow::{anyhow, Error, Result};
use dirs::home_dir;
use serde::{Deserialize, Serialize};

use crate::auth;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub accounts: Vec<auth::account::Account>,
    pub rich_presence: bool,
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

fn get_config_path() -> Result<PathBuf, Error> {
    let folder_name = if cfg!(debug_assertions) {
        ".launcher-dev"
    } else {
        ".launcher"
    };

    home_dir()
        .map(|path| path.join(folder_name).join("config.json"))
        .ok_or_else(|| anyhow!("Failed to get home directory"))
}

pub fn config_file_exists() -> Result<bool, Error> {
    let config_path = get_config_path()?;
    Ok(config_path.exists())
}

pub fn create_default_config_file() -> Result<(), Error> {
    let default_account = auth::account::Account::default();

    let default_config = Config {
        accounts: vec![default_account],
        rich_presence: true,
    };

    create_config_file(&default_config)
}

pub fn get_config() -> Result<Config, Error> {
    let config_path = get_config_path()?;
    let config_data = fs::read_to_string(config_path)
        .map_err(|e| anyhow!("Failed to read config file: {}", e))?;
    let config: Config = serde_json::from_str(&config_data)
        .map_err(|e| anyhow!("Failed to parse config file: {}", e))?;
    Ok(config)
}

pub fn save_config(config: &Config) -> Result<(), Error> {
    create_config_file(config)
}
