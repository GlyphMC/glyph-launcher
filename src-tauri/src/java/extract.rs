use std::path::{Component, PathBuf};

use anyhow::{Error, Ok, Result, anyhow};
use async_zip::tokio::read::seek::ZipFileReader;
use log::info;
use tauri::{AppHandle, Emitter, async_runtime::spawn};
use tokio::{
    fs::{self, File},
    io::{self, BufReader},
    time::Instant,
};
use tokio_util::compat::FuturesAsyncReadCompatExt;

use crate::{Payload, java::structs::Progress};

pub async fn extract_java(
    handle: AppHandle,
    paths: (PathBuf, PathBuf, PathBuf),
) -> Result<(PathBuf, PathBuf, PathBuf), Error> {
    let (java_8_archive_path, java_17_archive_path, java_21_archive_path) = paths;

    handle.emit(
        "extract-started",
        Payload {
            message: "Extract started",
        },
    )?;

    let handle_8 = {
        let handle = handle.clone();
        let java_8_path = java_8_archive_path.clone();
        spawn(async move {
            let output_dir_8 = extract_java_archive(handle, "8", java_8_path).await?;
            Ok(output_dir_8)
        })
    };

    let handle_17 = {
        let handle = handle.clone();
        let java_17_path = java_17_archive_path.clone();
        spawn(async move {
            let output_dir_17 = extract_java_archive(handle, "17", java_17_path).await?;
            Ok(output_dir_17)
        })
    };

    let handle_21 = {
        let handle = handle.clone();
        let java_21_path = java_21_archive_path.clone();
        spawn(async move {
            let output_dir_21 = extract_java_archive(handle, "21", java_21_path).await?;
            Ok(output_dir_21)
        })
    };

    let output_dir_8 = handle_8.await??;
    let output_dir_17 = handle_17.await??;
    let output_dir_21 = handle_21.await??;

    handle.emit(
        "extract-finished",
        Payload {
            message: "Extract finished",
        },
    )?;

    Ok((output_dir_8, output_dir_17, output_dir_21))
}

async fn extract_java_archive(
    handle: AppHandle,
    version: &str,
    archive_path: PathBuf,
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
    let num_entries = entries.len();

    for index in 0..num_entries {
        let entry = &entries[index];
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

        if last_emit_time.elapsed().as_secs() >= 1 {
            let percentage = (extracted_size as f64 / total_size as f64) * 100.0;
            let progress = Progress { percentage };

            handle.emit(&format!("java-extract-progress-{}", version), progress)?;
            last_emit_time = Instant::now();
        }
    }

    handle.emit(
        &format!("java-extract-progress-{}", version),
        Progress { percentage: 100.0 },
    )?;

    fs::remove_file(&archive_path).await?;

    info!("Extracted ZIP archive to: {}", output_dir.to_string_lossy());

    Ok(output_dir.to_path_buf())
}
