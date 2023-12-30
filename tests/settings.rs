use pocketbase_sdk::{admin::Admin, client::Client};

mod constants;

#[test]
fn settings_get_all() {
    let client = Admin::new(constants::POCKETBASE_URL)
        .auth_with_password(constants::USERNAME, constants::PASSWORD)
        .unwrap();

    let response = client
        .settings()
        .get_all()
        .fields(vec!["meta".to_string()])
        .call();

    assert!(response.is_ok());
    let settings = response.unwrap();

    assert!(settings.meta.app_name.len() > 0);
}

#[test]
fn test_s3() {
    let client = Admin::new(constants::POCKETBASE_URL)
        .auth_with_password(constants::USERNAME, constants::PASSWORD)
        .unwrap();

    let response = client
        .settings()
        .get_all()
        .fields(vec!["meta".to_string()])
        .call();

    assert!(response.is_ok());
    let settings = response.unwrap();

    assert!(settings.meta.app_name.len() > 0);
}


