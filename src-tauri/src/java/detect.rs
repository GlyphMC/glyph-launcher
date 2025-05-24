use std::{path::PathBuf, process::Command};

use anyhow::{Context, Result, anyhow};

#[cfg(target_os = "windows")]
const COMMAND: &str = "where";
#[cfg(not(target_os = "windows"))]
const COMMAND: &str = "which";

pub type JavaDetectionResult = (Option<PathBuf>, Option<PathBuf>, Option<PathBuf>);

pub fn detect_java() -> Result<JavaDetectionResult> {
    let mut java8 = None;
    let mut java17 = None;
    let mut java21 = None;

    let mut command_builder = Command::new(COMMAND);

    if cfg!(not(target_os = "windows")) {
        command_builder.arg("-a");
    }
    command_builder.arg("java");

    let output = command_builder
        .output()
        .with_context(|| format!("Failed to run `{}` command", COMMAND))?;

    if !output.status.success() {
        return Ok((None, None, None));
    }

    let paths = String::from_utf8_lossy(&output.stdout);
    for line in paths.lines() {
        let path = PathBuf::from(line.trim());
        if !path.exists() {
            continue;
        }

        if let Ok(version) = get_java_version(&path) {
            match version.as_str() {
                v if v.starts_with("1.8") || v.starts_with("8") => {
                    java8.get_or_insert(path.clone());
                }
                v if v.starts_with("17") => {
                    java17.get_or_insert(path.clone());
                }
                v if v.starts_with("21") => {
                    java21.get_or_insert(path.clone());
                }
                _ => {}
            };
        }
    }

    Ok((java8, java17, java21))
}

fn get_java_version(path: &PathBuf) -> Result<String> {
    let output = Command::new(path)
        .arg("-version")
        .output()
        .with_context(|| format!("Failed to run `-version` on {:?}", path))?;

    if !output.status.success() {
        return Err(anyhow!("`java -version` failed"));
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    for line in stderr.lines() {
        if line.contains("version") {
            if let Some(ver) = line.split('"').nth(1) {
                return Ok(ver.to_string());
            }
        }
    }

    Err(anyhow!("Could not parse Java version"))
}
