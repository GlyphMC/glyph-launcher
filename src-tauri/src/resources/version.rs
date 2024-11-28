use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::AppState;

#[derive(Serialize, Deserialize, Debug)]
pub struct Arguments {
    pub game: Vec<GameArgument>,
    pub jvm: Vec<JvmArgument>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum GameArgument {
    String(String),
    Object(GameArgumentObject),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameArgumentObject {
    pub rules: Vec<Rule>,
    pub value: Value,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum JvmArgument {
    String(String),
    Object(JvmArgumentObject),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JvmArgumentObject {
    pub rules: Vec<Rule>,
    pub value: Value,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Array(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rule {
    pub action: String,
    pub features: Option<Features>,
    pub os: Option<Os>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Features {
    pub is_demo_user: Option<bool>,
    pub has_custom_resolution: Option<bool>,
    pub has_quick_plays_support: Option<bool>,
    pub is_quick_play_singleplayer: Option<bool>,
    pub is_quick_play_multiplayer: Option<bool>,
    pub is_quick_play_realms: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Os {
    pub name: Option<String>,
    pub arch: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: u64,
	#[serde(rename = "totalSize")]
    pub total_size: u64,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Downloads {
    pub client: Download,
    pub client_mappings: Download,
    pub server: Download,
    pub server_mappings: Download,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Download {
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JavaVersion {
    pub component: String,
	#[serde(rename = "majorVersion")]
    pub major_version: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Library {
    pub downloads: LibraryDownloads,
    pub name: String,
    pub rules: Option<Vec<Rule>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibraryDownloads {
    pub artifact: Artifact,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Artifact {
    pub path: String,
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Logging {
    pub client: LoggingClient,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggingClient {
    pub argument: String,
    pub file: LoggingFile,
    #[serde(rename = "type")]
    pub log_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggingFile {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionManifest {
    pub arguments: Arguments,
	#[serde(rename = "assetIndex")]
    pub asset_index: AssetIndex,
    pub assets: String,
	#[serde(rename = "complianceLevel")]
    pub compliance_level: u32,
    pub downloads: Downloads,
    pub id: String,
	#[serde(rename = "javaVersion")]
    pub java_version: JavaVersion,
    pub libraries: Vec<Library>,
    pub logging: Logging,
	#[serde(rename = "mainClass")]
    pub main_class: String,
	#[serde(rename = "minimumLauncherVersion")]
    pub minimum_launcher_version: u32,
	#[serde(rename = "releaseTime")]
    pub release_time: String,
    pub time: String,
    pub r#type: String,
}

pub async fn get_version_manifest(state: State<'_, AppState>, url: &String) -> Result<VersionManifest, Error> {
	let client = state.client.lock().await;
	let response = client
		.get(url)
		.send()
		.await?
		.json::<VersionManifest>()
		.await?;

	Ok(response)
}
