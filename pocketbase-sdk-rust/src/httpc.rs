use reqwest::{Request, Response};

use crate::client::Client;
use anyhow::Result;

pub struct Httpc;

impl Httpc {
    fn attach_auth_info<T>(partial_request: Request, client: &Client<T>) -> Result<Request> {
        match client.auth_token.as_ref() {
            Some(token) => Ok(partial_request.set("Authorization", token)),
            None => Ok(partial_request),
        }
    }

    pub async fn get<T>(
        client: &Client<T>,
        url: &str,
        query_params: Option<Vec<(&str, &str)>>,
    ) -> Result<Response, reqwest::Error> {
        let res = reqwest::Client::new()
            .get(url)
            .header(
                "Authorization",
                client.auth_token.as_ref().unwrap_or(&"".into()),
            )
            .query(&query_params)
            .send()
            .await;

        res
    }

    pub fn post<T>(client: &Client<T>, url: &str, body_content: String) -> Result<Response> {
        Ok(ureq::post(url))
            .map(|request| request.set("Content-Type", "application/json"))
            .and_then(|request| Self::attach_auth_info(request, client))
            .and_then(|request| Ok(request.send_string(body_content.as_str())?))
    }

    pub fn delete<T>(client: &Client<T>, url: &str) -> Result<Response> {
        Ok(ureq::delete(url))
            .and_then(|request| Self::attach_auth_info(request, client))
            .and_then(|request| Ok(request.call()?))
    }

    pub fn patch<T>(client: &Client<T>, url: &str, body_content: String) -> Result<Response> {
        Ok(ureq::patch(url))
            .map(|request| request.set("Content-Type", "application/json"))
            .and_then(|request| Self::attach_auth_info(request, client))
            .and_then(|request| Ok(request.send_string(body_content.as_str())?))
    }
}
