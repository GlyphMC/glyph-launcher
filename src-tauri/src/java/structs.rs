use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Debug)]
pub struct JavaInfo {
    availability_type: String,
    distro_version: Vec<u32>,
    pub download_url: String,
    java_version: Vec<u32>,
    latest: bool,
    name: String,
    openjdk_build_number: u32,
    package_uuid: String,
    product: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Progress {
    pub percentage: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct JavaConfig {
    pub java_8_path: String,
    pub java_17_path: String,
    pub java_21_path: String,
}

impl JavaConfig {
    pub fn default() -> Self {
        Self {
            java_8_path: String::new(),
            java_17_path: String::new(),
            java_21_path: String::new(),
        }
    }
}
