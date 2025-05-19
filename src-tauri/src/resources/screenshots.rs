use std::{ffi::OsStr, fs, io::Read, path::PathBuf};

use anyhow::{Context, Error, Result};
use base64::Engine;
use log::{error, info};
use notify::{RecursiveMode, Watcher, event::EventKind};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::sync::Mutex;
use tauri::AppHandle;
use tauri_specta::Event;
use tokio::sync::oneshot::{self, Sender};

use crate::config;

#[derive(Serialize, Deserialize, Debug, Clone, Type)]
pub struct Screenshot {
    pub path: PathBuf,
    pub name: String,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct ScreenshotEvent(String);

fn get_screenshots_dir(slug: String) -> Result<PathBuf, Error> {
    let config_dir = config::get_config_dir()?;
    let instance_dir = config_dir.join("instances").join(slug);
    let screenshots_dir = instance_dir.join("screenshots");
    if !screenshots_dir.exists() {
        fs::create_dir_all(&screenshots_dir)?;
    }

    Ok(screenshots_dir)
}

pub fn get_screenshots(slug: String) -> Result<Vec<Screenshot>, Error> {
    let screenshots_dir = get_screenshots_dir(slug)?;
    let mut screenshots = Vec::new();

    for entry_result in fs::read_dir(&screenshots_dir).with_context(|| {
        format!(
            "Failed to read screenshots directory: {:?}",
            screenshots_dir
        )
    })? {
        let entry = entry_result.with_context(|| "Failed to read directory entry")?;
        let path = entry.path();

        if path.is_file() {
            match path.extension().and_then(OsStr::to_str) {
                Some(ext) if ext.eq_ignore_ascii_case("png") => {
                    let name = path
                        .file_name()
                        .ok_or_else(|| {
                            Error::msg(format!("Could not get file name from path {:?}", path))
                        })?
                        .to_string_lossy()
                        .to_string();

                    let mut file_content = Vec::new();
                    fs::File::open(&path)
                        .with_context(|| format!("Failed to open screenshot file: {:?}", path))?
                        .read_to_end(&mut file_content)
                        .with_context(|| format!("Failed to read screenshot file: {:?}", path))?;

                    let encoded_data =
                        base64::engine::general_purpose::STANDARD.encode(&file_content);
                    let data = format!("data:image/png;base64,{}", encoded_data);

                    screenshots.push(Screenshot {
                        path: path.clone(),
                        name,
                        data,
                    });
                }
                _ => {
                    continue;
                }
            }
        }
    }

    screenshots.sort_by(|a, b| b.path.cmp(&a.path));

    Ok(screenshots)
}

static SCREENSHOT_WATCHER: Lazy<Mutex<Option<Sender<()>>>> = Lazy::new(|| Mutex::new(None));

pub fn watch_screenshots(handle: AppHandle, slug: &str) -> Result<(), Error> {
    let screenshots_dir_path = get_screenshots_dir(slug.to_string())?;
    let slug = slug.to_string();

    if let Some(tx) = SCREENSHOT_WATCHER.lock().unwrap().take() {
        let _ = tx.send(());
        info!("Stopping existing screenshot watcher");
    }

    let (tx, rx) = oneshot::channel::<()>();
    SCREENSHOT_WATCHER.lock().unwrap().replace(tx);

    tokio::spawn(async move {
        let handle = handle.clone();
        let event_handler = move |res: Result<notify::Event, notify::Error>| match res {
            Ok(event) => {
                if let EventKind::Create(_) = event.kind {
                    for path_buf in event.paths {
                        if path_buf.is_file()
                            && path_buf
                                .extension()
                                .and_then(OsStr::to_str)
                                .is_some_and(|ext| ext.eq_ignore_ascii_case("png"))
                        {
                            if let Err(e) =
                                ScreenshotEvent("New screenshot added".into()).emit(&handle)
                            {
                                error!("Failed to emit ScreenshotEvent: {}", e);
                            }
                        }
                    }
                }
            }
            Err(e) => error!("Screenshot watch error: {:?}", e),
        };

        let mut watcher = match notify::recommended_watcher(event_handler) {
            Ok(w) => w,
            Err(e) => {
                error!(
                    "Failed to create screenshot watcher for slug {}: {}",
                    slug, e
                );
                return;
            }
        };

        if let Err(e) = watcher.watch(&screenshots_dir_path, RecursiveMode::NonRecursive) {
            error!(
                "Failed to watch path {:?} for slug {}: {}",
                screenshots_dir_path, slug, e
            );
            return;
        }

        info!(
            "Screenshot watcher started for slug: {} on dir {:?}",
            slug, screenshots_dir_path
        );

        let _ = rx.await;
        info!("Screenshot watcher task for slug {} is terminating.", slug);
        SCREENSHOT_WATCHER.lock().unwrap().take();
    });

    Ok(())
}

pub fn stop_watching_screenshots() {
    if let Some(tx) = SCREENSHOT_WATCHER.lock().unwrap().take() {
        let _ = tx.send(());
    }
}
