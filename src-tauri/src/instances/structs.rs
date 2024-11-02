use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Instance {
    slug: String,
    name: String,
    game: Game,
    java: Java,
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
pub struct InstanceConfig {
    has_launched: bool,
    minimized: bool,
    memory: u64,
}
