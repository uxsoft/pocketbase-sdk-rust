use pocketbase_sdk::client::Client;

mod constants;

#[test]
fn health_check() {
    let client = Client::new(constants::POCKETBASE_URL);
    let health_check_response = client.health_check();
    
    assert!(health_check_response.is_ok());
}