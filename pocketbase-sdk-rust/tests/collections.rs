use pocketbase_sdk::admin::Admin;
use serde_json::json;

mod constants;

#[tokio::test]
async fn collections_list_success() {
    let admin_client = Admin::new(constants::POCKETBASE_URL)
        .auth_with_password(constants::USER_EMAIL, constants::PASSWORD)
        .await
        .unwrap();

    let collections_list = admin_client.collections().list().call().await;
    assert!(collections_list.is_ok())
}

#[tokio::test]
async fn collection_view_succes() {
    let admin_client = Admin::new(constants::POCKETBASE_URL)
        .auth_with_password(constants::USER_EMAIL, constants::PASSWORD)
        .await
        .unwrap();
    let collection = admin_client.collections().view("posts").call().await;
    assert!(collection.is_ok())
}
