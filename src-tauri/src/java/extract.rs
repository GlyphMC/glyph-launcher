use std::path::{Component, PathBuf};

use anyhow::{Error, Ok, Result, anyhow};
use async_zip::tokio::read::seek::ZipFileReader;
use futures::future::try_join_all;
use log::info;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::AppHandle;
use tauri_specta::Event;
use tokio::{
    fs::{self, File},
    io::{self, BufReader},
    time::Instant,
};
use tokio_util::compat::FuturesAsyncReadCompatExt;

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct JavaExtractStartedEvent(String);

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct JavaExtractProgressEvent {
    pub version: i8,
    pub percentage: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct JavaExtractFinishedEvent {
    pub paths: Vec<String>,
}

pub async fn extract_java(
    handle: AppHandle,
    paths: Vec<PathBuf>,
    versions: Vec<i8>,
) -> Result<Vec<PathBuf>, Error> {
    if paths.len() != versions.len() {
        return Err(anyhow!("Number of paths and versions must match"));
    }

    JavaExtractStartedEvent("Extract started".to_string()).emit(&handle)?;

    let mut extract_futures = Vec::new();
    for (i, path) in paths.iter().enumerate() {
        let version = versions[i];
        extract_futures.push(extract_java_archive(&handle, version, path));
    }

    let output_dirs = try_join_all(extract_futures).await?;

    JavaExtractFinishedEvent {
        paths: output_dirs
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect(),
    }
    .emit(&handle)?;

    Ok(output_dirs)
}

async fn extract_java_archive(
    handle: &AppHandle,
    version: i8,
    archive_path: &PathBuf,
) -> Result<PathBuf, Error> {
    info!("Extracting ZIP archive: {}", archive_path.to_string_lossy());

    let output_dir = archive_path.with_extension("");

    let file = File::open(&archive_path).await?;
    let mut archive = ZipFileReader::with_tokio(BufReader::new(file)).await?;
    let total_size: u64 = archive
        .file()
        .entries()
        .iter()
        .map(|e| e.compressed_size())
        .sum();

    let mut extracted_size = 0;
    let mut last_emit_time = Instant::now();
    let entries = archive.file().entries().to_vec();

    for (index, entry) in entries.iter().enumerate() {
        let file_name = entry.filename().as_str()?;
        let file_path = PathBuf::from(file_name);

        let stripped_path = file_path.components().skip(1).collect::<PathBuf>();

        if stripped_path
            .components()
            .any(|c| matches!(c, Component::ParentDir))
        {
            return Err(anyhow!("Invalid zip path detected"));
        }

        let output_path = output_dir.join(stripped_path);

        if entry.dir()? {
            fs::create_dir_all(&output_path).await?;
        } else {
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent).await?;
            }
            let mut output_file = File::create(&output_path).await?;

            let mut entry_reader = archive.reader_with_entry(index).await?.compat();
            io::copy(&mut entry_reader, &mut output_file).await?;
        }

        extracted_size += entry.uncompressed_size();

        if last_emit_time.elapsed().as_millis() >= 250 {
            let percentage = (extracted_size as f64 / total_size as f64) * 100.0;

            JavaExtractProgressEvent {
                version,
                percentage,
            }
            .emit(handle)?;
            last_emit_time = Instant::now();
        }
    }

    JavaExtractProgressEvent {
        version,
        percentage: 100.0,
    }
    .emit(handle)?;

    fs::remove_file(&archive_path).await?;

    info!("Extracted ZIP archive to: {}", output_dir.to_string_lossy());

    Ok(output_dir.to_path_buf())
}
