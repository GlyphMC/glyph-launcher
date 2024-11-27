use std::{fs, path::PathBuf};

use anyhow::{anyhow, Error, Result};
use tauri::{AppHandle, Emitter, State};

use crate::{config, instances::structs::Java, resources::version, AppState, Payload};

use super::structs::{Instance, InstanceConfig};

fn get_instances_path() -> Result<PathBuf, Error> {
    let config_dir = config::get_config_dir()?;
    Ok(config_dir.join("instances.json"))
}

pub fn create_default_instances_file() -> Result<(), Error> {
    let instance_config = InstanceConfig { instances: vec![] };
    let instances_path = get_instances_path()?;

    if let Some(parent) = instances_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| anyhow!("Failed to create instances directory: {}", e))?;
    }

    let instances_data = serde_json::to_string_pretty(&instance_config)?;
    fs::write(instances_path, instances_data)
        .map_err(|e| anyhow!("Failed to write instances file: {}", e))?;

    Ok(())
}

pub fn get_instances() -> Result<InstanceConfig, Error> {
    let instances_path = get_instances_path()?;
    let instances_data = fs::read_to_string(instances_path)
        .map_err(|e| anyhow!("Failed to read instances file: {}", e))?;
    let instance_config = serde_json::from_str::<InstanceConfig>(&instances_data)
        .map_err(|e| anyhow!("Failed to parse instances file: {}", e))?;

    Ok(instance_config)
}

pub fn get_instance(slug: String) -> Result<Instance, Error> {
    let instance_config = get_instances()?;

    instance_config
        .instances
        .into_iter()
        .find(|inst| inst.slug == slug)
        .ok_or_else(|| anyhow!("Instance with slug '{}' not found", slug))
}

pub async fn create_instance(
    state: State<'_, AppState>,
    handle: AppHandle,
    mut instance: Instance,
    url: String,
) -> Result<(), Error> {
    let manifest = version::get_version_manifest(state, url).await?;
    let java_version = manifest.java_version.major_version;
    let config = config::get_config()?;
    let java_config = config.java;
    let instance_config = get_instances()?;
    let mut instances = instance_config.instances;

    match java_version {
        8 => {
            instance.java = Java {
                path: java_config.java_8_path,
                args: vec![],
            };
        }
        17 => {
            instance.java = Java {
                path: java_config.java_17_path,
                args: vec![],
            };
        }
        21 => {
            instance.java = Java {
                path: java_config.java_21_path,
                args: vec![],
            };
        }
        _ => return Err(anyhow!("Unsupported Java version: {}", java_version)),
    }

    instances.push(instance);
    let new_instance_config = InstanceConfig { instances };
    let instances_path = get_instances_path()?;
    let instances_data = serde_json::to_string_pretty(&new_instance_config)?;
    fs::write(instances_path, instances_data)
        .map_err(|e| anyhow!("Failed to write instances file: {}", e))?;

    handle.emit(
        "instance-list-updated",
        Payload {
            message: "Instance created",
        },
    )?;

    Ok(())
}

pub fn delete_instance(handle: AppHandle, slug: String) -> Result<(), Error> {
    let instance_config = get_instances()?;
    let instances = instance_config.instances;
    let new_instances = instances
        .into_iter()
        .filter(|inst| inst.slug != slug)
        .collect();
    let new_instance_config = InstanceConfig {
        instances: new_instances,
    };
    let instances_path = get_instances_path()?;
    let instances_data = serde_json::to_string_pretty(&new_instance_config).unwrap();
    fs::write(instances_path, instances_data)?;

	handle.emit("instance-list-updated", Payload {
		message: "Instance deleted"
	})?;

	Ok(())
}
