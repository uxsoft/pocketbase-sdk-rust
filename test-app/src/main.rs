#![allow(non_snake_case)]
use dioxus::prelude::*;
use eventsource_stream::Eventsource;
use futures::stream::StreamExt;

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let mut events = use_ref(cx, Vec::<String>::new);

    let sync_task: &Coroutine<()> = use_coroutine(cx, |rx: UnboundedReceiver<_>| {
        let events = events.to_owned();

        async move {
            let mut stream = reqwest::Client::new()
                .get("http://localhost:8090/api/realtime")
                .send()
                .await
                .unwrap()
                .bytes_stream()
                .eventsource();

            events.write().push("Initalizing....".into());

            while let Some(event) = stream.next().await {
                match event {
                    Ok(event) => events.write().push(event.data),
                    Err(e) => events.write().push(e.to_string()),
                }
            }
        }
    });

    cx.render(rsx! {
        div {
            style: "margin: 200px auto; width: 100px",

            ul {
                for event in &*events.read() {
                    // Notice the body of this for loop is rsx code, not an expression
                    li {
                        "{event}"
                    }
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
