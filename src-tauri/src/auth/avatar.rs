use anyhow::{Error, Result};
use base64::Engine;
use log::{error, info};
use tauri::State;

use crate::AppState;

pub async fn get_cached_avatar(state: &State<'_, AppState>, uuid: String) -> Result<String, Error> {
    info!("Fetching avatar from Crafatar for UUID: {}...", uuid);
    let url = format!("https://crafatar.com/avatars/{}?size=64&overlay=true", uuid);
    let client = state.client.lock().await;

    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.bytes().await {
                    Ok(bytes) => {
                        let base64_image = base64::engine::general_purpose::STANDARD.encode(bytes);
                        Ok(format!("data:image/png;base64,{}", base64_image))
                    }
                    Err(e) => {
                        error!(
                            "Failed to get bytes from Crafatar response for {}: {}",
                            uuid, e
                        );
                        Err(e.into())
                    }
                }
            } else {
                let status = response.status();
                error!(
                    "Crafatar request failed for {} with status: {}",
                    uuid, status
                );
                Err(Error::msg(format!(
                    "Crafatar request failed with status: {}",
                    status
                )))
            }
        }
        Err(e) => {
            error!("Failed to download avatar for {}: {}", uuid, e);
            Err(Error::msg(format!(
                "Failed to download avatar for {}: {}",
                uuid, e
            )))
        }
    }
}
