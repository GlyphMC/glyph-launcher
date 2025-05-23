use std::path::PathBuf;

use anyhow::Result;
use log::error;
use tauri::{AppHandle, Manager, State};

use crate::{
    AppState,
    auth::{
        self,
        account::{Account, Profile},
    },
    config::{self, LauncherSettings},
    discord,
    instance::Instance,
    java::{self, detect::JavaDetectionResult, structs::JavaConfig, test::JavaTestInfo},
    resources::{self, screenshots::Screenshot, versions::Version, worlds::World},
};

#[tauri::command]
#[specta::specta]
pub async fn login(state: State<'_, AppState>, handle: AppHandle) -> Result<Profile, String> {
    let login_handle = state.login_handle.clone();
    match auth::auth::login(&state, handle, login_handle).await {
        Ok(profile_response) => Ok(Profile::from(profile_response)),
        Err(e) => {
            error!("Login failed: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
#[specta::specta]
pub fn cancel_login(state: State<'_, AppState>) -> Result<(), String> {
    state.login_handle.cancel();
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn set_onboarding_complete() -> Result<(), String> {
    config::set_onboarding_complete().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn get_minecraft_profiles() -> Result<Vec<Profile>, String> {
    let config = config::get_config().map_err(|e| e.to_string())?;
    let default_account = Account::default();
    let accounts = config.accounts;
    let mut profiles = Vec::new();
    for account in accounts {
        if account != default_account {
            profiles.push(account.profile);
        }
    }
    Ok(profiles)
}

#[tauri::command]
#[specta::specta]
pub fn switch_account(id: String) -> Result<(), String> {
    auth::auth::switch_account(id).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn delete_account(id: String) -> Result<(), String> {
    auth::auth::delete_account(id).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn get_active_account() -> Result<Option<Account>, String> {
    let account = auth::auth::get_active_account().map_err(|e| e.to_string())?;
    Ok(account)
}

#[tauri::command]
#[specta::specta]
pub fn detect_java() -> Result<JavaDetectionResult, String> {
    let (java8, java17, java21) = java::detect::detect_java().map_err(|e| e.to_string())?;
    Ok((java8, java17, java21))
}

#[tauri::command]
#[specta::specta]
pub async fn download_java(
    state: State<'_, AppState>,
    handle: AppHandle,
    versions: Vec<i8>,
) -> Result<Vec<PathBuf>, String> {
    let paths = java::download::download_java(&state, handle, versions)
        .await
        .map_err(|e| e.to_string())?;
    Ok(paths)
}

#[tauri::command]
#[specta::specta]
pub async fn extract_java(
    handle: AppHandle,
    paths: Vec<PathBuf>,
    versions: Vec<i8>,
) -> Result<Vec<PathBuf>, String> {
    let output_paths = java::extract::extract_java(handle, paths, versions)
        .await
        .map_err(|e| e.to_string())?;
    Ok(output_paths)
}

#[tauri::command]
#[specta::specta]
pub fn test_java(
    paths: (PathBuf, PathBuf, PathBuf),
) -> Result<(JavaTestInfo, JavaTestInfo, JavaTestInfo), String> {
    let results = java::test::test_java(paths).map_err(|e| e.to_string())?;
    Ok(results)
}

#[tauri::command]
#[specta::specta]
pub fn save_java_to_config(
    paths: (PathBuf, PathBuf, PathBuf),
    automatic: bool,
) -> Result<(), String> {
    java::config::save_java_to_config(paths, automatic).map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub fn get_java_from_config() -> Result<JavaConfig, String> {
    let java_config = java::config::get_java_from_config().map_err(|e| e.to_string())?;
    Ok(java_config)
}

#[tauri::command]
#[specta::specta]
pub async fn get_instances(state: State<'_, AppState>) -> Result<Vec<Instance>, String> {
    let instances_lock = state.instances.lock().await;
    Ok(instances_lock.get_instances().to_vec())
}

#[tauri::command]
#[specta::specta]
pub async fn get_instance(state: State<'_, AppState>, slug: &str) -> Result<Instance, String> {
    let instances_lock = state.instances.lock().await;
    instances_lock
        .get_instance(slug)
        .ok_or_else(|| format!("Instance with slug '{}' not found", slug))
}

#[tauri::command]
#[specta::specta]
pub async fn create_instance(
    state: State<'_, AppState>,
    handle: AppHandle,
    instance: Instance,
) -> Result<(), String> {
    let mut instances_lock = state.instances.lock().await;
    instances_lock
        .add_instance(&state, &handle, instance)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn update_instance(
    state: State<'_, AppState>,
    handle: AppHandle,
    instance: Instance,
) -> Result<(), String> {
    let mut instances_lock = state.instances.lock().await;
    instances_lock
        .update_instance(&handle, instance)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn delete_instance(
    state: State<'_, AppState>,
    handle: AppHandle,
    slug: String,
) -> Result<(), String> {
    let mut instances_lock = state.instances.lock().await;
    instances_lock
        .delete_instance(&handle, &slug)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn launch_instance(
    state: State<'_, AppState>,
    handle: AppHandle,
    slug: String,
) -> Result<(), String> {
    match resources::launch::launch(state, handle, &slug).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Error launching instance: {:?}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn kill_instance(
    state: State<'_, AppState>,
    handle: AppHandle,
    slug: String,
) -> Result<(), String> {
    match resources::launch::kill_instance(state, handle, &slug).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Failed to kill instance {}: {}", slug, e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_versions(state: State<'_, AppState>) -> Result<Vec<Version>, String> {
    let versions = resources::versions::get_versions(state)
        .await
        .map_err(|e| e.to_string())?;
    Ok(versions)
}

#[tauri::command]
#[specta::specta]
pub async fn set_discord_activity(
    state: State<'_, AppState>,
    details: String,
    status: String,
) -> Result<(), String> {
    discord::set_activity(&state.discord_client, details, status)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn toggle_discord_rpc(state: State<'_, AppState>, enabled: bool) -> Result<(), String> {
    discord::toggle_rpc(&state.discord_client, enabled)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn get_launcher_settings() -> Result<LauncherSettings, String> {
    let config = config::get_config().map_err(|e| e.to_string())?;
    Ok(LauncherSettings {
        rich_presence: config.rich_presence,
        use_discrete_gpu: config.use_discrete_gpu,
    })
}

#[tauri::command]
#[specta::specta]
pub async fn save_launcher_settings(
    handle: AppHandle,
    settings: LauncherSettings,
) -> Result<(), String> {
    config::update_launcher_settings(&handle, &settings).map_err(|e| e.to_string())?;
    let discord_client = &handle.state::<AppState>().discord_client;
    discord::toggle_rpc(discord_client, settings.rich_presence)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_avatar(state: State<'_, AppState>, uuid: String) -> Result<String, String> {
    match auth::avatar::get_cached_avatar(&state, uuid).await {
        Ok(avatar) => Ok(avatar),
        Err(e) => {
            error!("Failed to get avatar: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
#[specta::specta]
pub fn get_screenshots(slug: String) -> Result<Vec<Screenshot>, String> {
    match resources::screenshots::get_screenshots(slug) {
        Ok(screenshots) => Ok(screenshots),
        Err(e) => {
            error!("Failed to get screenshots: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
#[specta::specta]
pub fn open_screenshots_dir(slug: String) -> Result<(), String> {
    match resources::screenshots::open_screenshots_dir(slug) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Failed to open screenshots directory: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn watch_screenshots_for_instance(handle: AppHandle, slug: String) -> Result<(), String> {
    match resources::screenshots::watch_screenshots(handle, &slug) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!(
                "Failed to start screenshot watcher for slug {}: {}",
                slug, e
            );
            Err(e.to_string())
        }
    }
}

#[tauri::command]
#[specta::specta]
pub fn stop_watching_screenshots() -> Result<(), String> {
    resources::screenshots::stop_watching_screenshots();
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn get_worlds(slug: String) -> Result<Vec<World>, String> {
    match resources::worlds::get_worlds(slug) {
        Ok(worlds) => Ok(worlds),
        Err(e) => {
            error!("Failed to get worlds: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
#[specta::specta]
pub fn open_worlds_dir(slug: String) -> Result<(), String> {
    match resources::worlds::open_worlds_dir(slug) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Failed to open worlds directory: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
#[specta::specta]
pub fn open_world_dir(slug: String, world_name: String) -> Result<(), String> {
    match resources::worlds::open_world_dir(slug, world_name) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Failed to open world directory: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn delete_world(
    state: State<'_, AppState>,
    slug: String,
    world_name: String,
) -> Result<(), String> {
    match resources::worlds::delete_world(state, slug, world_name).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Failed to delete world: {}", e);
            Err(e.to_string())
        }
    }
}
