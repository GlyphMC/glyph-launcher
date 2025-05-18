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
    java::{self, structs::JavaConfig, test::JavaTestInfo},
    resources::{self, versions::Version},
};

#[tauri::command]
pub async fn login(state: State<'_, AppState>, handle: AppHandle) -> Result<Profile, ()> {
    let login_handle = state.login_handle.clone();
    match auth::auth::login(&state, handle, login_handle).await {
        Ok(profile_response) => Ok(Profile::from(profile_response)),
        Err(e) => {
            error!("Login failed: {}", e);
            Err(())
        }
    }
}

#[tauri::command]
pub fn cancel_login(state: State<'_, AppState>) -> Result<(), ()> {
    state.login_handle.cancel();
    Ok(())
}

#[tauri::command]
pub fn set_onboarding_complete() -> Result<(), ()> {
    config::set_onboarding_complete().unwrap();
    Ok(())
}

#[tauri::command]
pub fn get_minecraft_profiles() -> Result<Vec<Profile>, ()> {
    let config = config::get_config().unwrap();
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
pub fn switch_account(id: String) -> Result<(), ()> {
    auth::auth::switch_account(id).unwrap();
    Ok(())
}

#[tauri::command]
pub fn delete_account(id: String) -> Result<(), ()> {
    auth::auth::delete_account(id).unwrap();
    Ok(())
}

#[tauri::command]
pub fn get_active_account() -> Result<Option<Account>, ()> {
    let account = auth::auth::get_active_account().unwrap();
    Ok(account)
}

#[tauri::command]
pub async fn download_java(
    state: State<'_, AppState>,
    handle: AppHandle,
) -> Result<(PathBuf, PathBuf, PathBuf), ()> {
    let paths = java::download::download_java(&state, handle).await.unwrap();
    Ok((paths.0, paths.1, paths.2))
}

#[tauri::command]
pub async fn extract_java(
    handle: AppHandle,
    paths: (PathBuf, PathBuf, PathBuf),
) -> Result<(PathBuf, PathBuf, PathBuf), ()> {
    let paths = java::extract::extract_java(handle, paths).await.unwrap();
    Ok((paths.0, paths.1, paths.2))
}

#[tauri::command]
pub fn test_java(
    paths: (PathBuf, PathBuf, PathBuf),
) -> Result<(JavaTestInfo, JavaTestInfo, JavaTestInfo), ()> {
    let results = java::test::test_java(paths).unwrap();
    Ok(results)
}

#[tauri::command]
pub fn save_java_to_config(paths: (PathBuf, PathBuf, PathBuf), automatic: bool) {
    java::config::save_java_to_config(paths, automatic).unwrap();
}

#[tauri::command]
pub fn get_java_from_config() -> Result<JavaConfig, ()> {
    let java_config = java::config::get_java_from_config().unwrap();
    Ok(java_config)
}

#[tauri::command]
pub async fn get_instances(state: State<'_, AppState>) -> Result<Vec<Instance>, ()> {
    let instances_lock = state.instances.lock().await;
    Ok(instances_lock.get_instances().to_vec())
}

#[tauri::command]
pub async fn get_instance(state: State<'_, AppState>, slug: &str) -> Result<Instance, ()> {
    let instances_lock = state.instances.lock().await;
    Ok(instances_lock.get_instance(slug).unwrap())
}

#[tauri::command]
pub async fn create_instance(
    state: State<'_, AppState>,
    handle: AppHandle,
    instance: Instance,
) -> Result<(), ()> {
    let mut instances_lock = state.instances.lock().await;
    instances_lock
        .add_instance(&state, &handle, instance)
        .await
        .unwrap();
    Ok(())
}

#[tauri::command]
pub async fn update_instance(
    state: State<'_, AppState>,
    handle: AppHandle,
    instance: Instance,
) -> Result<(), ()> {
    let mut instances_lock = state.instances.lock().await;
    instances_lock.update_instance(&handle, instance).unwrap();
    Ok(())
}

#[tauri::command]
pub async fn delete_instance(
    state: State<'_, AppState>,
    handle: AppHandle,
    slug: String,
) -> Result<(), ()> {
    let mut instances_lock = state.instances.lock().await;
    instances_lock.delete_instance(&handle, &slug).unwrap();
    Ok(())
}

#[tauri::command]
pub async fn launch_instance(
    state: State<'_, AppState>,
    handle: AppHandle,
    slug: String,
) -> Result<(), ()> {
    if let Err(e) = resources::launch::launch(state, handle, &slug).await {
        error!("Error launching instance: {:?}", e);
        return Err(());
    }

    Ok(())
}

/* #[tauri::command]
pub async fn kill_instance(
    state: State<'_, AppState>,
    handle: AppHandle,
    slug: String,
) -> Result<(), ()> {
    match resources::launch::kill_instance(&state, &handle, &slug).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Failed to kill instance {}: {}", slug, e);
            Err(())
        }
    }
} */

#[tauri::command]
pub async fn get_versions(state: State<'_, AppState>) -> Result<Vec<Version>, ()> {
    let versions = resources::versions::get_versions(state).await.unwrap();
    Ok(versions)
}

#[tauri::command]
pub fn set_discord_activity(
    state: State<'_, AppState>,
    details: String,
    status: String,
) -> Result<(), ()> {
    discord::set_activity(&state.discord_client, details, status);
    Ok(())
}

#[tauri::command]
pub fn toggle_discord_rpc(state: State<'_, AppState>, enabled: bool) -> Result<(), ()> {
    discord::toggle_rpc(&state.discord_client, enabled);
    Ok(())
}

#[tauri::command]
pub fn get_launcher_settings() -> Result<LauncherSettings, String> {
    let config = config::get_config().map_err(|e| e.to_string())?;
    Ok(LauncherSettings {
        rich_presence: config.rich_presence,
        use_discrete_gpu: config.use_discrete_gpu,
    })
}

#[tauri::command]
pub fn save_launcher_settings(handle: AppHandle, settings: LauncherSettings) -> Result<(), String> {
    config::update_launcher_settings(&handle, &settings).map_err(|e| e.to_string())?;

    let discord_client = &handle.state::<AppState>().discord_client;
    discord::toggle_rpc(discord_client, settings.rich_presence);

    Ok(())
}

#[tauri::command]
pub async fn get_avatar(state: State<'_, AppState>, uuid: String) -> Result<String, String> {
    match auth::avatar::get_cached_avatar(&state, uuid).await {
        Ok(avatar) => Ok(avatar),
        Err(e) => {
            error!("Failed to get avatar: {}", e);
            Err(e.to_string())
        }
    }
}
