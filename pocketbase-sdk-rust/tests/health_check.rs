use pocketbase_sdk::client::Client;

mod constants;

#[tokio::test]
async fn health_check() {
    let client = Client::new(constants::POCKETBASE_URL);
    let health_check_response = client.health_check().await;

    assert!(health_check_response.is_ok());
}
