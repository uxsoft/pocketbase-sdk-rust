use std::pin::{pin, Pin};

use crate::client::{Auth, Client};
use anyhow::Result;
use eventsource_stream::{Event, EventStream, Eventsource};
use futures::{Stream, StreamExt};
use pin_project_lite::pin_project;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone)]
pub struct RealtimeManager<'a> {
    pub client: &'a Client<Auth>,
}

pub struct ConnectedRealtimeManager<'a> {
    pub client: &'a Client<Auth>,
    pub client_id: String,
    pub stream: Pin<Box<dyn Stream<Item = Event>>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WelcomeMessage {
    client_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    Update,
    Create,
    Delete,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Change {
    pub record: Box<serde_json::value::RawValue>,
    pub action: Action,
}

impl Change {
    pub fn record<T: DeserializeOwned>(&self) -> Result<T> {
        let res = serde_json::from_str::<T>(&self.record.as_ref().get())?;
        Ok(res)
    }

    pub fn apply<T, F, K>(&self, collection: &mut Vec<T>, get_key: F)
    where
        T: DeserializeOwned,
        F: FnMut(&T) -> K,
        K: Eq,
    {
        let record = self.record::<T>().unwrap();

        match self.action {
            Action::Update => collection.,
            Action::Create => collection.push(record),
            Action::Delete => collection.retain(|i| get_key(&i) != get_key(&record)),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordBase {
    collection_id: String,
    collection_name: String,
    created: String,
    id: String,
    updated: String,
}

impl<'a> RealtimeManager<'a> {
    pub async fn connect(&self) -> Result<ConnectedRealtimeManager<'a>> {
        let url = format!("{}/api/realtime", self.client.base_url);
        let mut stream = reqwest::Client::new()
            .get(url)
            .send()
            .await
            .unwrap()
            .bytes_stream()
            .eventsource();

        let first_event = stream.next().await.unwrap().unwrap(); // to do prevent panic!
        let first_message: WelcomeMessage = serde_json::from_str(&first_event.data).unwrap();

        let only_successes =//: dyn Stream<Item = Event> =
            stream.filter_map(|i| async { i.ok() });

        Ok(ConnectedRealtimeManager {
            client: self.client,
            client_id: first_message.client_id,
            stream: Box::pin(only_successes),
        })
    }
}

impl<'a> ConnectedRealtimeManager<'a> {
    pub async fn announce_topics(&self, topics: &[&str]) -> Result<()> {
        let url = format!("{}/api/realtime", self.client.base_url);

        let body = json!({
            "clientId": self.client_id,
            "subscriptions": topics,
        })
        .to_string();

        let _res = reqwest::Client::new()
            .post(url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_next(self: Pin<&mut Self>) -> Result<(String, Change)> {
        let mut pinned_stream = Pin::new(&mut self.get_mut().stream);

        let event = pinned_stream.next().await;

        let event = event.ok_or(anyhow::anyhow!("Stream yielded no events."))?;

        let change = serde_json::from_str::<Change>(&event.data)?;

        Ok((event.event, change))
    }
}
