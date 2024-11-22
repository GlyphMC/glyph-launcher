use std::{fs, path::PathBuf};

use anyhow::{anyhow, Error, Result};

use super::structs::{Instance, InstanceConfig};

fn get_instances_path() -> Result<PathBuf, Error> {
    let config_dir = crate::config::get_config_dir()?;
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
	let instance_config: InstanceConfig = serde_json::from_str(&instances_data)
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

pub fn create_instance(mut instance: Instance, url: String) {
	// the instance we receive does not have java set up
	// we need to set it up here

	// if version is from 1.8 to 1.16.4, use Java 8
	// if version is from 1.16.5 to 1.17.1, use Java
	//

	

}
