use pocketbase_sdk::settings::{EmailTemplateType, S3FileSystem};
use pocketbase_sdk::{admin::Admin, client::Client};

mod constants;

#[tokio::test]
async fn settings_get_all() {
    let client = Admin::new(constants::POCKETBASE_URL)
        .auth_with_password(constants::USER_EMAIL, constants::PASSWORD)
        .await
        .unwrap();

    let response = client.settings().get_all().call().await;

    assert!(response.is_ok());
    let settings = response.unwrap();

    assert!(settings.meta.app_name.len() > 0);
}

#[tokio::test]
async fn test_s3() {
    let client = Admin::new(constants::POCKETBASE_URL)
        .auth_with_password(constants::USER_EMAIL, constants::PASSWORD)
        .await
        .unwrap();

    let are_backups_connected = client
        .settings()
        .test_s3()
        .filesystem(S3FileSystem::Backups)
        .call()
        .await;

    let is_storage_connected = client
        .settings()
        .test_s3()
        .filesystem(S3FileSystem::Storage)
        .call()
        .await;

    assert_eq!(are_backups_connected, true);
    assert_eq!(is_storage_connected, false);
}

#[tokio::test]
async fn test_email() {
    let client = Admin::new(constants::POCKETBASE_URL)
        .auth_with_password(constants::USER_EMAIL, constants::PASSWORD)
        .await
        .unwrap();

    let sent = client
        .settings()
        .test_email(constants::USER_EMAIL.to_string())
        .template(EmailTemplateType::EmailChange)
        .call()
        .await;

    assert_eq!(sent, true);
}

#[tokio::test]
async fn test_generate_apple_client_secret() {
    let client = Admin::new(constants::POCKETBASE_URL)
        .auth_with_password(constants::USER_EMAIL, constants::PASSWORD)
        .await
        .unwrap();

    let success = client
        .settings()
        .generate_apple_client_secret(
            constants::APPLE_CLIENT_ID,
            constants::APPLE_TEAM_ID,
            constants::APPLE_KEY_ID,
            constants::APPLE_PRIVATE_KEY,
            15777000,
        )
        .call()
        .await;

    assert_eq!(success, true);
}
