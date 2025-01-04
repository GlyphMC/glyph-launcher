use std::{path::PathBuf, process::Command};

use anyhow::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct JavaTestInfo {
    pub valid: bool,
    pub version: u8,
    pub vendor: String,
    #[serde(rename = "expectedVersion")]
    pub expected_version: u8,
    #[serde(rename = "versionMismatch")]
    version_mismatch: bool,
}

fn extract_major_version(version: &str) -> Option<u8> {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.is_empty() {
        return None;
    }

    let version_str: &str = if parts[0] == "1" && parts.len() > 1 {
        parts[1]
    } else {
        parts[0]
    };

    version_str.parse::<u8>().ok()
}

pub fn test_java(
    paths: (PathBuf, PathBuf, PathBuf),
) -> Result<(JavaTestInfo, JavaTestInfo, JavaTestInfo), Error> {
    let mut paths = [(paths.0, 8), (paths.1, 17), (paths.2, 21)];
    let mut results = (
        JavaTestInfo::default(),
        JavaTestInfo::default(),
        JavaTestInfo::default(),
    );

    for (i, (ref mut path, expected_version)) in paths.iter_mut().enumerate() {
        let mut info = JavaTestInfo::default();
        info.expected_version = *expected_version;

        if !path.as_os_str().is_empty() {
            if path.ends_with("javaw.exe") {
                *path = path.with_file_name("java.exe");
            }
            let output = Command::new(path)
                .args(["-XshowSettings:properties", "-version"])
                .output()?;
            let output_str = String::from_utf8_lossy(&output.stderr);

            for line in output_str.lines() {
                if let Some(value) = line.trim().strip_prefix("java.version = ") {
                    if let Some(version_num) = extract_major_version(&value) {
                        info.version = version_num;
                        info.version_mismatch = version_num != info.expected_version;
                    }
                }
                if let Some(value) = line.trim().strip_prefix("java.vendor = ") {
                    info.vendor = value.to_string();
                }
            }

            info.valid = info.version != 0 && !info.vendor.is_empty();
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
