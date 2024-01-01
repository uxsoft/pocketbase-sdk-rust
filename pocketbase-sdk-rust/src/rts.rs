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

pin_project! {
    pub struct ConnectedRealtimeManager<'a> {
        pub client: &'a Client<Auth>,
        pub client_id: String,
        #[pin]
        pub stream: Box<dyn Stream<Item = Event>>,
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FirstMessage {
    client_id: String,
}

impl<'a> RealtimeManager<'a> {
    pub async fn connect<E>(&self) -> Result<ConnectedRealtimeManager<'a>> {
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

        let only_successes: Box<dyn Stream<Item = Event>> =
            Box::new(stream.filter_map(|i| async { i.ok() }));

        Ok(ConnectedRealtimeManager {
            client: self.client,
            client_id: first_message.client_id,
            stream: only_successes,
        })
    }
}

impl<'a> ConnectedRealtimeManager<'a> {
    async fn announce_topics(&self, topics: &[&str]) -> Result<()> {
        let url = format!("{}/api/realtime", self.client.base_url);

        let body = json!({
            "clientId": self.client_id,
            "subscriptions": topics,
        })
        .to_string();

        let _res = reqwest::Client::new().post(url).body(body).send().await?;

        Ok(())
    }

    pub fn get_stream(self: Pin<&mut Self>) {
        let mut this = self.project();

        let a = this.stream.as_mut().next();
    }
}
