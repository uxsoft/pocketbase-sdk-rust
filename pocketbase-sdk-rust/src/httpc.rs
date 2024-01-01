use reqwest::{Request, RequestBuilder, Response};

use crate::client::Client;
use anyhow::Result;

trait PocketbaseAuthExt {
    fn attach_auth_info<T>(self, client: &Client<T>) -> RequestBuilder;
}

impl PocketbaseAuthExt for RequestBuilder {
    fn attach_auth_info<T>(self, client: &Client<T>) -> RequestBuilder {
        match client.auth_token.as_ref() {
            Some(token) => self.header("Authorization", token),
            None => self,
        }
    }
}

pub struct HttpClient;

impl HttpClient {
    pub async fn get<T>(
        client: &Client<T>,
        url: &str,
        query_params: Option<Vec<(&str, &str)>>,
    ) -> Result<Response> {
        let res = reqwest::Client::new()
            .get(url)
            .attach_auth_info(client)
            .query(&query_params)
            .send()
            .await?;

        Ok(res)
    }

    pub async fn post<T>(client: &Client<T>, url: &str, body_content: String) -> Result<Response> {
        let res = reqwest::Client::new()
            .post(url)
            .attach_auth_info(client)
            .header("Content-Type", "application/json")
            .body(body_content)
            .send()
            .await?;

        Ok(res)
    }

    pub async fn delete<T>(client: &Client<T>, url: &str) -> Result<Response> {
        let res = reqwest::Client::new()
            .delete(url)
            .attach_auth_info(client)
            .send()
            .await?;

        Ok(res)
    }

    pub async fn patch<T>(client: &Client<T>, url: &str, body_content: String) -> Result<Response> {
        let res = reqwest::Client::new()
            .patch(url)
            .attach_auth_info(client)
            .header("Content-Type", "application/json")
            .body(body_content)
            .send()
            .await?;

        Ok(res)
    }
}
