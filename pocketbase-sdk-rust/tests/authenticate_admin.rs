use pocketbase_sdk::admin::Admin;
mod constants;

#[tokio::test]
pub async fn authenticate_admin_success() {
    let client = Admin::new(constants::POCKETBASE_URL)
        .auth_with_password(constants::USER_EMAIL, constants::PASSWORD)
        .await;
    assert!(client.is_ok());
}

#[tokio::test]
pub async fn authenticate_admin_failure() {
    let client = Admin::new(constants::POCKETBASE_URL)
        .auth_with_password("wrongidentity@wrongidentity.com", "wrongpassword")
        .await;
    assert!(client.is_err());
}