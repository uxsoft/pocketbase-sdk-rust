use crate::client::Auth;
use crate::client::Client;
use crate::httpc::HttpClient;
use anyhow::{anyhow, Result};
use serde::Deserialize;
use serde_json::json;

pub struct Admin<'a> {
    pub base_url: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
struct AuthSuccessResponse {
    token: String,
}

impl<'a> Admin<'a> {
    pub async fn auth_with_password(&self, identifier: &str, secret: &str) -> Result<Client<Auth>> {
        let url = format!("{}/api/admins/auth-with-password", self.base_url);
        let credentials = json!({
            "identity": identifier,
            "password": secret,
        });
        let client = Client::new(self.base_url);
        let res = HttpClient::post(&client, &url, credentials.to_string())
            .await?
            .json::<AuthSuccessResponse>()
            .await?;

        Ok(Client {
            base_url: self.base_url.to_string(),
            state: Auth,
            auth_token: Some(res.token),
        })
    }

    pub fn new(base_url: &'a str) -> Admin<'a> {
        Admin { base_url }
    }
}
