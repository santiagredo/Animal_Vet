use backoffice::main_test::spawn_test_app;
use models::entities::users::Model;
use once_cell::sync::Lazy;

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
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );
}

#[tokio::test]
async fn verified_user_getting_pet_type_succeeds() {
    let app_address = spawn_test_app().await;

    static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
        reqwest::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap()
    });

    let login_info = Model {
        email: Some(String::from("tests_staff@tests.com")),
        password: Some(String::from("test")),
        ..Default::default()
    };

    let response = CLIENT
        .post(format!("{app_address}/session/login"))
        .json(&login_info)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_getting_pet_type_succeeds / login_response -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

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
            .json::<String>()
            .await
            .expect("Failed to deserialize error")
    );

    let response = CLIENT
        .post(format!("{app_address}/session/logout"))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_getting_pet_type_succeeds / logout response -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
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

    let login_info = Model {
        email: Some(String::from("tests_staff@tests.com")),
        password: Some(String::from("test")),
        ..Default::default()
    };

    let response = CLIENT
        .post(format!("{app_address}/session/login"))
        .json(&login_info)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_getting_wrong_pet_type_fails / login_response -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialize error")
    );

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

    let response = CLIENT
        .post(format!("{app_address}/session/logout"))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_getting_wrong_pet_type_fails / logout response -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );
}
