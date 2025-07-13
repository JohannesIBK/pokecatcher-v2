use std::ops::Deref;

use anyhow::{Context, Result};
use config_handler::AuthConfig;
use tokio::process::Command;
use twitch_oauth2::{
    AccessToken, ClientId, DeviceUserTokenBuilder, RefreshToken, Scope, TwitchToken, UserToken,
};

pub async fn get_user_token(client_id: &str, token: Option<AuthConfig<'_>>) -> Result<UserToken> {
    let http_client = reqwest::Client::new();

    let client_id = ClientId::from(client_id);

    let Some(AuthConfig {
        access_token,
        refresh_token,
    }) = token
    else {
        return request_new_token(client_id, &http_client).await;
    };

    if let Ok(token) = UserToken::from_existing(
        &http_client,
        AccessToken::from(access_token.deref()),
        RefreshToken::from(refresh_token.deref()),
        None,
    )
    .await
    {
        return Ok(token);
    }

    tracing::info!("Refreshing token");

    let mut token = UserToken::from_existing_unchecked(
        AccessToken::from(access_token.deref()),
        Some(RefreshToken::from(refresh_token.deref())),
        client_id,
        None,
        String::new().into(),
        String::new().into(),
        Some(vec![Scope::ChatEdit, Scope::ChatRead, Scope::UserWriteChat]),
        None,
    );

    token
        .refresh_token(&http_client)
        .await
        .context("Failed to refresh token")?;

    let mut updated_token = UserToken::from_token(&http_client, token.access_token)
        .await
        .context("Failed to validate token")?;

    updated_token.refresh_token = token.refresh_token;

    Ok(updated_token)
}

async fn request_new_token(client_id: ClientId, http: &reqwest::Client) -> Result<UserToken> {
    let scopes = vec![Scope::ChatEdit, Scope::ChatRead, Scope::UserWriteChat];

    let mut token_builder = DeviceUserTokenBuilder::new(client_id, scopes);
    let device_code = token_builder
        .start(http)
        .await
        .context("Failed to start token request")?;

    if Command::new("powershell")
        .arg("Start-Process")
        .arg(format!("\"{}\"", device_code.verification_uri))
        .output()
        .await
        .is_err()
    {
        tracing::warn!("Please go to {}", device_code.verification_uri);
    }

    let token = token_builder
        .wait_for_code(http, tokio::time::sleep)
        .await
        .context("Failed to wait for token")?;

    Ok(token)
}
