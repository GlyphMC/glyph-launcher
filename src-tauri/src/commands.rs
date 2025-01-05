use std::path::PathBuf;

use anyhow::Result;
use tauri::{AppHandle, State};

use crate::{
    auth::{
        self,
        account::{Account, Profile},
    },
    config,
    instances::{
        self,
        structs::{Instance, InstanceConfig},
    },
    java::{self, structs::JavaConfig, test::JavaTestInfo},
    resources::{self, versions::Version},
    AppState,
};

#[tauri::command]
pub async fn login(state: State<'_, AppState>, handle: AppHandle) -> Result<Profile, ()> {
    let login_handle = state.login_handle.clone();
    let profile = auth::auth::login(&state, handle, login_handle)
        .await
        .unwrap();
    Ok(profile.into())
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
pub fn get_instances() -> Result<InstanceConfig, ()> {
    let instance_config = instances::instance::get_instances().unwrap();
    Ok(instance_config)
}

#[tauri::command]
pub fn get_instance(slug: String) -> Result<Instance, ()> {
    let instance = instances::instance::get_instance(&slug).unwrap();
    Ok(instance)
}

#[tauri::command]
pub async fn create_instance(
    state: State<'_, AppState>,
    handle: AppHandle,
    instance: Instance,
) -> Result<(), ()> {
    if let Err(e) = instances::instance::create_instance(state, handle, instance).await {
        eprintln!("Error creating instance: {:?}", e);
        return Err(());
    }

    Ok(())
}

#[tauri::command]
pub fn update_instance(instance: Instance) -> Result<(), ()> {
    if let Err(e) = instances::instance::update_instance(instance) {
        eprintln!("Error updating instance: {:?}", e);
        return Err(());
    }

    Ok(())
}

#[tauri::command]
pub fn delete_instance(handle: AppHandle, slug: String) -> Result<(), ()> {
    if let Err(e) = instances::instance::delete_instance(handle, slug) {
        eprintln!("Error deleting instance: {:?}", e);
        return Err(());
    }

    Ok(())
}

#[tauri::command]
pub async fn launch_instance(state: State<'_, AppState>, slug: String) -> Result<(), ()> {
    if let Err(e) = resources::launch::launch(state, slug).await {
        eprintln!("Error launching instance: {:?}", e);
        return Err(());
    }

    Ok(())
}

#[tauri::command]
pub async fn get_versions(state: State<'_, AppState>) -> Result<Vec<Version>, ()> {
    let versions = resources::versions::get_versions(state).await.unwrap();
    Ok(versions)
}
