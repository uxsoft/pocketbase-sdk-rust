#[tokio::test]
async fn realtime() {
    let receiver = spinta::connect("http://localhost:8090/api/realtime").unwrap();
    while let Some(event) = receiver.try_recv() {
        println!("Received `{:?}`", event);
    }  

    assert!(false);
}
