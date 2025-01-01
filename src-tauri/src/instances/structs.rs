use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InstanceConfig {
    pub instances: Vec<Instance>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Instance {
    pub slug: String,
    pub name: String,
    pub game: Game,
    pub java: Java,
    pub settings: Settings,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    pub version: String,
    modloader: Modloader,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Modloader {
    loader: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Java {
    pub path: String,
    pub args: Vec<String>,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub has_launched: bool,
    rich_presence: bool,
}
