use std::sync::Arc;

use anyhow::{Error, anyhow};
use discord_rich_presence::{DiscordIpc, DiscordIpcClient, activity::Activity};
use log::{error, info};
use tokio::sync::Mutex;

pub const DISCORD_APP_ID: &str = "1112468903336083477";

pub async fn connect(
    discord_client_state: &Arc<Mutex<Option<DiscordIpcClient>>>,
) -> Result<(), Error> {
    let discord_client_state = discord_client_state.clone();

    let handle = tauri::async_runtime::spawn_blocking(move || -> Result<(), Error> {
        let mut client = DiscordIpcClient::new(DISCORD_APP_ID)
            .map_err(|e| anyhow!("Failed to create Discord IPC client: {:?}", e))?;

        client
            .connect()
            .map_err(|e| anyhow!("Failed to connect to Discord RPC: {}", e))?;

        info!("Discord RPC connected successfully.");
        {
            let mut discord_guard = discord_client_state.blocking_lock();
            *discord_guard = Some(client);
        }

        set_initial_activity(&discord_client_state)?;

        Ok(())
    });

    handle
        .await
        .map_err(|e| anyhow!("Join error while connecting to Discord RPC: {}", e))?
}

fn set_initial_activity(
    discord_client_state: &Arc<Mutex<Option<DiscordIpcClient>>>,
) -> Result<(), Error> {
    // You can't `.await` inside a `spawn_blocking`, so we block manually here.
    tauri::async_runtime::block_on(set_activity(
        discord_client_state,
        "Exploring the Launcher".to_string(),
        "Idle".to_string(),
    ))
}

pub async fn set_activity(
    discord_client_state: &Arc<Mutex<Option<DiscordIpcClient>>>,
    details: String,
    state: String,
) -> Result<(), Error> {
    let discord_client_state = discord_client_state.clone();

    let result = tauri::async_runtime::spawn_blocking(move || -> Result<(), Error> {
        let mut guard = discord_client_state.blocking_lock();

        if let Some(client) = guard.as_mut() {
            let activity = Activity::new().details(&details).state(&state);
            client.set_activity(activity).map_err(|e| {
                error!("Failed to set Discord activity: {}", e);
                anyhow!("Failed to set activity: {}", e)
            })?;
            info!("Discord activity set: {} - {}", details, state);
            Ok(())
        } else {
            error!("Discord client not connected, cannot set activity.");
            Err(anyhow!("Discord client not connected"))
        }
    })
    .await;

    result.map_err(|e| anyhow!("Join error while setting activity: {}", e))?
}

pub fn close_rpc(discord_client_state: &Arc<Mutex<Option<DiscordIpcClient>>>) {
    let discord_client_state = discord_client_state.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let mut guard = discord_client_state.blocking_lock();
        if let Some(mut client) = guard.take() {
            if let Err(e) = client.close() {
                error!("Error while closing Discord RPC connection: {}", e);
            } else {
                info!("Discord RPC connection closed.");
            }
        } else {
            info!("No Discord RPC connection to close.");
        }
    });
}

pub async fn toggle_rpc(
    discord_client_state: &Arc<Mutex<Option<DiscordIpcClient>>>,
    enabled: bool,
) -> Result<(), Error> {
    let guard = discord_client_state.blocking_lock();

    if enabled {
        if guard.is_none() {
            drop(guard);
            info!("RPC toggled on. Attempting to connect.");
            connect(discord_client_state).await.map_err(|e| {
                error!("Failed to connect Discord RPC: {}", e);
                e
            })?;
        } else {
            info!("RPC toggled on, but already connected.");
        }
    } else if guard.is_some() {
        drop(guard);
        info!("RPC toggled off. Attempting to close connection.");
        close_rpc(discord_client_state);
    } else {
        info!("RPC toggled off, but already disconnected.");
    }

    Ok(())
}
