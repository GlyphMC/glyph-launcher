use std::{env, path::PathBuf};

use anyhow::{Error, Result, anyhow};
use futures::try_join;
use log::{debug, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{AppHandle, State};
use tauri_specta::Event;
use tokio::{fs::File, io::AsyncWriteExt, time::Instant};

use crate::{AppState, config, java::structs::JavaInfo};

const BASE_URL: &str = "https://api.azul.com/metadata/v1/zulu/packages/";

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct JavaDownloadStartedEvent(String);

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct JavaDownloadProgressEvent {
    pub version: i8,
    pub percentage: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct JavaDownloadFinishedEvent {
    pub paths: Vec<String>,
}

pub async fn download_java(
    state: &State<'_, AppState>,
    handle: AppHandle,
) -> Result<(PathBuf, PathBuf, PathBuf), Error> {
    JavaDownloadStartedEvent("Download started".into()).emit(&handle)?;

    let client = state.client.lock().await;
    let config_dir = config::get_config_dir()?;
    let runtime_dir = config_dir.join("runtime");

    debug!("Clearing runtime directory: {}", runtime_dir.display());
    if runtime_dir.exists() {
        tokio::fs::remove_dir_all(&runtime_dir).await?;
    }
    tokio::fs::create_dir_all(&runtime_dir).await?;

    let (java_8_archive_path, java_17_archive_path, java_21_archive_path) = try_join!(
        download_java_version(8, &client, &handle),
        download_java_version(17, &client, &handle),
        download_java_version(21, &client, &handle)
    )?;

    JavaDownloadFinishedEvent {
        paths: vec![
            java_8_archive_path.to_string_lossy().to_string(),
            java_17_archive_path.to_string_lossy().to_string(),
            java_21_archive_path.to_string_lossy().to_string(),
        ],
    }
    .emit(&handle)?;

    Ok((
        java_8_archive_path,
        java_17_archive_path,
        java_21_archive_path,
    ))
}

async fn download_java_version(
    version: i8,
    client: &Client,
    handle: &AppHandle,
) -> Result<PathBuf, Error> {
    let config_dir = config::get_config_dir()?;
    let runtime_dir = config_dir.join("runtime");
    tokio::fs::create_dir_all(&runtime_dir).await?;

    let os = match env::consts::OS {
        "windows" => "windows",
        "macos" => "macos",
        "linux" => "linux",
        _ => return Err(anyhow!("Unsupported OS")),
    };

    let arch = match env::consts::ARCH {
        "x86_64" => "amd64",
        "aarch64" => "aarch64",
        _ => return Err(anyhow!("Unsupported architecture")),
    };

    let java_version = version.to_string();
    let query_params = [
        ("java_version", java_version.as_str()),
        ("os", os),
        ("arch", arch),
        ("archive_type", "zip"),
        ("java_package_type", "jdk"),
        ("javafx_bundled", "false"),
        ("crac_supported", "false"),
        ("latest", "true"),
        ("release_status", "ga"),
    ];

    let response = client.get(BASE_URL).query(&query_params).send().await?;

    if response.status().is_success() {
        let json = response.json::<Vec<JavaInfo>>().await?;
        debug!("{:?}", json);

        if let Some(java_info) = json.first() {
            let download_url = &java_info.download_url;
            info!("Downloading Java {} from: {}", version, download_url);

            let mut zip_response = client.get(download_url).send().await?;

            return if zip_response.status().is_success() {
                let file_name = download_url.split("/").last().unwrap();
                let file_path = runtime_dir.join(file_name);
                let mut file = File::create(runtime_dir.join(file_name)).await?;

                let total_size = zip_response
                    .content_length()
                    .ok_or(anyhow!("Failed to get zip size"))?;
                let mut downloaded_size = 0;
                let mut last_emit_time = Instant::now();

                while let Ok(bytes_read) = zip_response.chunk().await {
                    let bytes = match bytes_read {
                        Some(bytes) => bytes,
                        None => break,
                    };
                    file.write_all(&bytes).await?;
                    downloaded_size += bytes.len() as u64;

                    if last_emit_time.elapsed().as_millis() >= 250 {
                        let percentage = (downloaded_size as f64 / total_size as f64) * 100.0;

                        JavaDownloadProgressEvent {
                            version,
                            percentage,
                        }
                        .emit(handle)?;
                        last_emit_time = Instant::now();
                    }

                    if downloaded_size == total_size {
                        break;
                    }
                }

                JavaDownloadProgressEvent {
                    version,
                    percentage: 100.0,
                }
                .emit(handle)?;

                info!(
                    "Downloaded Java {} to: {}",
                    version,
                    runtime_dir.join(file_name).to_string_lossy()
                );

                Ok(file_path)
            } else {
                Err(anyhow!(
                    "Failed to download ZIP file, status: {}",
                    zip_response.status()
                ))
            };
        }
    } else {
        info!("Request failed with status: {}", response.status());
    }

    Err(anyhow!("Failed to download Java {}", version))
}
