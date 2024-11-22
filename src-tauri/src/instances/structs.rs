use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InstanceConfig {
	pub instances: Vec<Instance>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Instance {
    pub slug: String,
    pub name: String,
    game: Game,
    java: Java,
	settings: Settings
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    version: String,
    modloader: Modloader,
}

#[derive(Serialize, Deserialize, Debug)]
struct Modloader {
    loader: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Java {
    path: String,
    args: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    has_launched: bool,
	rich_presence: bool,
    maximised: bool,
    memory: u64,
}
