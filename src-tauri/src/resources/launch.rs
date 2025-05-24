use std::{
    io::{BufRead, BufReader},
    path::Path,
    process::{Command, ExitStatus, Stdio},
    sync::Arc,
    thread,
    time::Duration,
};

use anyhow::{Error, Result, anyhow};
use chrono::Utc;
use discord_rich_presence::DiscordIpcClient;
use futures::try_join;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{AppHandle, State};
use tauri_specta::Event;
use tokio::{sync::Mutex, time::Instant};
use walkdir::WalkDir;

use crate::{
    AppState, ProcessHandle, RunningInstancesMap,
    auth::account::Account,
    config::{self, Config},
    discord,
    instance::Instance,
    resources::{
        assets::AssetManager,
        version::{VersionManifest, get_version_manifest},
    },
};

#[cfg(target_os = "windows")]
use crate::resources::gpu_prefs::{self, GpuPreference};

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct AssetsDownloadStartedEvent(String);

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct AssetsDownloadFinishedEvent(String);

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct InstanceStartedEvent<'a> {
    pub slug: &'a str,
    pub message: &'a str,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct InstanceStoppedEvent<'a> {
    pub slug: &'a str,
    pub message: &'a str,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct InstanceLogEvent<'a> {
    pub slug: &'a str,
    pub line: &'a str,
}

async fn download_instance_assets(
    state: &State<'_, AppState>,
    handle: &AppHandle,
    slug: &str,
    version_manifest: &VersionManifest,
    config_dir: &Path,
) -> Result<()> {
    AssetsDownloadStartedEvent("Download started".into()).emit(handle)?;

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

    AssetsDownloadFinishedEvent("Download finished".into()).emit(handle)?;

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
    let running_instances_map = state.running_instances.clone();

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
        running_instances_map,
    )
    .await;

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

fn prepare_game_args<'a>(
    instance: &'a Instance,
    instance_dir: &'a str,
    version_manifest: &'a VersionManifest,
    account: &'a Account,
    assets_dir: &'a str,
    width: &'a str,
    height: &'a str,
) -> Result<Vec<&'a str>, Error> {
    let profile = &account.profile;
    let version = &instance.game.version;
    let settings = &instance.settings;

    #[rustfmt::skip]
    let mut game_args = vec![
        "--username", &profile.name,
        "--version", version,
        "--gameDir", instance_dir,
        "--assetsDir", assets_dir,
        "--assetIndex", &version_manifest.asset_index.id,
        "--uuid", &profile.id,
        "--accessToken", &account.access_token,
        "--userType", "msa",
        "--versionType", "Glyph Launcher",
    ];

    if !settings.maximized {
        game_args.push("--width");
        game_args.push(width);
        game_args.push("--height");
        game_args.push(height);
    }

    Ok(game_args)
}

fn configure_launch_command(
    instance: &Instance,
    instance_dir: &Path,
    main_class: &str,
    classpath: &str,
    game_args: Vec<&str>,
    config: &Config,
) -> Command {
    let mut command = Command::new(&instance.java.path);
    command
        .current_dir(instance_dir)
        .arg("-cp")
        .arg(classpath)
        .arg(main_class)
        .args(game_args);

    #[cfg(target_os = "windows")]
    {
        let gpu = if config.use_discrete_gpu {
            GpuPreference::Discrete
        } else {
            GpuPreference::Integrated
        };
        if let Err(e) = gpu_prefs::set_gpu_preference(&instance.java.path, gpu) {
            error!("Failed to set GPU preference: {}", e);
        }
    }

    #[cfg(target_os = "linux")]
    {
        if config.use_discrete_gpu {
            command
                .env("DRI_PRIME", "1")
                .env("__NV_PRIME_RENDER_OFFLOAD", "1")
                .env("__VK_LAYER_NV_optimus", "NVIDIA_only")
                .env("__GLX_VENDOR_LIBRARY_NAME", "nvidia");
        }

        if let Ok(ld_library_path) = std::env::var("LD_LIBRARY_PATH") {
            command.env("LD_LIBRARY_PATH", ld_library_path);
        }
    }

    command
}

async fn launch_game(
    instance: &Instance,
    instance_dir_path: &Path,
    version_manifest: &VersionManifest,
    handle: &AppHandle,
    discord_client_state: &Arc<Mutex<Option<DiscordIpcClient>>>,
    running_instances_map: Arc<Mutex<RunningInstancesMap>>,
) -> Result<(), Error> {
    let config = config::get_config()?;
    let config_dir = config::get_config_dir()?;

    let main_class = &version_manifest.main_class;
    let classpath = construct_classpath(&config_dir, version_manifest)?;
    let assets_dir = config_dir.join("assets");

    if config.rich_presence {
        if let Err(e) = discord::set_activity(
            discord_client_state,
            format!("Playing {}", instance.name),
            format!("Version: {}", instance.game.version),
        )
        .await
        {
            warn!("Failed to set Discord activity (playing): {:?}", e);
        }
    }

    let account = config
        .accounts
        .iter()
        .find(|acc| acc.active)
        .ok_or_else(|| anyhow!("No active account found"))?;

    let instance_dir = instance_dir_path.to_str().ok_or_else(|| {
        anyhow!(
            "Instance directory path is not valid UTF-8: {:?}",
            instance_dir_path
        )
    })?;
    let assets_dir = assets_dir
        .to_str()
        .ok_or_else(|| anyhow!("Assets directory path is not valid UTF-8: {:?}", assets_dir))?;

    let width = instance.settings.window_width.to_string();
    let height = instance.settings.window_height.to_string();

    let game_args = prepare_game_args(
        instance,
        instance_dir,
        version_manifest,
        account,
        assets_dir,
        &width,
        &height,
    )?;

    let mut command = configure_launch_command(
        instance,
        instance_dir_path,
        main_class,
        &classpath,
        game_args,
        &config,
    );

    let formatted_slug = instance.slug.replace(".", "_");

    let child = command
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| Error::msg(format!("Failed to launch game: {}", e)))?;

    let process_id = child.id();
    info!("Launched game process with ID: {}", process_id);

    let process_handle: ProcessHandle = Arc::new(Mutex::new(Some(child)));
    {
        let mut running_instances = running_instances_map.lock().await;
        running_instances.insert(instance.slug.clone(), Arc::clone(&process_handle));
    }

    InstanceStartedEvent {
        slug: &formatted_slug,
        message: "Game instance started",
    }
    .emit(handle)?;

    let stdout = {
        let mut child_opt = process_handle.lock().await;
        child_opt.as_mut().and_then(|c| c.stdout.take())
    };

    if let Some(stdout) = stdout {
        let stdout_handle = handle.clone();
        let log_slug = formatted_slug.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines().map_while(Result::ok) {
                let line = line.trim().to_string();
                if !line.is_empty() {
                    if let Err(e) = (InstanceLogEvent {
                        slug: &log_slug,
                        line: &line,
                    })
                    .emit(&stdout_handle)
                    {
                        error!("Failed to emit instance log event for {}: {}", log_slug, e);
                    }
                }
            }
        });
    }

    let status = wait_for_process_completion(&process_handle).await?;

    {
        let mut child_opt = process_handle.lock().await;
        *child_opt = None;
    }

    {
        let mut running_instances = running_instances_map.lock().await;
        running_instances.remove(&instance.slug);
    }

    InstanceStoppedEvent {
        slug: &formatted_slug,
        message: "Game instance stopped",
    }
    .emit(handle)?;

    #[cfg(target_os = "windows")]
    {
        if let Err(e) = gpu_prefs::delete_gpu_preference(&instance.java.path) {
            error!("Failed to delete GPU preference: {}", e);
        }
    }

    if config.rich_presence {
        if let Err(e) = discord::set_activity(
            discord_client_state,
            "Exploring the Launcher".to_string(),
            "Idle".to_string(),
        )
        .await
        {
            warn!("Failed to set Discord activity (idle): {:?}", e);
        }
    }

    if !status.success() {
        return Err(Error::msg(format!(
            "Game process exited with status: {}",
            status
        )));
    }

    Ok(())
}

async fn wait_for_process_completion(process_handle: &ProcessHandle) -> Result<ExitStatus, Error> {
    tokio::task::spawn_blocking({
        let handle = Arc::clone(process_handle);
        move || loop {
            thread::sleep(Duration::from_millis(200));
            let mut guard = handle.blocking_lock();

            match guard.as_mut().map(|c| c.try_wait()) {
                Some(Ok(Some(status))) => return Ok(status),
                Some(Ok(None)) => continue,
                Some(Err(e)) => return Err(anyhow!("Error checking process status: {}", e)),
                None => return success_exit_status(),
            }
        }
    })
    .await
    .map_err(|e| anyhow!("Failed to wait for process: {}", e))?
}

fn success_exit_status() -> Result<ExitStatus, Error> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::ExitStatusExt;
        Ok(ExitStatus::from_raw(0))
    }
    #[cfg(not(target_os = "windows"))]
    {
        use std::os::unix::process::ExitStatusExt;
        Ok(ExitStatus::from_raw(0))
    }
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

pub async fn kill_instance(
    state: State<'_, AppState>,
    handle: AppHandle,
    slug: &str,
) -> Result<(), Error> {
    info!("Attempting to kill instance: {}", slug);

    let process_handle_arc = get_process_handle(&state, slug).await?;
    let kill_result = perform_kill(&process_handle_arc, slug).await;
    cleanup_instance(&state, &handle, slug).await?;

    kill_result
}

async fn get_process_handle(
    state: &State<'_, AppState>,
    slug: &str,
) -> Result<ProcessHandle, Error> {
    let running_instances_map = state.running_instances.clone();
    let instances = running_instances_map.lock().await;

    instances.get(slug).cloned().ok_or_else(|| {
        warn!("Instance {} not found in running instances map.", slug);
        anyhow!("Instance {} is not currently running.", slug)
    })
}

async fn perform_kill(process_handle: &ProcessHandle, slug: &str) -> Result<(), Error> {
    let mut handle_guard = process_handle.lock().await;

    let child = match handle_guard.as_mut() {
        Some(child) => child,
        None => {
            info!("Instance {} already stopped.", slug);
            return Ok(());
        }
    };

    let pid = child.id();
    info!("Attempting to kill PID: {}", pid);

    if let Err(e) = child.kill() {
        error!("Failed to kill {} gracefully: {}", slug, e);
    }

    if wait_for_termination(pid).await {
        info!("Process {} terminated", slug);
        *handle_guard = None;
        Ok(())
    } else {
        force_kill_and_cleanup(pid, &mut handle_guard, slug)
    }
}

async fn wait_for_termination(pid: u32) -> bool {
    let result = tokio::time::timeout(
        Duration::from_secs(3),
        tokio::task::spawn_blocking(move || {
            for _ in 0..30 {
                thread::sleep(Duration::from_millis(100));

                #[cfg(target_os = "windows")]
                {
                    if let Ok(output) = Command::new("tasklist")
                        .args(["/FI", &format!("PID eq {}", pid), "/FO", "CSV"])
                        .output()
                    {
                        let output_str = String::from_utf8_lossy(&output.stdout);
                        if !output_str.contains(&pid.to_string()) {
                            return true;
                        }
                    }
                }

                #[cfg(not(target_os = "windows"))]
                {
                    if unsafe { libc::kill(pid as i32, 0) } != 0 {
                        return true;
                    }
                }
            }
            false
        }),
    )
    .await;

    matches!(result, Ok(Ok(true)))
}

fn force_kill_and_cleanup(
    pid: u32,
    process_handle_opt: &mut Option<std::process::Child>,
    slug: &str,
) -> Result<(), Error> {
    info!("Attempting force kill for PID: {}", pid);

    match force_kill_process(pid) {
        Ok(_) => {
            info!("Force killed process {} successfully", pid);
            *process_handle_opt = None;
            Ok(())
        }
        Err(e) => {
            error!("Failed to force kill PID {}: {}", pid, e);
            Err(anyhow!(
                "Failed to kill instance {}: Process did not respond to kill signals",
                slug
            ))
        }
    }
}

async fn cleanup_instance(
    state: &State<'_, AppState>,
    handle: &AppHandle,
    slug: &str,
) -> Result<(), Error> {
    {
        let mut instances = state.running_instances.lock().await;
        instances.remove(slug);
    }

    let formatted_slug = slug.replace(".", "_");
    InstanceStoppedEvent {
        slug: &formatted_slug,
        message: "Instance killed by user",
    }
    .emit(handle)
    .map_err(|e| {
        error!(
            "Failed to emit InstanceStoppedEvent after killing {}: {}",
            slug, e
        );
        anyhow!("Failed to emit instance stopped event: {}", e)
    })?;

    info!(
        "Instance {} removed from running map after kill attempt.",
        slug
    );
    Ok(())
}

#[cfg(target_os = "windows")]
fn force_kill_process(pid: u32) -> Result<(), Error> {
    let output = Command::new("taskkill")
        .args(["/F", "/PID", &pid.to_string()])
        .output()
        .map_err(|e| anyhow!("Failed to execute taskkill: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow!("taskkill failed: {}", stderr))
    }
}

#[cfg(not(target_os = "windows"))]
fn force_kill_process(pid: u32) -> Result<(), Error> {
    let output = Command::new("kill")
        .args(["-9", &pid.to_string()])
        .output()
        .map_err(|e| anyhow!("Failed to execute kill -9: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow!("kill -9 failed: {}", stderr))
    }
}
