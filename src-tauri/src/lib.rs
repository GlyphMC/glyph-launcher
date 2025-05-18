use std::{collections::HashMap, process, sync::Arc};

use auth::auth::LoginHandle;
use discord_rich_presence::DiscordIpcClient;
use instance::InstanceConfig;
use log::{error, info};
use reqwest::Client;
use specta_typescript::{BigIntExportBehavior, Typescript};
use tauri::{Manager, WindowEvent, Wry};
use tauri_specta::{Builder, collect_commands, collect_events};
use tokio::sync::Mutex;

mod auth;
mod commands;
mod config;
mod discord;
mod instance;
mod java;
mod resources;

pub type ProcessHandle = Arc<Mutex<Option<process::Child>>>;
pub type RunningInstancesMap = HashMap<String, ProcessHandle>;

pub struct AppState {
    client: Arc<Mutex<Client>>,
    instances: Arc<Mutex<InstanceConfig>>,
    login_handle: LoginHandle,
    discord_client: Arc<Mutex<Option<DiscordIpcClient>>>,
    running_instances: Arc<Mutex<RunningInstancesMap>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::<Wry>::new()
        .commands(collect_commands![
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
            // commands::kill_instance,
            commands::get_versions,
            commands::set_discord_activity,
            commands::toggle_discord_rpc,
            commands::get_launcher_settings,
            commands::save_launcher_settings,
            commands::get_avatar,
        ])
        .events(collect_events![
            auth::auth::LoginDetailsEvent,
            instance::InstanceListUpdatedEvent,
            java::download::JavaDownloadStartedEvent,
            java::download::JavaDownloadProgressEvent,
            java::download::JavaDownloadFinishedEvent,
            java::extract::JavaExtractStartedEvent,
            java::extract::JavaExtractProgressEvent,
            java::extract::JavaExtractFinishedEvent,
            resources::assets::AssetProgressEvent,
            resources::launch::AssetsDownloadStartedEvent,
            resources::launch::AssetsDownloadFinishedEvent,
            resources::launch::InstanceStartedEvent,
            resources::launch::InstanceStoppedEvent,
            resources::launch::InstanceLogEvent,
        ]);

    #[cfg(debug_assertions)]
    builder
        .export(
            Typescript::default().bigint(BigIntExportBehavior::Number),
            "../src/lib/bindings.ts",
        )
        .expect("Failed to export TS bindings");

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
            let discord_client = Arc::new(Mutex::new(None));
            let running_instances = Arc::new(Mutex::new(HashMap::new()));

            AppState {
                client,
                instances,
                login_handle,
                discord_client,
                running_instances,
            }
        })
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);

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
            info!(
                "Initial config loaded. Rich Presence enabled: {}",
                config.rich_presence
            );

            let discord_client_state = &handle.state::<AppState>().discord_client;

            if config.rich_presence {
                let discord_client_arc_clone = discord_client_state.clone();
                tauri::async_runtime::spawn(async move {
                    if let Err(e) = discord::connect(&discord_client_arc_clone).await {
                        error!("Failed to connect to Discord RPC: {:?}", e);
                    }
                });
            }

            info!("Completed onboarding: {}", config.completed_onboarding);
            let location = if config.completed_onboarding {
                "launcher".to_string()
            } else {
                "onboarding".to_string()
            };

            window
                .eval(format!("window.location.href = '/#/{}'", location).as_str())
                .unwrap();

            let discord_client_state = handle.state::<AppState>().discord_client.clone();
            window.on_window_event(move |event| {
                if let WindowEvent::CloseRequested { .. } = event {
                    discord::close_rpc(&discord_client_state);
                }
            });

            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let state = handle.state::<AppState>();
                auth::auth::refresh(&*state.client.lock().await).await
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running Tauri Application");
}
