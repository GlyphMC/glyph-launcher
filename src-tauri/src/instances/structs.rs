use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InstanceConfig {
    pub instances: Vec<Instance>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Instance {
    pub slug: String,
    pub name: String,
    pub game: Game,
    pub java: Java,
    pub settings: Settings,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub version: String,
    modloader: Modloader,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Modloader {
    loader: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Java {
    pub path: String,
    pub args: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub has_launched: bool,
    rich_presence: bool,
    maximised: bool,
    memory: u64,
}
