#![allow(non_snake_case)]
use std::{hash::Hash, pin::Pin};

use dioxus::prelude::*;
use eventsource_stream::Eventsource;
use futures::stream::StreamExt;
use serde::Deserialize;

fn main() {
    dioxus_web::launch(App);
}

#[derive(Deserialize)]
struct Post {
    // collection_id: String,
    // collection_name: String,
    // created: String,
    id: String,
    // updated: String,
    title: String,
}

async fn get_posts() -> anyhow::Result<Vec<Post>> {
    let client = pocketbase_sdk::client::Client::new("http://localhost:8090")
        .auth_with_password("users", "me@uxsoft.cz", "asdasdasdasd")
        .await?;

    let records = client.records("posts").list().call().await?;

    Ok(records.items)
}

fn App(cx: Scope) -> Element {
    let posts = use_future(cx, (), |_| get_posts());

    let mut events = use_ref(cx, Vec::<Post>::new);
    let sync_task: &Coroutine<()> = use_coroutine(cx, |rx: UnboundedReceiver<_>| {
        let events = events.to_owned();

        async move {
            let client = pocketbase_sdk::client::Client::new("http://localhost:8090")
                .auth_with_password("users", "me@uxsoft.cz", "asdasdasdasd")
                .await
                .unwrap();

            let initial = client.records("posts").list().call::<Post>().await.unwrap();

            events.write().extend(initial.items);

            let mut rts = client.realtime().connect().await.unwrap();
            let mut prts = Pin::new(&mut rts);

            prts.announce_topics(&["posts"]).await.unwrap();

            while let Ok((topic, change)) = prts.as_mut().get_next().await {
                events.with_mut(|col| change.apply(col, |r| &r.id))
            }
        }
    });

    cx.render(rsx! {
        div {
            ul {
                style: "background-color: blue",

                match posts.value() {
                    Some(Ok(list)) => {
                        // if it is, render the stories
                        rsx! {
                            div {
                                // iterate over the stories with a for loop
                                for p in list {
                                    // render every story with the StoryListing component
                                    li { key: "{p.id}", "{p.title}" }
                                }
                            }
                        }
                    }
                    Some(Err(err)) => {
                        // if there was an error, render the error
                        rsx! {"An error occurred while fetching stories {err}"}
                    }
                    None => {
                        // if the future is not resolved yet, render a loading message
                        rsx! {"Loading items"}
                    }
                }
            }

            ul {
                style: "background-color: green",
                for p in &*events.read() {
                    li { key: "{p.id}", "{p.title}" }
                }
            }
            br {}
            button {
                onclick: move |_| {

                },
                "Connect"
            }
        }
    })
}
