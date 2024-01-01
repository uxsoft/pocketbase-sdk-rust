use pocketbase_sdk::client::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

mod constants;

#[derive(Clone, Debug, Serialize, Default, Deserialize)]
pub struct Record {
    pub id: String,
    pub title: String,
}

#[tokio::test]
async fn list_records_success() {
    let client = Client::new(constants::POCKETBASE_URL)
        .auth_with_password("users", constants::USER_EMAIL, constants::PASSWORD)
        .await
        .unwrap();

    let records = client
        .records("posts")
        .list()
        .per_page(1010)
        .call::<Record>()
        .await;

    dbg!(&records);
    
    assert!(records.is_ok());
}