use std::sync::Arc;

use auth::auth::LoginHandle;
use instance::InstanceConfig;
use log::{error, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::Manager;
use tokio::sync::Mutex;

mod auth;
mod commands;
mod config;
mod instance;
mod java;
mod resources;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Payload<'a> {
    message: &'a str,
}

pub struct AppState {
    client: Arc<Mutex<Client>>,
    instances: Arc<Mutex<InstanceConfig>>,
    login_handle: LoginHandle,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("Failed to get main window")
                .set_focus();
        }))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage({
            let client = Arc::new(Mutex::new(Client::new()));
            let instances = Arc::new(Mutex::new(
                InstanceConfig::read_from_file()
                    .unwrap_or_else(|_| InstanceConfig { instances: vec![] }),
            ));
            let login_handle = LoginHandle::new();

            AppState {
                client,
                instances,
                login_handle,
            }
        })
        .setup(|app| {
            let handle = app.handle();
            let window = handle.get_webview_window("main").unwrap();

            #[cfg(debug_assertions)]
            {
                handle.plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;

                window.open_devtools();
            }

            let first_launch = !config::config_file_exists()?;

            if first_launch {
                config::create_default_config_file()?;
                if let Err(err) = InstanceConfig::create_default_file() {
                    error!("Failed to create default instance config file: {:?}", err);
                }
            }

            let config = config::get_config()?;
            info!("Completed onboarding: {}", config.completed_onboarding);
            let location = if config.completed_onboarding {
                "launcher".to_string()
            } else {
                "onboarding".to_string()
            };

            window
                .eval(format!("window.location.href = '/#/{}'", location).as_str())
                .unwrap();

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
            commands::cancel_login,
            commands::set_onboarding_complete,
            commands::switch_account,
            commands::delete_account,
            commands::get_active_account,
            commands::get_minecraft_profiles,
            commands::download_java,
            commands::extract_java,
            commands::test_java,
            commands::save_java_to_config,
            commands::get_java_from_config,
            commands::get_instances,
            commands::get_instance,
            commands::create_instance,
            commands::update_instance,
            commands::delete_instance,
            commands::launch_instance,
            commands::get_versions,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri Application");
}
