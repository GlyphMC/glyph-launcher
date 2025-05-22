use std::{
    fs::{self},
    io::{Cursor, Read},
    path::PathBuf,
};

use anyhow::{Context, Error, Result};
use base64::Engine;
use chrono::{DateTime, TimeZone, Utc};
use fastnbt::from_reader;
use flate2::read::GzDecoder;
use log::warn;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::State;

use crate::{AppState, config};

#[derive(Serialize, Deserialize, Debug, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct World {
    pub path: PathBuf,
    pub folder_name: String,
    pub level_name: String,
    #[specta(type = String)]
    pub last_played: Option<DateTime<Utc>>,
    pub icon: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct LevelDat {
    data: LevelData,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct LevelData {
    level_name: String,
    last_played: Option<i64>,
}

fn get_worlds_dir(slug: String) -> Result<PathBuf, Error> {
    let config_dir = config::get_config_dir()?;
    let instance_dir = config_dir.join("instances").join(slug);
    let worlds_dir = instance_dir.join("saves");
    if !worlds_dir.exists() {
        fs::create_dir_all(&worlds_dir)
            .with_context(|| format!("Failed to create worlds directory at {:?}", worlds_dir))?;
    }

    Ok(worlds_dir)
}

pub fn open_worlds_dir(slug: String) -> Result<(), Error> {
    let worlds_dir = get_worlds_dir(slug)?;
    if let Err(e) = open::that(worlds_dir) {
        return Err(Error::msg(format!(
            "Failed to open worlds directory: {}",
            e
        )));
    }

    Ok(())
}

pub fn open_world_dir(slug: String, world_name: String) -> Result<(), Error> {
    let worlds_dir = get_worlds_dir(slug)?;
    let world_path = worlds_dir.join(world_name);
    if !world_path.exists() {
        return Err(Error::msg(format!(
            "World directory does not exist: {:?}",
            world_path
        )));
    }

    if let Err(e) = open::that(world_path) {
        return Err(Error::msg(format!("Failed to open world directory: {}", e)));
    }

    Ok(())
}

pub async fn delete_world(
    state: State<'_, AppState>,
    slug: String,
    world_name: String,
) -> Result<(), Error> {
    let running_instances_map = state.running_instances.clone();
    let is_running = {
        let instances = running_instances_map.lock().await;
        instances.contains_key(&slug)
    };

    if is_running {
        return Err(Error::msg(format!(
            "Cannot delete world: Instance '{}' is currently running.",
            slug
        )));
    }

    let worlds_dir = get_worlds_dir(slug)?;
    let world_path = worlds_dir.join(&world_name);

    if !world_path.exists() {
        return Err(Error::msg(format!(
            "World directory to delete does not exist: {:?}",
            world_path
        )));
    }

    if !world_path.is_dir() {
        return Err(Error::msg(format!(
            "Path to delete is not a directory: {:?}",
            world_path
        )));
    }

    fs::remove_dir_all(&world_path).with_context(|| {
        format!(
            "Failed to delete world directory: {:?} for world folder {}",
            world_path, world_name
        )
    })?;

    Ok(())
}

pub fn get_worlds(instance_slug: String) -> Result<Vec<World>, Error> {
    let worlds_dir = get_worlds_dir(instance_slug)?;
    let mut worlds = Vec::new();

    for entry_result in fs::read_dir(&worlds_dir)
        .with_context(|| format!("Failed to read worlds directory: {:?}", worlds_dir))?
    {
        let entry = entry_result.with_context(|| "Failed to read directory entry")?;
        let path = entry.path();

        if path.is_dir() {
            let folder_name = path
                .file_name()
                .ok_or_else(|| {
                    Error::msg(format!("Could not get folder name from path {:?}", path))
                })?
                .to_string_lossy()
                .to_string();

            let level_dat_path = path.join("level.dat");
            let mut level_name = None;
            let mut last_played_timestamp = None;

            if level_dat_path.exists() {
                let mut file = fs::File::open(&level_dat_path).with_context(|| {
                    format!("Failed to open level.dat for world {}", folder_name)
                })?;
                let mut contents = Vec::new();
                file.read_to_end(&mut contents).with_context(|| {
                    format!("Failed to read level.dat for world {}", folder_name)
                })?;

                let cursor = Cursor::new(contents);
                let mut decoder = GzDecoder::new(cursor);

                match from_reader::<_, LevelDat>(&mut decoder) {
                    Ok(level_dat) => {
                        level_name = Some(level_dat.data.level_name);
                        if let Some(timestamp_ms) = level_dat.data.last_played {
                            if timestamp_ms > 0 {
                                last_played_timestamp =
                                    Utc.timestamp_millis_opt(timestamp_ms).single();
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Failed to parse level.dat for world {}: {}", folder_name, e);
                    }
                }
            }

            let icon_path = path.join("icon.png");
            let mut icon_data = None;
            if icon_path.exists() {
                let mut icon_file_content = Vec::new();
                match fs::File::open(&icon_path)
                    .with_context(|| format!("Failed to open icon.png for world {}", folder_name))
                {
                    Ok(mut file) => {
                        if file.read_to_end(&mut icon_file_content).is_ok() {
                            let encoded_data = base64::engine::general_purpose::STANDARD
                                .encode(&icon_file_content);
                            icon_data = Some(format!("data:image/png;base64,{}", encoded_data));
                        } else {
                            warn!("Failed to read icon.png for world {}", folder_name);
                        }
                    }
                    Err(e) => {
                        warn!("Could not open icon.png for world {}: {}", folder_name, e);
                    }
                }
            }

            worlds.push(World {
                path: path.clone(),
                folder_name: folder_name.clone(),
                level_name: level_name.unwrap_or(folder_name),
                last_played: last_played_timestamp,
                icon: icon_data,
            });
        }
    }

    worlds.sort_by(|a, b| {
        b.last_played
            .cmp(&a.last_played)
            .then_with(|| a.level_name.cmp(&b.level_name))
    });

    Ok(worlds)
}
