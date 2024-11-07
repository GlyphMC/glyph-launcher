use std::path::PathBuf;

use anyhow::Result;
use tauri::{AppHandle, State};

use crate::{
    auth::{
        self,
        account::{Account, Profile},
    }, config, java, AppState
};

#[tauri::command]
pub async fn login(state: State<'_, AppState>, handle: AppHandle) -> Result<Profile, ()> {
    let profile = auth::auth::login(&state, handle).await.unwrap();
    Ok(profile.into())
}

#[tauri::command]
pub async fn get_minecraft_profiles() -> Result<Vec<Profile>, ()> {
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
pub async fn download_java(state: State<'_, AppState>, handle: AppHandle) -> Result<(PathBuf, PathBuf, PathBuf), ()> {
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
pub async fn save_java_to_config(paths: (PathBuf, PathBuf, PathBuf)) {
	java::config::save_java_to_config(paths).unwrap();
}
