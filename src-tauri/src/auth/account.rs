use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Skin {
    id: String,
    state: String,
    url: String,
    variant: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Cape {
    id: String,
    state: String,
    url: String,
    alias: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub active: bool,
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub skins: Vec<Skin>,
    pub capes: Vec<Cape>,
}
