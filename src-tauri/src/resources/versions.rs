use anyhow::{Error, Result};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::AppState;

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionsManifest {
    latest: Latest,
    versions: Vec<Version>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Latest {
    release: String,
    snapshot: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    id: String,
    r#type: String,
    url: String,
    time: String,
    #[serde(rename = "releaseTime")]
    release_time: String,
    sha1: String,
    #[serde(rename = "complianceLevel")]
    compliance_level: i32,
}

async fn get_versions_manifest(state: State<'_, AppState>) -> Result<VersionsManifest, Error> {
    let client = state.client.lock().await;
    let response = client
        .get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
        .send()
        .await?
        .text()
        .await?;

    Ok(serde_json::from_str(&response)?)
}

pub async fn get_versions(state: State<'_, AppState>) -> Result<Vec<Version>, Error> {
    let manifest = get_versions_manifest(state).await?;
    let mut versions = manifest.versions;
    versions.sort_by(|a, b| {
        let time_a = DateTime::parse_from_rfc3339(&a.release_time).unwrap();
        let time_b = DateTime::parse_from_rfc3339(&b.release_time).unwrap();
        time_b.cmp(&time_a)
    });

    Ok(versions)
}
