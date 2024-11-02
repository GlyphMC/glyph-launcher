use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::account::{Cape, Profile, Skin};

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceCodeResponse {
    pub user_code: String,
    pub device_code: String,
    pub verification_uri: String,
    expires_in: u32,
    pub interval: u64,
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizationTokenResponse {
    token_type: String,
    scope: String,
    pub expires_in: u64,
    ext_expires_in: u32,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    refresh_token: String,
    scope: String,
    pub expires_in: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct XboxLiveAuthenticationResponse {
    issue_instant: String,
    not_after: String,
    pub token: String,
    pub display_claims: HashMap<String, Vec<HashMap<String, String>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MinecraftAuthenticationResponse {
    username: String,
    pub access_token: String,
    token_type: String,
    expires_in: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MinecraftProfileResponse {
    id: String,
    name: String,
    skins: Vec<Skin>,
    capes: Vec<Cape>,
}

impl Into<Profile> for MinecraftProfileResponse {
    fn into(self) -> Profile {
        Profile {
            id: self.id,
            name: self.name,
            skins: self.skins,
            capes: self.capes,
        }
    }
}
