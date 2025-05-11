use std::{
    io::{BufRead, BufReader},
    path::Path,
    process::{Command, Stdio},
    sync::Arc,
    thread,
};

use anyhow::{Error, Result, anyhow};
use chrono::Utc;
use discord_rich_presence::DiscordIpcClient;
use log::{error, info};
use tauri::{AppHandle, Emitter, State};
use tokio::{sync::Mutex, time::Instant};
use walkdir::WalkDir;

use crate::{
    AppState, Payload, config, discord,
    instance::Instance,
    resources::{
        assets::AssetManager,
        gpu_prefs,
        version::{VersionManifest, get_version_manifest},
    },
};

#[cfg(target_os = "windows")]
use crate::resources::gpu_prefs::GpuPreference;

pub async fn launch(
    state: State<'_, AppState>,
    handle: AppHandle,
    slug: &str,
) -> Result<(), Error> {
    info!("Launching instance: {}", slug);

    let client = state.client.lock().await.clone();
    let config_dir = config::get_config_dir()?;
    let discord_client_state = state.discord_client.clone();

    let (game_url, needs_asset_download) = {
        let instances_config = state.instances.lock().await;
        let instance_data = instances_config
            .get_instance(slug)
            .ok_or_else(|| anyhow!("Instance {} not found", slug))?;

        (
            instance_data.game.url.clone(),
            !instance_data.settings.has_launched,
        )
    };

    let version_manifest = get_version_manifest(&state, &game_url).await?;

    if needs_asset_download {
        handle.emit(
            "instance-download-assets-started",
            Payload {
                message: "Download started",
            },
        )?;

        info!("Downloading assets for instance: {}", slug);
        let asset_manager = AssetManager::new(client, &handle, &config_dir);

        asset_manager
            .download_assets(&version_manifest)
            .await
            .map_err(|e| anyhow!("Failed to download assets for {}: {}", slug, e))?;
        info!("Assets downloaded for instance: {}", slug);

        asset_manager
            .download_libraries(&version_manifest)
            .await
            .map_err(|e| anyhow!("Failed to download libraries for {}: {}", slug, e))?;
        info!("Libraries downloaded for instance: {}", slug);

        asset_manager
            .download_version_jar(&version_manifest)
            .await
            .map_err(|e| anyhow!("Failed to download version JAR for {}: {}", slug, e))?;
        info!("Version JAR downloaded for instance: {}", slug);

        handle.emit(
            "instance-download-assets-finished",
            Payload {
                message: "Download finished",
            },
        )?;

        let mut instances_config = state.instances.lock().await;
        let mut instance_to_update = instances_config
            .get_instance(slug)
            .ok_or_else(|| anyhow!("Instance {} not found for asset update", slug))?;

        instance_to_update.settings.has_launched = true;
        instances_config.update_instance(handle.clone(), instance_to_update)?;
    }

    let (instance_game_launch, instance_dir) = {
        let instances_config = state.instances.lock().await;
        let instance = instances_config
            .get_instance(slug)
            .ok_or_else(|| anyhow!("Instance {} not found for game launch", slug))?;
        let dir = config_dir.join("instances").join(slug);

        (instance, dir)
    };

    let start_time = Instant::now();

    let launch_game_result = launch_game(
        instance_game_launch.clone(),
        &instance_dir,
        &version_manifest,
        handle.clone(),
        discord_client_state.clone(),
    );

    info!("Stopped instance: {}", slug);

    let duration_played = start_time.elapsed().as_secs();
    let current_time = Utc::now();

    {
        let mut instances_config = state.instances.lock().await;
        let mut instance_to_update = instances_config
            .get_instance(slug)
            .ok_or_else(|| anyhow!("Instance {} not found for time played update", slug))?;

        instance_to_update.settings.time_played += duration_played;
        instance_to_update.settings.last_played = Some(current_time);

        if !instance_to_update.settings.has_launched {
            instance_to_update.settings.has_launched = true;
        }

        instances_config.update_instance(handle.clone(), instance_to_update)?;
    }

    launch_game_result?;

    Ok(())
}

fn launch_game(
    instance: Instance,
    instance_dir: &Path,
    version_manifest: &VersionManifest,
    handle: AppHandle,
    discord_client_state: Arc<Mutex<Option<DiscordIpcClient>>>,
) -> Result<(), Error> {
    discord::set_activity(
        discord_client_state.clone(),
        format!("Playing {}", instance.name),
        format!("Version: {}", instance.game.version),
    );

    let config = config::get_config()?;
    let config_dir = config::get_config_dir()?;

    let main_class = &version_manifest.main_class;
    let classpath = construct_classpath(&config_dir, version_manifest)?;
    let assets_dir = config_dir.join("assets");

    let account = config
        .accounts
        .iter()
        .find(|acc| acc.active)
        .ok_or_else(|| anyhow!("No active account found"))?;

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
        &instance_dir.to_str().unwrap(),
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
        "Glyph Launcher",
    ];

    if !settings.maximized {
        game_args.push("--width");
        game_args.push(width.as_str());
        game_args.push("--height");
        game_args.push(height.as_str());
    }

    let mut command = Command::new(&instance.java.path);
    command
        .current_dir(instance_dir)
        .arg("-cp")
        .arg(classpath)
        .arg(main_class)
        .args(game_args);

    #[cfg(target_os = "windows")]
    {
        let gpu_to_use = if config.use_discrete_gpu {
            GpuPreference::Discrete
        } else {
            GpuPreference::Integrated
        };
        if let Err(e) = gpu_prefs::set_gpu_preference(&instance.java.path, gpu_to_use) {
            error!("Failed to set GPU preference: {}", e);
        }
    }

    #[cfg(target_os = "linux")]
    if config.use_discrete_gpu {
        command
            .env("DRI_PRIME", "1")
            .env("__NV_PRIME_RENDER_OFFLOAD", "1")
            .env("__VK_LAYER_NV_optimus", "NVIDIA_only")
            .env("__GLX_VENDOR_LIBRARY_NAME", "nvidia");
    }

    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| Error::msg(format!("Failed to launch game: {}", e)))?;

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| anyhow!("Failed to capture stdout from game process"))?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| anyhow!("Failed to capture stderr from game process"))?;
    let slug_arc = Arc::new(instance.slug.replace(".", "_"));

    let stdout_handle = handle.clone();
    let slug = Arc::clone(&slug_arc);
    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines().map_while(Result::ok) {
            let event_payload = format!("[stdout] {}", line);
            if let Err(e) = stdout_handle.emit(
                &format!("{}-log", slug),
                Payload {
                    message: &event_payload,
                },
            ) {
                error!("Failed to send stdout event: {}", e);
            }
        }
    });

    let stderr_handle = handle.clone();
    let slug = Arc::clone(&slug_arc);
    thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines().map_while(Result::ok) {
            let event_payload = format!("[stderr] {}", line);
            if let Err(e) = stderr_handle.emit(
                &format!("{}-log", slug),
                Payload {
                    message: &event_payload,
                },
            ) {
                error!("Failed to send stderr event: {}", e);
            }
        }
    });

    let status = child
        .wait()
        .map_err(|e| Error::msg(format!("Failed to wait for game process: {}", e)))?;

    #[cfg(target_os = "windows")]
    {
        if let Err(e) = gpu_prefs::delete_gpu_preference(&instance.java.path) {
            error!("Failed to delete GPU preference: {}", e);
        }
    }

    discord::set_activity(
        discord_client_state,
        "Exploring the Launcher".to_string(),
        "Idle".to_string(),
    );

    if !status.success() {
        return Err(Error::msg(format!(
            "Game process exited with status: {}",
            status
        )));
    }

    Ok(())
}

fn construct_classpath(
    config_dir: &Path,
    version_manifest: &VersionManifest,
) -> Result<String, Error> {
    let mut classpath_entries: Vec<String> = Vec::new();
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
