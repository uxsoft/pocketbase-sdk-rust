use std::collections::HashMap;

use crate::client::{Auth, Client};
use crate::httpc::Httpc;
use anyhow::{anyhow, Result};
use serde::Serialize;
use serde::{de::DeserializeOwned, Deserialize};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsResponse {
    pub meta: Meta,
    pub logs: Logs,
    pub smtp: Smtp,
    pub s3: S3,
    pub backups: Backups,
    pub admin_auth_token: AuthToken,
    pub admin_password_reset_token: AuthToken,
    pub admin_file_token: AuthToken,
    pub record_auth_token: AuthToken,
    pub record_password_reset_token: AuthToken,
    pub record_email_change_token: AuthToken,
    pub record_verification_token: AuthToken,
    pub record_file_token: AuthToken,
    pub email_auth: EmailAuth,
    pub google_auth: SocialAuth,
    pub facebook_auth: SocialAuth,
    pub github_auth: SocialAuth,
    pub gitlab_auth: SocialAuth,
    pub discord_auth: SocialAuth,
    pub twitter_auth: SocialAuth,
    pub microsoft_auth: SocialAuth,
    pub spotify_auth: SocialAuth,
    pub kakao_auth: SocialAuth,
    pub twitch_auth: SocialAuth,
    pub strava_auth: SocialAuth,
    pub gitee_auth: SocialAuth,
    pub livechat_auth: SocialAuth,
    pub gitea_auth: SocialAuth,
    pub oidc_auth: SocialAuth,
    pub oidc2_auth: SocialAuth,
    pub oidc3_auth: SocialAuth,
    pub apple_auth: SocialAuth,
    pub instagram_auth: SocialAuth,
    pub vk_auth: SocialAuth,
    pub yandex_auth: SocialAuth,
    pub patreon_auth: SocialAuth,
    pub mailcow_auth: SocialAuth,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailTemplate {
    pub body: String,
    pub subject: String,
    pub action_url: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub app_name: String,
    pub app_url: String,
    pub hide_controls: bool,
    pub sender_name: String,
    pub sender_address: String,
    pub verification_template: EmailTemplate,
    pub reset_password_template: EmailTemplate,
    pub confirm_email_change_template: EmailTemplate,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Logs {
    pub max_days: usize,
    pub min_level: usize,
    pub log_ip: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Smtp {
    pub enabled: bool,
    pub host: String,
    pub port: usize,
    pub username: String,
    pub password: String,
    pub auth_method: String,
    pub tls: bool,
    pub local_name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct S3 {
    pub enabled: bool,
    pub bucket: String,
    pub region: String,
    pub endpoint: String,
    pub access_key: String,
    pub secret: String,
    pub force_path_style: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Backups {
    pub cron: String,
    pub cron_max_keep: usize,
    pub s3: S3,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthToken {
    pub secret: String,
    pub duration: usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailAuth {
    pub enabled: bool,
    pub except_domains: Option<String>,
    pub only_domains: Option<String>,
    pub min_password_length: usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialAuth {
    pub enabled: bool,
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub user_api_url: String,
    pub display_name: String,
    pub pkce: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GetAllRequestBuilder<'a> {
    pub client: &'a Client<Auth>,
    pub fields: Option<Vec<String>>,
}

impl<'a> GetAllRequestBuilder<'a> {
    pub fn call(&self) -> Result<SettingsResponse> {
        let url = format!("{}/api/settings", self.client.base_url);

        let mut build_opts: Vec<(&str, String)> = vec![];

        if let Some(fields) = &self.fields {
            let fields_json = serde_json::to_string(&fields).unwrap();
            build_opts.push(("filter", fields_json))
        }

        match Httpc::get(
            self.client,
            &url,
            Some(
                build_opts
                    .iter()
                    .map(|(a, b)| (*a, b.as_str()))
                    .collect(),
            ),
        ) {
            Ok(result) => {

                let response = result.into_json::<SettingsResponse>()?;
                Ok(response)
            }
            Err(e) => Err(e),
        }
    }

    pub fn fields(&self, fields: Vec<String>) -> Self {
        Self {
            fields: Some(fields),
            ..self.clone()
        }
    }
}

#[derive(Debug, Clone)]
pub struct TestS3RequestBuilder<'a> {
    pub client: &'a Client<Auth>,
    pub fields: Option<Vec<String>>,
}

impl<'a> TestS3RequestBuilder<'a> {
    pub fn call(&self) -> Result<SettingsResponse> {
        let url = format!("{}/api/settings", self.client.base_url);

        let mut build_opts: Vec<(&str, String)> = vec![];

        if let Some(fields) = &self.fields {
            let fields_json = serde_json::to_string(&fields).unwrap();
            build_opts.push(("filter", fields_json))
        }

        match Httpc::get(
            self.client,
            &url,
            Some(
                build_opts
                    .iter()
                    .map(|(a, b)| (*a, b.as_str()))
                    .collect(),
            ),
        ) {
            Ok(result) => {

                let response = result.into_json::<SettingsResponse>()?;
                Ok(response)
            }
            Err(e) => Err(e),
        }
    }

    pub fn fields(&self, fields: Vec<String>) -> Self {
        Self {
            fields: Some(fields),
            ..self.clone()
        }
    }
}


#[derive(Debug, Clone)]
pub struct SettingsManager<'a> {
    pub client: &'a Client<Auth>,
}

impl<'a> SettingsManager<'a> {
    pub fn get_all(&self) -> GetAllRequestBuilder<'a> {
        GetAllRequestBuilder {
            client: self.client,
            fields: None,
        }
    }

    pub fn update(&self, identifier: &'a str) -> () {}
    pub fn test_s3(&self, identifier: &'a str) -> () {}
    pub fn test_email(&self, identifier: &'a str) -> () {}
    pub fn generate_apple_client_secret(&self, identifier: &'a str) -> () {}
}
