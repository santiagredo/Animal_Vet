use backend::main_test::spawn_test_app;
use models::entities::users::Model;
use once_cell::sync::Lazy;
use sea_orm::prelude::Uuid;

#[tokio::test]
async fn unverified_user_getting_pet_type_fails() {
    let app_address = spawn_test_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{app_address}/pet_types"))
        .query(&[("pet_type_id", "1")])
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: unverified_user_getting_pet_type_fails -- {:?}",
        response
    );
}

#[tokio::test]
async fn verified_user_getting_pet_type_succeeds() {
    static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
        reqwest::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap()
    });

    let app_address = spawn_test_app().await;

    // new user
    let uuid = Uuid::new_v4().to_string();

    let new_user = Model {
        name: Some(String::from("user name test")),
        email: Some(format!("{uuid}@test.com")),
        password: Some(String::from("user_password")),
        phone_number: Some(String::from("3004006000")),
        document_id: Some(String::from("1000400600")),
        ..Default::default()
    };

    let response = CLIENT
        .post(format!("{app_address}/users"))
        .json(&new_user)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: new_user_is_success -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialize error")
    );

    // login new user
    let login_info = Model {
        email: Some(format!("{uuid}@test.com")),
        password: Some(String::from("user_password")),
        ..Default::default()
    };

    let login_response = CLIENT
        .post(format!("{app_address}/session/login"))
        .json(&login_info)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        login_response.status().is_success(),
        "failed: verified_user_getting_pet_type_succeeds / login_response -- {:?}",
        login_response
            .json::<String>()
            .await
            .expect("Failed to deserialize error")
    );

    // select pet type
    let pet_type_response = CLIENT
        .get(format!("{app_address}/pet_types"))
        .query(&[("pet_type_id", "1")])
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        pet_type_response.status().is_success(),
        "failed: unverified_user_getting_pet_type_fails / pet_type_response -- {:?}",
        pet_type_response
    );

    // new user can delete itself
    let response = CLIENT
        .delete(format!("{app_address}/users"))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: new_user_can_delete -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to desrialize error")
    );
}

#[tokio::test]
async fn verified_user_getting_wrong_pet_type_fails() {
    let app_address = spawn_test_app().await;

    static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
        reqwest::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap()
    });

    // new user
    let uuid = Uuid::new_v4().to_string();

    let new_user = Model {
        name: Some(String::from("user name test")),
        email: Some(format!("{uuid}@test.com")),
        password: Some(String::from("user_password")),
        phone_number: Some(String::from("3004006000")),
        document_id: Some(String::from("1000400600")),
        ..Default::default()
    };

    let response = CLIENT
        .post(format!("{app_address}/users"))
        .json(&new_user)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: new_user_is_success -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialize error")
    );

    // login new user
    let login_info = Model {
        email: Some(format!("{uuid}@test.com")),
        password: Some(String::from("user_password")),
        ..Default::default()
    };

    let login_response = CLIENT
        .post(format!("{app_address}/session/login"))
        .json(&login_info)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        login_response.status().is_success(),
        "failed: verified_user_getting_pet_type_succeeds / login_response -- {:?}",
        login_response
            .json::<String>()
            .await
            .expect("Failed to deserialize error")
    );

    // select wrong pet type id
    let pet_type_response = CLIENT
        .get(format!("{app_address}/pet_types"))
        .query(&[("pet_type_id", "2147483647")])
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        pet_type_response.status().is_client_error(),
        "failed: verified_user_getting_wrong_pet_type_fails / pet_type_response -- {:?}",
        pet_type_response
            .json::<String>()
            .await
            .expect("Failed to deserialize error")
    );

    // new user can delete itself
    let response = CLIENT
        .delete(format!("{app_address}/users"))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: new_user_can_delete -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to desrialize error")
    );
}
