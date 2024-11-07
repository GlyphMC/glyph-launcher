use std::{
    fs::{self, File},
    io::{self, BufReader},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Error, Ok, Result};
use flate2::bufread::GzDecoder;
use log::info;
use tar::Archive;
use tauri::{async_runtime::spawn, AppHandle, Emitter};
use zip::ZipArchive;

use crate::{java::structs::Progress, Payload};

pub async fn extract_java(
    handle: AppHandle,
    paths: (PathBuf, PathBuf, PathBuf),
) -> Result<(PathBuf, PathBuf, PathBuf), Error> {
    let (java_8_archive_path, java_17_archive_path, java_21_archive_path) = paths;

    handle
        .emit(
            "extract-started",
            Payload {
                message: "Extract started",
            },
        )
        .unwrap();

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

    handle
        .emit(
            "extract-finished",
            Payload {
                message: "Extract finished",
            },
        )
        .unwrap();

    Ok((output_dir_8, output_dir_17, output_dir_21))
}

async fn extract_java_archive(
    handle: AppHandle,
    version: &str,
    file_path: PathBuf,
) -> Result<PathBuf, Error> {
    let output_dir = file_path.with_extension("");

    let extract_dir = if cfg!(windows) {
        extract_zip(&handle, version, &file_path, &output_dir)?
    } else if cfg!(unix) {
        extract_tar_gz(&handle, version, &file_path, &output_dir)?
    } else {
        return Err(anyhow!("Unsupported platform").into());
    };

    Ok(extract_dir)
}

fn extract_zip(
    handle: &AppHandle,
    version: &str,
    archive_path: &Path,
    output_dir: &Path,
) -> Result<PathBuf, Error> {
    info!("Extracting ZIP archive: {}", archive_path.to_string_lossy());

    let file = File::open(archive_path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut total_size = 0;

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        total_size += file.size();
    }

    let file = File::open(archive_path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut extracted_size = 0;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_path = Path::new(file.name());

        let relative_path = file_path.iter().skip(1).collect::<PathBuf>();

        let output_path = output_dir.join(relative_path);
        extracted_size += file.size();

        let percentage = (extracted_size as f64 / total_size as f64) * 100.0;
        let progress = Progress { percentage };

        handle
            .emit(&format!("java-extract-progress-{}", version), progress)
            .unwrap();

        if file.name().ends_with("/") {
            fs::create_dir_all(output_path)?;
        } else {
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut output_file = File::create(&output_path)?;
            io::copy(&mut file, &mut output_file)?;
        }
    }

    fs::remove_file(archive_path)?;

    info!("Extracted ZIP archive to: {}", output_dir.to_string_lossy());

    Ok(output_dir.to_path_buf())
}

fn extract_tar_gz(
    handle: &AppHandle,
    version: &str,
    archive_path: &Path,
    output_dir: &Path,
) -> Result<PathBuf, Error> {
    info!(
        "Extracting TAR.GZ archive: {}",
        archive_path.to_string_lossy()
    );

    let file = File::open(archive_path)?;
    let buf_reader = BufReader::new(file);
    let decompressed = GzDecoder::new(buf_reader);
    let mut archive = Archive::new(decompressed);

    let mut total_size = 0;
    for entry in archive.entries()? {
        let entry = entry?;
        total_size += entry.header().size()?;
    }

    let file = File::open(archive_path)?;
    let buf_reader = BufReader::new(file);
    let decompressed = GzDecoder::new(buf_reader);
    let mut archive = Archive::new(decompressed);
    let mut extracted_size = 0;

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        let file_size = entry.header().size()?;
        extracted_size += file_size;

        let relative_path = path.iter().skip(1).collect::<PathBuf>();

        let output_path = output_dir.join(relative_path);
        let percentage = (extracted_size as f64 / total_size as f64) * 100.0;
        let progress = Progress { percentage };

        handle
            .emit(&format!("java-extract-progress-{}", version), progress)
            .unwrap();

        entry.unpack(&output_path)?;
    }

    let mut final_output_dir = output_dir.to_path_buf();
    if let Some(output_str) = output_dir.to_str() {
        if output_str.ends_with(".tar") {
            final_output_dir = PathBuf::from(&output_str[..output_str.len() - 4]);
            fs::rename(output_dir, &final_output_dir)?;
        }
    }

    fs::remove_file(archive_path)?;

    info!(
        "Extracted TAR.GZ archive to: {}",
        final_output_dir.to_string_lossy()
    );

    Ok(final_output_dir.to_path_buf())
}
