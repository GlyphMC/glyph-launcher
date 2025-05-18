use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Type)]
pub struct Skin {
    id: String,
    state: String,
    url: String,
    variant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Type)]
pub struct Cape {
    id: String,
    state: String,
    url: String,
    alias: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Type)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub active: bool,
    #[specta(type = String)]
    pub expiry: u64,
    pub access_token: String,
    pub refresh_token: String,
    pub profile: Profile,
}

impl Default for Account {
    fn default() -> Self {
        Account {
            active: false,
            expiry: 0,
            access_token: String::new(),
            refresh_token: String::new(),
            profile: Profile {
                id: String::new(),
                name: String::new(),
                skins: Vec::new(),
                capes: Vec::new(),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Type)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub skins: Vec<Skin>,
    pub capes: Vec<Cape>,
}
