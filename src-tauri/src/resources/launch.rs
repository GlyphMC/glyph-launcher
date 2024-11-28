use std::{fs, path::PathBuf, process::Command};

use anyhow::{anyhow, Error, Result};
use log::info;
use tauri::State;
use walkdir::WalkDir;

use crate::{
    config,
    instances::{instance, structs::Instance},
    resources::{assets::AssetManager, version::get_version_manifest},
    AppState,
};

use super::version::VersionManifest;

pub async fn launch(state: State<'_, AppState>, slug: String) -> Result<(), Error> {
    info!("Launching instance: {}", &slug);

    let client = state.client.lock().await.clone();
    let instance = instance::get_instance(&slug)?;
    let instance_dir = instance::get_instances_path()?.join(&slug);
    let version_manifest = get_version_manifest(state, &instance.game.url).await?;

    if !instance.settings.has_launched {
        info!("Downloading assets for instance: {}", &slug);
        let asset_manager = AssetManager::new(client, &instance_dir);

        let _ = asset_manager
            .download_assets(&version_manifest)
            .await
            .map_err(|e| e.to_string());
        info!("Assets downloaded for instance: {}", &slug);

        let _ = asset_manager
            .download_libraries(&version_manifest)
            .await
            .map_err(|e| e.to_string());
        info!("Libraries downloaded for instance: {}", &slug);

        let _ = asset_manager
            .download_version_jar(&version_manifest)
            .await
            .map_err(|e| e.to_string());
        info!("Version JAR downloaded for instance: {}", &slug);
    }

    launch_game(instance, &instance_dir, &version_manifest)?;

    Ok(())
}

fn launch_game(
    instance: Instance,
    minecraft_dir: &PathBuf,
    version_manifest: &VersionManifest,
) -> Result<(), Error> {
    info!("Launching game: {}", &instance.game.version);

    let java_path = "java";
    let main_class = &version_manifest.main_class;
    let classpath = construct_classpath(minecraft_dir, version_manifest)?;
    let assets_dir = minecraft_dir.join("assets");
    let config = config::get_config()?;
    let account = &config.accounts[0];
    let profile = &account.profile;
    let version = &instance.game.version;

    let game_args = vec![
        "--username",
        &profile.name,
        "--version",
        version,
        "--gameDir",
        &minecraft_dir.to_str().unwrap(),
        "--assetsDir",
        &assets_dir.to_str().unwrap(),
        "--assetIndex",
        version,
        "--uuid",
        &profile.id,
        "--accessToken",
        &account.access_token,
        "--userType",
        "msa",
        "--versionType",
        "release",
		"--gameDir",
		&minecraft_dir.to_str().unwrap(),
    ];

    let mut command = Command::new(java_path);
    command
        .arg("-cp")
        .arg(classpath)
        .arg(main_class)
        .args(&game_args);

    command
        .spawn()
        .map_err(|e| Error::msg(format!("Failed to launch game: {}", e)))?;

    Ok(())
}

fn construct_classpath(
    minecraft_dir: &PathBuf,
    version_manifest: &VersionManifest,
) -> Result<String, Error> {
    let mut classpath_entries = Vec::new();
    let libraries_dir = minecraft_dir.join("libraries");

    for entry in WalkDir::new(&libraries_dir)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path.is_file() && path.extension() == Some("jar".as_ref()) {
            classpath_entries.push(path.to_string_lossy().to_string());
        }
    }

    let minecraft_jar = minecraft_dir
        .join("versions")
        .join(&version_manifest.id)
        .join(format!("{}.jar", version_manifest.id));
    if minecraft_jar.exists() {
        classpath_entries.push(minecraft_jar.to_string_lossy().to_string());
    } else {
        return Err(anyhow!(
            "Minecraft JAR not found at {}",
            minecraft_jar.display()
        ));
    }

    #[cfg(target_os = "windows")]
    let separator = ";";
    #[cfg(not(target_os = "windows"))]
    let separator = ":";

    Ok(classpath_entries.join(separator))
}
