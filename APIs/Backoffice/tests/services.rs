use backoffice::main_test::spawn_test_app;
use models::entities::services::Model;
use once_cell::sync::Lazy;
use sea_orm::prelude::Uuid;

#[tokio::test]
async fn unverified_user_posting_service_fails() {
    let app_address = spawn_test_app().await;

    let client = reqwest::Client::new();

    let service = Model {
        name: Some(format!("Test service")),
        duration: Some(5),
        is_enabled: Some(true),
        ..Default::default()
    };

    let response = client
        .post(format!("{app_address}/services"))
        .json(&service)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: unverified_user_posting_service_fails -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );
}

#[tokio::test]
async fn unverified_user_getting_service_fails() {
    let app_address = spawn_test_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{app_address}/services"))
        .query(&[("service_id", "1")])
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: unverified_user_getting_service_fails -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );
}

#[tokio::test]
async fn unverified_user_patching_service_fails() {
    let app_address = spawn_test_app().await;

    let client = reqwest::Client::new();

    let service = Model {
        service_id: 1,
        name: Some(format!("Test service")),
        duration: Some(5),
        is_enabled: Some(true),
        ..Default::default()
    };

    let response = client
        .patch(format!("{app_address}/services"))
        .json(&service)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: unverified_user_patching_service_fails -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );
}

#[tokio::test]
async fn unverified_user_deleting_service_fails() {
    let app_address = spawn_test_app().await;

    let client = reqwest::Client::new();

    let service = Model {
        service_id: 1,
        ..Default::default()
    };

    let response = client
        .delete(format!("{app_address}/services"))
        .json(&service)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: unverified_user_deleting_service_fails -- {:?}",
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
async fn verified_user_service_crud_operations() {
    let app_address = spawn_test_app().await;

    //login staff
    let login_info = models::entities::users::Model {
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
        "failed: verified_user_service_crud_succeeds / login existing staff succeeds -- {:?}",
        {
            response
                .json::<String>()
                .await
                .expect("failed to deserialize error")
        }
    );

    //new service
    let uuid = Uuid::new_v4().to_string();

    let mut service = Model {
        name: Some(uuid),
        duration: Some(5),
        is_enabled: Some(true),
        ..Default::default()
    };

    let response = CLIENT
        .post(format!("{app_address}/services"))
        .json(&service)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_service_crud_succeeds / post new service -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    let stored_service = response.json::<Model>().await.unwrap();

    service.service_id = stored_service.service_id;

    //repeated name service
    let response = CLIENT
        .post(format!("{app_address}/services"))
        .json(&service)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: verified_user_service_crud_succeeds / post repeated name service -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    //get service
    let response = CLIENT
        .get(format!("{app_address}/services"))
        .query(&[("service_id", stored_service.service_id)])
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_service_crud_succeeds / get stored service -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    service.name = Some(format!("Test service name"));

    //patch service
    let response = CLIENT
        .patch(format!("{app_address}/services"))
        .json(&service)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_service_crud_succeeds / patch stored service -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    //delete service
    let response = CLIENT
        .delete(format!("{app_address}/services"))
        .json(&service)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_service_crud_succeeds / delete stored service -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    //logout staff
    let response = CLIENT
        .post(format!("{app_address}/session/logout"))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_service_crud_succeeds / logout existing staff succeeds -- {:?}",
        {
            response
                .json::<String>()
                .await
                .expect("failed to deserialize error")
        }
    );
}
