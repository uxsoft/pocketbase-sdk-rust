use std::pin::{pin, Pin};

use crate::client::{Auth, Client};
use anyhow::Result;
use eventsource_stream::{Event, EventStream, Eventsource};
use futures::{Stream, StreamExt};
use pin_project_lite::pin_project;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone)]
pub struct RealtimeManager<'a> {
    pub client: &'a Client<Auth>,
}

// pin_project! {
pub struct ConnectedRealtimeManager<'a> {
    pub client: &'a Client<Auth>,
    pub client_id: String,
    // #[pin]
    pub stream: Pin<Box<dyn Stream<Item = Event>>>,
}
// }

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FirstMessage {
    client_id: String,
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
        let first_message: FirstMessage = serde_json::from_str(&first_event.data).unwrap();

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

    pub async fn get_next(self: Pin<&mut Self>) -> Option<Event> {
        let mut pinned_stream = unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().stream) };

        let event = pinned_stream.next().await;

        event
    }
}
