use serde::{Deserialize, Serialize};

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
