use std::{path::PathBuf, process::Command};

use anyhow::Error;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct JavaTestInfo {
    pub valid: bool,
    pub version: String,
    pub distribution: String,
	#[serde(rename = "expectedVersion")]
    pub expected_version: u8,
	#[serde(rename = "versionMismatch")]
    version_mismatch: bool,
}

pub fn test_java(
    paths: (PathBuf, PathBuf, PathBuf),
) -> Result<(JavaTestInfo, JavaTestInfo, JavaTestInfo), Error> {
    let paths = [(paths.0, 8), (paths.1, 17), (paths.2, 21)];
    let mut results = (
        JavaTestInfo::default(),
        JavaTestInfo::default(),
        JavaTestInfo::default(),
    );

    for (i, (path, expected_version)) in paths.iter().enumerate() {
        let mut info = JavaTestInfo::default();
        info.expected_version = *expected_version;

        if !path.as_os_str().is_empty() {
            let output = Command::new(path).arg("-version").output()?;
            let output_str = String::from_utf8_lossy(&output.stderr);

            let version_regex = Regex::new(r#"version \"([\d._]+)\""#).unwrap();
            let distro_regex = Regex::new(r#"Runtime Environment ([\w-]+)"#).unwrap();

            if let Some(version_cap) = version_regex.captures(&output_str) {
                info.version = version_cap[1].to_string();

                let major_version = info
				.version
				.split('.')
				.next()
				.and_then(|v| v.parse::<u8>().ok())
				.unwrap();

				info.version_mismatch = major_version != *expected_version;
				info.valid = !info.version_mismatch;
            }

            if let Some(distro_cap) = distro_regex.captures(&output_str) {
                info.distribution = distro_cap[1].to_string();
            }
        }

        match i {
            0 => results.0 = info,
            1 => results.1 = info,
            2 => results.2 = info,
            _ => unreachable!(),
        }
    }

    Ok(results)
}
