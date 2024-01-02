use crate::{collections::CollectionsManager, httpc::HttpClient};
use crate::{logs::LogsManager, records::RecordsManager, settings::SettingsManager, realtime::RealtimeManager};
use anyhow::Result;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct AuthSuccessResponse {
    token: String,
}

#[derive(Debug, Clone)]
pub struct NoAuth;

#[derive(Debug, Clone)]
pub struct Auth;

#[derive(Debug, Clone)]
pub struct Client<State = NoAuth> {
    pub base_url: String,
    pub auth_token: Option<String>,
    pub state: State,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HealthCheckResponse {
    pub code: i32,
    pub message: String,
}

impl<T> Client<T> {
    pub async fn health_check(&self) -> Result<HealthCheckResponse> {
        let url = format!("{}/api/health", self.base_url);
        let res = HttpClient::get(self, &url, None).await?.json().await?;
        Ok(res)
    }
}

impl Client<Auth> {
    pub fn collections(&self) -> CollectionsManager {
        CollectionsManager { client: self }
    }

    pub fn logs(&self) -> LogsManager {
        LogsManager { client: self }
    }

    pub fn records(&self, record_name: &'static str) -> RecordsManager {
        RecordsManager {
            client: self,
            name: record_name.into(),
        }
    }

    pub fn settings(&self) -> SettingsManager {
        SettingsManager { client: self }
    }

    pub fn realtime(&self) -> RealtimeManager {
        RealtimeManager { client: self }
    }
}

impl Client<NoAuth> {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            auth_token: None,
            state: NoAuth,
        }
    }

    pub async fn auth_with_password(
        &self,
        collection: &str,
        identifier: &str,
        secret: &str,
    ) -> Result<Client<Auth>> {
        let url = format!(
            "{}/api/collections/{}/auth-with-password",
            self.base_url, collection
        );

        let auth_payload = json!({
            "identity": identifier,
            "password": secret
        });

        let res = HttpClient::post(self, &url, auth_payload.to_string())
            .await?
            .json::<AuthSuccessResponse>()
            .await?;

        Ok(Client {
            base_url: self.base_url.clone(),
            state: Auth,
            auth_token: Some(res.token),
        })
    }
}
