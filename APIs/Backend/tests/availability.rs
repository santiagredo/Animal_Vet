use backend::main_test::spawn_test_app;
use models::entities::users::Model;
use once_cell::sync::Lazy;
use sea_orm::prelude::Uuid;

#[tokio::test]
async fn unverified_user_getting_availability_fails() {
    let app_address = spawn_test_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{app_address}/availability"))
        .query(&[("service_id", "1")])
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: unverified_user_getting_availability_fails -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );
}

static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap()
});

#[tokio::test]
async fn verified_user_availability_crud_operations() {
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

    // test availability
    let response = CLIENT
        .get(format!("{app_address}/availability"))
        .query(&[("service_id", "1")])
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_availability_crud_operations -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    //logout user
    let response = CLIENT
        .post(format!("{app_address}/session/logout"))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_appointment_crud_operations / logout existing user succeeds -- {:?}",
        {
            response
                .json::<String>()
                .await
                .expect("failed to deserialize error")
        }
    );
}
