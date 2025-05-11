use std::{fs, path::PathBuf};

use anyhow::{Error, Result, anyhow};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, State};

use crate::{AppState, Payload, config, resources::version};

#[derive(Serialize, Deserialize, Debug)]
pub struct InstanceConfig {
    pub instances: Vec<Instance>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Instance {
    pub slug: String,
    pub name: String,
    pub game: Game,
    pub java: Java,
    pub settings: Settings,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    pub version: String,
    pub modloader: Modloader,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Modloader {
    pub loader: String,
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Java {
    pub path: String,
    pub args: Vec<String>,
    pub version: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub has_launched: bool,
    pub rich_presence: bool,
    pub window_width: u32,
    pub window_height: u32,
    pub maximized: bool,
    #[serde(default)]
    pub time_played: u64,
    #[serde(default)]
    pub last_played: Option<DateTime<Utc>>,
}

impl InstanceConfig {
    fn get_instance_config_path() -> Result<PathBuf, Error> {
        let config_dir = config::get_config_dir()?;
        Ok(config_dir.join("instances.json"))
    }

    pub fn create_default_file() -> Result<(), Error> {
        let instance_config = InstanceConfig { instances: vec![] };
        let instance_config_path = Self::get_instance_config_path()?;

        if let Some(parent) = instance_config_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| anyhow!("Failed to create instances config directory: {}", e))?;
        }

        let instances_data = serde_json::to_string_pretty(&instance_config)?;
        fs::write(instance_config_path, instances_data)
            .map_err(|e| anyhow!("Failed to write instances config file: {}", e))?;

        let instances_path = config::get_config_dir()?.join("instances");
        fs::create_dir_all(instances_path)?;

        Ok(())
    }

    pub fn read_from_file() -> Result<Self, Error> {
        let instance_config_path = Self::get_instance_config_path()?;
        let instances_data = fs::read_to_string(instance_config_path)
            .map_err(|e| anyhow!("Failed to read instances config file: {}", e))?;
        let instance_config = serde_json::from_str::<InstanceConfig>(&instances_data)
            .map_err(|e| anyhow!("Failed to parse instances config file: {}", e))?;

        Ok(instance_config)
    }

    pub fn write_to_file(&self) -> Result<(), Error> {
        let instance_config_path = Self::get_instance_config_path()?;
        let instances_data = serde_json::to_string_pretty(self)?;
        fs::write(instance_config_path, instances_data)
            .map_err(|e| anyhow!("Failed to write instances config file: {}", e))?;

        Ok(())
    }

    pub fn get_instances(&self) -> Vec<Instance> {
        self.instances.clone()
    }

    pub fn get_instance(&self, slug: &str) -> Option<Instance> {
        self.instances.iter().find(|i| i.slug == slug).cloned()
    }

    pub async fn add_instance(
        &mut self,
        state: &State<'_, AppState>,
        handle: AppHandle,
        mut instance: Instance,
    ) -> Result<(), Error> {
        let manifest = version::get_version_manifest(state, &instance.game.url).await?;
        let java_version = manifest.java_version.major_version;
        let config = config::get_config()?;
        let java_config = config.java;

        match java_version {
            8 => {
                instance.java = Java {
                    path: java_config.java_8_path,
                    args: vec![],
                    version: 8,
                };
            }
            17 => {
                instance.java = Java {
                    path: java_config.java_17_path,
                    args: vec![],
                    version: 17,
                };
            }
            21 => {
                instance.java = Java {
                    path: java_config.java_21_path,
                    args: vec![],
                    version: 21,
                };
            }
            _ => return Err(anyhow!("Unsupported Java version: {}", java_version)),
        }

        self.instances.push(instance);
        self.write_to_file()?;

        handle.emit(
            "instance-list-updated",
            Payload {
                message: "Instance added",
            },
        )?;

        Ok(())
    }

    pub fn update_instance(&mut self, handle: AppHandle, instance: Instance) -> Result<(), Error> {
        if let Some(i) = self.instances.iter_mut().find(|i| i.slug == instance.slug) {
            *i = instance;
        }
        self.write_to_file()?;

        handle.emit(
            "instance-list-updated",
            Payload {
                message: "Instance updated",
            },
        )?;

        Ok(())
    }

    pub fn delete_instance(&mut self, handle: AppHandle, slug: &str) -> Result<(), Error> {
        self.instances.retain(|i| i.slug != slug);
        self.write_to_file()?;

        let instances_path = config::get_config_dir()?.join("instances");
        let instance_dir = instances_path.join(slug);
        if instance_dir.exists() {
            fs::remove_dir_all(instance_dir)?;
        }

        handle.emit(
            "instance-list-updated",
            Payload {
                message: "Instance deleted",
            },
        )?;

        Ok(())
    }
}
