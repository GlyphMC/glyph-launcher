use std::{path::PathBuf, process::Command};

use anyhow::Error;

pub fn test_java(paths: (PathBuf, PathBuf, PathBuf)) -> Result<Vec<String>, Error> {
	let paths = [(paths.0, 8), (paths.1, 17), (paths.2, 21)];
	let mut versions = Vec::new();

	for (path, _version) in paths.iter() {
		if !path.as_os_str().is_empty() {
			let command = Command::new(path)
				.arg("-version")
				.output()?;
			versions.push(String::from_utf8_lossy(&command.stderr).to_string());
		} else {
			versions.push("Not configured".to_string());
		}
	}

	Ok(versions)
}
