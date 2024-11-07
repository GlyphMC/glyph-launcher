use std::sync::Arc;

use log::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::Manager;
use tokio::sync::Mutex;

mod auth;
mod commands;
mod config;
mod instances;
mod java;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Payload<'a> {
    message: &'a str,
}

pub struct AppState {
    client: Arc<Mutex<Client>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage({
            let client = Arc::new(Mutex::new(Client::new()));
            AppState { client }
        })
        .setup(|app| {
            let handle = app.handle();
            let window = handle.get_webview_window("main").unwrap();

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
                window.open_devtools();
            }

            let is_first_launch = !config::config_file_exists()?;
            info!("First launch: {}", is_first_launch);
            if is_first_launch {
                config::create_default_config_file()?;
            }

            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let state = handle.state::<AppState>();
                let binding = state.client.clone();
                let client = binding.lock().await;
                auth::auth::refresh(&client).await
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::login,
            commands::get_minecraft_profiles,
			commands::download_java,
			commands::extract_java,
			commands::save_java_to_config,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri Application");
}
