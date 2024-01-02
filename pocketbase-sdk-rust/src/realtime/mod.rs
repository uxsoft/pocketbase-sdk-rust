mod change;
mod store;

use crate::client::{Auth, Client};
use anyhow::Result;
pub use change::*;
use eventsource_stream::{Event, Eventsource};
use futures::{Stream, StreamExt};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use std::{
    pin::Pin,
    sync::{Arc, Mutex}, any::Any,
};
pub use store::*;

#[derive(Debug, Clone)]
pub struct RealtimeManager<'a> {
    pub client: &'a Client<Auth>,
}

pub struct ConnectedRealtimeManager<'a> {
    client: &'a Client<Auth>,
    client_id: String,
    stream: Pin<Box<dyn Stream<Item = Event>>>,
    stores: Vec<Arc<Mutex<dyn Subscriber>>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WelcomeMessage {
    client_id: String,
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
            stores: Vec::new(),
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

    fn notify_stores(&mut self, topic: String, change: Change) {
        for store in &self.stores {
            store.lock().unwrap().notify(&topic, &change);
        }
    }

    pub fn create_store<T: Record + DeserializeOwned>(
        &mut self,
        topic: String,
    ) -> &Mutex<Store<T>> {
        let store = Store::<T>::new(topic.clone());
        let arc = Arc::new(Mutex::new(store));
        self.stores.push(arc);

        arc.as_ref()
    }
}
