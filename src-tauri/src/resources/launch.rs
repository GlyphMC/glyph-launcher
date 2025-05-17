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
use futures::try_join;
use log::{error, info};
use tauri::{AppHandle, Emitter, State};
use tokio::{sync::Mutex, time::Instant};
use walkdir::WalkDir;

use crate::{
    AppState, Payload, config, discord,
    instance::Instance,
    resources::{
        assets::AssetManager,
        version::{VersionManifest, get_version_manifest},
    },
};

#[cfg(target_os = "windows")]
use crate::resources::gpu_prefs::{self, GpuPreference};

async fn download_instance_assets(
    state: &State<'_, AppState>,
    handle: &AppHandle,
    slug: &str,
    version_manifest: &VersionManifest,
    config_dir: &Path,
) -> Result<()> {
    handle.emit(
        "instance-download-assets-started",
        Payload {
            message: "Download started",
        },
    )?;

    info!("Downloading assets for instance: {}", slug);
    let client = state.client.lock().await.clone();
    let asset_manager = AssetManager::new(client, handle, config_dir);

    let assets_download_future = async {
        asset_manager
            .download_assets(version_manifest)
            .await
            .map_err(|e| anyhow!("Failed to download assets for {}: {}", slug, e))?;
        info!("Assets downloaded for instance: {}", slug);
        Result::<()>::Ok(())
    };

    let libraries_download_future = async {
        asset_manager
            .download_libraries(version_manifest)
            .await
            .map_err(|e| anyhow!("Failed to download libraries for {}: {}", slug, e))?;
        info!("Libraries downloaded for instance: {}", slug);
        Result::<()>::Ok(())
    };

    let version_jar_download_future = async {
        asset_manager
            .download_version_jar(version_manifest)
            .await
            .map_err(|e| anyhow!("Failed to download version JAR for {}: {}", slug, e))?;
        info!("Version JAR downloaded for instance: {}", slug);
        Result::<()>::Ok(())
    };

    try_join!(
        assets_download_future,
        libraries_download_future,
        version_jar_download_future
    )?;

    info!(
        "All asset components downloaded successfully for instance: {}",
        slug
    );

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
    instances_config.update_instance(handle, instance_to_update)?;

    Ok(())
}

pub async fn launch(
    state: State<'_, AppState>,
    handle: AppHandle,
    slug: &str,
) -> Result<(), Error> {
    info!("Launching instance: {}", slug);

    let config_dir = config::get_config_dir()?;
    let discord_client_state = &state.discord_client;

    let (needs_asset_download, version_manifest) = {
        let instances_config = state.instances.lock().await;
        let instance_data = instances_config
            .get_instance(slug)
            .ok_or_else(|| anyhow!("Instance {} not found", slug))?;

        let needs_download = !instance_data.settings.has_launched;
        let manifest = get_version_manifest(&state, &instance_data.game.url).await?;

        (needs_download, manifest)
    };

    if needs_asset_download {
        download_instance_assets(&state, &handle, slug, &version_manifest, &config_dir).await?;
    }

    let (instance_game_launch, instance_dir) = {
        let instances_config = state.instances.lock().await;
        let instance = instances_config
            .get_instance(slug)
            .ok_or_else(|| anyhow!("Instance {} not found for game launch", slug))?;
        let dir = config_dir.join("instances").join(slug);

        (instance, dir)
    };

    if !instance_dir.exists() {
        tokio::fs::create_dir_all(&instance_dir)
            .await
            .map_err(|e| {
                anyhow!(
                    "Failed to create instance directory {}: {}",
                    instance_dir.display(),
                    e
                )
            })?;
    }

    let start_time = Instant::now();

    let launch_game_result = launch_game(
        &instance_game_launch,
        &instance_dir,
        &version_manifest,
        &handle,
        discord_client_state,
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

        instances_config.update_instance(&handle, instance_to_update)?;
    }

    launch_game_result?;

    Ok(())
}

fn launch_game(
    instance: &Instance,
    instance_dir: &Path,
    version_manifest: &VersionManifest,
    handle: &AppHandle,
    discord_client_state: &Arc<Mutex<Option<DiscordIpcClient>>>,
) -> Result<(), Error> {
    let config = config::get_config()?;
    let config_dir = config::get_config_dir()?;

    let main_class = &version_manifest.main_class;
    let classpath = construct_classpath(&config_dir, version_manifest)?;
    let assets_dir = config_dir.join("assets");

    if config.rich_presence {
        discord::set_activity(
            discord_client_state,
            format!("Playing {}", instance.name),
            format!("Version: {}", instance.game.version),
        );
    }

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

    let formatted_slug = instance.slug.replace(".", "_");
    let slug_arc = Arc::new(formatted_slug);

    let mut child = command
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| Error::msg(format!("Failed to launch game: {}", e)))?;

    if let Err(e) = handle.emit(
        &format!("{}-instance-started", slug_arc),
        Payload {
            message: "Game instance started",
        },
    ) {
        error!(
            "Failed to emit game instance started event for {}: {}",
            instance.slug, e
        );
    }

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| anyhow!("Failed to capture stdout from game process"))?;

    let stdout_handle = handle.clone();
    let slug = Arc::clone(&slug_arc);
    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines().map_while(Result::ok) {
            if let Err(e) = stdout_handle.emit(&format!("{}-log", slug), Payload { message: &line })
            {
                error!("Failed to send stdout event: {}", e);
            }
        }
    });

    let status = child
        .wait()
        .map_err(|e| Error::msg(format!("Failed to wait for game process: {}", e)))?;

    if let Err(e) = handle.emit(
        &format!("{}-instance-stopped", slug_arc),
        Payload {
            message: "Game instance stopped",
        },
    ) {
        error!(
            "Failed to emit game instance stopped event for {}: {}",
            instance.slug, e
        );
    }

    #[cfg(target_os = "windows")]
    {
        if let Err(e) = gpu_prefs::delete_gpu_preference(&instance.java.path) {
            error!("Failed to delete GPU preference: {}", e);
        }
    }

    if config.rich_presence {
        discord::set_activity(
            discord_client_state,
            "Exploring the Launcher".to_string(),
            "Idle".to_string(),
        );
    }

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
