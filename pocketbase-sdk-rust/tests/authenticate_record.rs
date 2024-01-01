use pocketbase_sdk::client::Client;

mod constants;

#[tokio::test]
pub async fn authenticate_record_success() {
    let client = Client::new(constants::POCKETBASE_URL)
        .auth_with_password("users", constants::USER_EMAIL, constants::PASSWORD)
        .await;

    dbg!(&client);

    assert!(client.is_ok());
}

#[tokio::test]
pub async fn authenticate_record_error() {
    let client = Client::new(constants::POCKETBASE_URL)
        .auth_with_password("users", "bingo", "bango")
        .await;
    assert!(client.is_err());
}
