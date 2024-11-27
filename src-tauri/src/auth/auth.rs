use std::{
    collections::HashMap,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::{Error, Ok, Result};
use log::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::{AppHandle, Emitter, State};
use tokio::time::sleep;

use crate::{auth::account::Profile, config, AppState};

use super::{
    account::Account,
    structs::{
        AuthorizationTokenResponse, DeviceCodeResponse, MinecraftAuthenticationResponse,
        MinecraftProfileResponse, RefreshTokenResponse, XboxLiveAuthenticationResponse,
    },
};

const CLIENT_ID: &str = "04bc8538-fc3c-4490-9e61-a2b3f4cbcf5c";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LoginDetails<'a> {
    code: &'a str,
    uri: &'a str,
}

pub async fn login(state: &State<'_, AppState>, handle: AppHandle) -> Result<MinecraftProfileResponse, Error> {
    let client = state.client.lock().await;

    let device_code_response = device_response(&client).await?;
    let device_response = device_code_response;
    let login_details = LoginDetails {
        code: &device_response.user_code,
        uri: &device_response.verification_uri,
    };

    handle.emit("login-details", login_details)?;

    let mut authentication_response: Option<AuthorizationTokenResponse> = None;
    while authentication_response.is_none() {
        match authorization_token_response(&device_response.device_code, &client).await {
            Result::Ok(token_response) => {
                authentication_response = Some(token_response);
                info!("Received authentication token");
            }
            Result::Err(e) => {
                info!("Failed to receive authentication token: {}", e);
                sleep(Duration::from_secs(device_response.interval)).await;
            }
        }
    }

    let auth_response = authentication_response.unwrap();
    let xbox_response = xbox_response(&auth_response.access_token, &client).await?;
    let xbox_security_token_response =
        xbox_security_token_response(xbox_response.token, &client).await?;
    let minecraft_response = minecraft_response(
        xbox_security_token_response.display_claims,
        xbox_security_token_response.token,
        &client,
    )
    .await?;
    let minecraft_profile_response =
        minecraft_profile_response(minecraft_response.access_token, &client).await?;

    let minecraft_profile_response_clone = minecraft_profile_response.clone();

    let expires_in = Duration::from_secs(auth_response.expires_in);
    let system_time = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let combined_duration = system_time + expires_in;
    let combined_timestamp = combined_duration.as_secs();

    let account = Account {
        active: true,
        expiry: combined_timestamp,
        access_token: auth_response.access_token,
        refresh_token: auth_response.refresh_token,
        profile: minecraft_profile_response_clone.into(),
    };

    let mut config = config::get_config()?;
    let accounts = &mut config.accounts;
    let default_account = Account::default();

    if accounts.is_empty() {
        accounts.push(account);
    } else {
        accounts.iter_mut().for_each(|acc| acc.active = false);
        accounts.retain(|acc| acc != &default_account);
        accounts.push(account);
    }

    config::save_config(&config)?;

    Ok(minecraft_profile_response)
}

pub async fn refresh(client: &Client) -> Result<(), Error> {
    let config = config::get_config()?;
    let default_account = Account::default();
    if config.accounts.len() == 1 && config.accounts[0] == default_account {
        info!("No accounts to refresh");
        return Ok(());
    }
    for account in config.accounts {
        let expiry = Duration::from_secs(account.expiry);
        let system_time = SystemTime::now().duration_since(UNIX_EPOCH)?;

        if expiry <= system_time {
            info!("Refreshing token for account: {}", account.profile.name);

            let refresh_token_response =
                refresh_token_response(&account.refresh_token, client).await?;
            let xbox_response = xbox_response(&refresh_token_response.access_token, client).await?;
            let xbox_security_token_response =
                xbox_security_token_response(xbox_response.token, client).await?;
            let minecraft_response = minecraft_response(
                xbox_security_token_response.display_claims,
                xbox_security_token_response.token,
                client,
            )
            .await?;
            let minecraft_profile_response =
                minecraft_profile_response(minecraft_response.access_token, client).await?;

            let expires_in = Duration::from_secs(refresh_token_response.expires_in.into());
            let system_time = SystemTime::now().duration_since(UNIX_EPOCH)?;
            let combined_duration = system_time + expires_in;
            let combined_timestamp = combined_duration.as_secs();

            let profile: Profile = minecraft_profile_response.clone().into();

            let new_account = Account {
                active: account.active,
                expiry: combined_timestamp,
                access_token: refresh_token_response.access_token,
                refresh_token: account.refresh_token,
                profile,
            };

            let mut config = config::get_config()?;
            config
                .accounts
                .retain(|acc| acc.profile.id != new_account.profile.id);
            config.accounts.push(new_account);
            config::save_config(&config)?;

            info!("Token refreshed for account: {}", account.profile.name);
        } else {
            info!("Token for account: {} is still valid", account.profile.name);
        }
    }

    Ok(())
}

async fn device_response(client: &Client) -> Result<DeviceCodeResponse> {
    let response = client
        .get("https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode")
        .form(&vec![
            ("client_id", CLIENT_ID),
            ("response_type", "code"),
            ("scope", "XboxLive.signin offline_access"),
        ])
        .send()
        .await?
        .json::<DeviceCodeResponse>()
        .await?;

    Ok(response)
}

async fn authorization_token_response(
    device_code: &str,
    client: &Client,
) -> Result<AuthorizationTokenResponse> {
    let response = client
        .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
        .form(&vec![
            ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
            ("client_id", CLIENT_ID),
            ("device_code", &device_code),
        ])
        .send()
        .await?
        .json::<AuthorizationTokenResponse>()
        .await?;

    Ok(response)
}

async fn refresh_token_response(
    refresh_token: &str,
    client: &Client,
) -> Result<RefreshTokenResponse> {
    let response = client
        .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
        .form(&vec![
            ("grant_type", "refresh_token"),
            ("client_id", CLIENT_ID),
            ("refresh_token", &refresh_token),
        ])
        .send()
        .await?
        .json::<RefreshTokenResponse>()
        .await?;
    Ok(response)
}

async fn xbox_response(
    access_token: &str,
    client: &Client,
) -> Result<XboxLiveAuthenticationResponse> {
    let response = client
        .post("https://user.auth.xboxlive.com/user/authenticate")
        .json(&json!({
                "Properties": {
                "AuthMethod": "RPS",
                "SiteName": "user.auth.xboxlive.com",
                "RpsTicket": &format!("d={}", access_token)
            },
            "RelyingParty": "http://auth.xboxlive.com",
            "TokenType": "JWT"
        }))
        .send()
        .await?
        .json::<XboxLiveAuthenticationResponse>()
        .await?;

    Ok(response)
}

async fn xbox_security_token_response(
    token: String,
    client: &Client,
) -> Result<XboxLiveAuthenticationResponse> {
    let response = client
        .post("https://xsts.auth.xboxlive.com/xsts/authorize")
        .json(&json!({
            "Properties": {
                "SandboxId": "RETAIL",
                "UserTokens": [&token]
            },
            "RelyingParty": "rp://api.minecraftservices.com/",
            "TokenType": "JWT"
        }))
        .send()
        .await?
        .json::<XboxLiveAuthenticationResponse>()
        .await?;

    Ok(response)
}

async fn minecraft_response(
    display_claims: HashMap<String, Vec<HashMap<String, String>>>,
    token: String,
    client: &Client,
) -> Result<MinecraftAuthenticationResponse, Error> {
    let response = client
        .post("https://api.minecraftservices.com/authentication/login_with_xbox")
        .json(&json!({
            "identityToken": &format!("XBL3.0 x={};{}", &display_claims["xui"][0]["uhs"], token)
        }))
        .send()
        .await?
        .json::<MinecraftAuthenticationResponse>()
        .await?;

    Ok(response)
}

async fn minecraft_profile_response(
    access_token: String,
    client: &Client,
) -> Result<MinecraftProfileResponse, Error> {
    let response = client
        .get("https://api.minecraftservices.com/minecraft/profile")
        .bearer_auth(&access_token)
        .send()
        .await?
        .json::<MinecraftProfileResponse>()
        .await?;

    Ok(response)
}
