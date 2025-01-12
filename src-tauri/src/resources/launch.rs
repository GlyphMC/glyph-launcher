use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
    process::{Command, Stdio},
    thread,
};

use anyhow::{anyhow, Error, Result};
use log::info;
use tauri::State;
use walkdir::WalkDir;

use crate::{
    config,
    instance::Instance,
    resources::{assets::AssetManager, version::get_version_manifest},
    AppState,
};

use super::version::VersionManifest;

pub async fn launch(state: State<'_, AppState>, slug: &str) -> Result<(), Error> {
    info!("Launching instance: {}", slug);

    let client = state.client.lock().await.clone();
    let instances_lock = state.instances.lock().await;

    let config_dir = config::get_config_dir()?;
    let instance = instances_lock.get_instance(slug).unwrap();
    let instance_dir = config_dir.join("instances").join(slug);
    let version_manifest = get_version_manifest(&state, &instance.game.url).await?;

    if !instance.settings.has_launched {
        info!("Downloading assets for instance: {}", slug);
        let asset_manager = AssetManager::new(client, &config_dir);

        let _ = asset_manager
            .download_assets(&version_manifest)
            .await
            .map_err(|e| e.to_string());
        info!("Assets downloaded for instance: {}", slug);

        let _ = asset_manager
            .download_libraries(&version_manifest)
            .await
            .map_err(|e| e.to_string());
        info!("Libraries downloaded for instance: {}", slug);

        let _ = asset_manager
            .download_version_jar(&version_manifest)
            .await
            .map_err(|e| e.to_string());
        info!("Version JAR downloaded for instance: {}", slug);
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

    let config = config::get_config()?;
    let config_dir = config::get_config_dir()?;

    let main_class = &version_manifest.main_class;
    let classpath = construct_classpath(&config_dir, version_manifest)?;
    let assets_dir = config_dir.join("assets");
    let account = &config.accounts[0];
    let profile = &account.profile;
    let version = &instance.game.version;
    let settings = &instance.settings;
    let width = settings.window_width.to_string();
    let height = settings.window_height.to_string();

    let mut game_args = vec![
        "--username",
        &profile.name,
        "--version",
        version,
        "--gameDir",
        &minecraft_dir.to_str().unwrap(),
        "--assetsDir",
        &assets_dir.to_str().unwrap(),
        "--assetIndex",
        version_manifest.asset_index.id.as_str(),
        "--uuid",
        &profile.id,
        "--accessToken",
        &account.access_token,
        "--userType",
        "msa",
        "--versionType",
        "release",
    ];

    if !settings.maximized {
        game_args.push("--width");
        game_args.push(width.as_str());
        game_args.push("--height");
        game_args.push(height.as_str());
    }

    let mut command = Command::new(instance.java.path);
    command
        .arg("-cp")
        .arg(classpath)
        .arg(main_class)
        .args(&game_args);

    // info!("{:#?}", command);

    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| Error::msg(format!("Failed to launch game: {}", e)))?;

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                info!("[stdout] {}", line);
            }
        }
    });

    thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(line) = line {
                info!("[stderr] {}", line);
            }
        }
    });

    let status = child
        .wait()
        .map_err(|e| Error::msg(format!("Failed to wait for game process: {}", e)))?;

    if !status.success() {
        return Err(Error::msg(format!(
            "Game process exited with status: {}",
            status
        )));
    }

    Ok(())
}

fn construct_classpath(
    config_dir: &PathBuf,
    version_manifest: &VersionManifest,
) -> Result<String, Error> {
    let mut classpath_entries = Vec::new();
    let libraries_dir = config_dir.join("libraries");

    for entry in WalkDir::new(&libraries_dir)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path.is_file() && path.extension() == Some("jar".as_ref()) {
            classpath_entries.push(path.to_string_lossy().to_string());
        }
    }

    let minecraft_jar = config_dir
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
