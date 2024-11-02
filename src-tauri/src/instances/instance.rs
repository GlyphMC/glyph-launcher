use serde::{Deserialize, Serialize};

use super::structs::Instance;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InstanceConfig {
    has_launched: bool,
    rich_presence: bool,
    minimized: bool,
    memory: String,
}

pub fn make_instance(instance: Instance) {}
