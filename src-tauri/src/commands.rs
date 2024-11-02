use anyhow::Result;
use tauri::{AppHandle, State};

use crate::{
    auth::{self, account::Profile},
    config, AppState,
};

#[tauri::command]
pub async fn login(state: State<'_, AppState>, handle: AppHandle) -> Result<Profile, ()> {
    let client = state.client.lock().await;
    let profile = auth::auth::login(&client, handle).await.unwrap();
    Ok(profile.into())
}

#[tauri::command]
pub async fn get_minecraft_profiles() -> Result<Vec<Profile>, ()> {
    let config = config::get_config().unwrap();
    let accounts = config.accounts;
    let mut profiles = Vec::new();
    for account in accounts {
        profiles.push(account.profile);
    }
    Ok(profiles)
}
