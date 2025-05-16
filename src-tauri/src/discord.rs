use std::sync::Arc;

use discord_rich_presence::{DiscordIpc, DiscordIpcClient, activity::Activity};
use log::{error, info};
use tokio::sync::Mutex;

pub const DISCORD_APP_ID: &str = "1112468903336083477";

pub fn connect(discord_client_state: &Arc<Mutex<Option<DiscordIpcClient>>>) {
    let discord_client_state = discord_client_state.clone();
    tauri::async_runtime::spawn_blocking(move || match DiscordIpcClient::new(DISCORD_APP_ID) {
        Ok(mut client) => {
            if let Err(e) = client.connect() {
                error!("Failed to connect to Discord RPC: {}", e);
                return;
            }
            info!("Discord RPC connected successfully.");
            let mut discord_guard = discord_client_state.blocking_lock();
            *discord_guard = Some(client);
            drop(discord_guard);

            set_initial_activity(&discord_client_state);
        }
        Err(e) => {
            error!("Failed to create Discord IPC client: {:?}", e);
        }
    });
}

fn set_initial_activity(discord_client_state: &Arc<Mutex<Option<DiscordIpcClient>>>) {
    set_activity(
        discord_client_state,
        "Exploring the Launcher".to_string(),
        "Idle".to_string(),
    );
}

pub fn set_activity(
    discord_client_state: &Arc<Mutex<Option<DiscordIpcClient>>>,
    details: String,
    state: String,
) {
    let discord_client_state = discord_client_state.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let mut guard = discord_client_state.blocking_lock();
        if let Some(client) = guard.as_mut() {
            let activity = Activity::new().details(&details).state(&state);
            if let Err(e) = client.set_activity(activity) {
                error!("Failed to set Discord activity: {}", e);
            } else {
                info!("Discord activity set: {} - {}", details, state);
            }
        } else {
            error!("Discord client not connected, cannot set activity.");
        }
    });
}

pub fn close_rpc(discord_client_state: &Arc<Mutex<Option<DiscordIpcClient>>>) {
    let discord_client_state = discord_client_state.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let mut guard = discord_client_state.blocking_lock();
        if let Some(mut client) = guard.take() {
            let _ = client.close();
            info!("Discord RPC connection closed.");
        }
    });
}

pub fn toggle_rpc(discord_client_state: &Arc<Mutex<Option<DiscordIpcClient>>>, enabled: bool) {
    let guard = discord_client_state.blocking_lock();
    if enabled {
        if guard.is_none() {
            // Only connect if not already connected
            drop(guard);
            info!("RPC toggled on. Attempting to connect.");
            connect(discord_client_state);
        } else {
            info!("RPC toggled on, but already connected.");
        }
    } else if guard.is_some() {
        // Only close if connected
        drop(guard);
        info!("RPC toggled off. Attempting to close connection.");
        close_rpc(discord_client_state);
    } else {
        info!("RPC toggled off, but already disconnected.");
    }
}
