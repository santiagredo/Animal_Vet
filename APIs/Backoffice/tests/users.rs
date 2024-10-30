use backoffice::main_test::spawn_test_app;
use models::entities::users::Model;
use once_cell::sync::Lazy;
use sea_orm::prelude::Uuid;

static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap()
});

#[tokio::test]
async fn new_user_crud_operations_by_staff() {
    let app_address = spawn_test_app().await;

    let uuid = Uuid::new_v4().to_string();

    let mut new_user = Model {
        name: Some(String::from("staff name test")),
        email: Some(format!("{uuid}@test.com")),
        password: Some(String::from("staff_password")),
        phone_number: Some(String::from("3004006000")),
        document_id: Some(String::from("1000400600")),
        role: Some(3),
        ..Default::default()
    };

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
        "failed: login existing staff succeeds -- {:?}",
        {
            response
                .json::<String>()
                .await
                .expect("failed to deserialize error")
        }
    );

    let response = CLIENT
        .post(format!("{app_address}/users"))
        .json(&new_user)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: new user inserted by staff succeeds -- {:?}",
        {
            response
                .json::<String>()
                .await
                .expect("failed to deserialize error")
        }
    );

    let stored_user = response.json::<Model>().await.unwrap();

    new_user.user_id = stored_user.user_id;

    let user_email = new_user.email.clone();

    let new_user_repeated = Model {
        name: Some(String::from("staff name test")),
        email: user_email.clone(),
        password: Some(String::from("staff_password")),
        phone_number: Some(String::from("3004006000")),
        document_id: Some(String::from("1000400600")),
        ..Default::default()
    };

    let response = CLIENT
        .post(format!("{app_address}/users"))
        .json(&new_user_repeated)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: new user repeated email by staff fails -- {:?}",
        { response }
    );

    let user_id = new_user.user_id;

    let user_response = CLIENT
        .get(format!("{app_address}/users"))
        .query(&[("user_id", user_id.to_string())])
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        user_response.status().is_success(),
        "failed: new user selected by staff / user_response -- {:?}",
        user_response
    );

    let uuid = Uuid::new_v4().to_string();

    new_user.name = Some(String::from("test staff name"));
    new_user.email = Some(format!("{uuid}@test.com"));
    new_user.password = Some(String::from("test_password"));

    let response = CLIENT
        .patch(format!("{app_address}/users"))
        .json(&new_user)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: new user updated by staff / user_response -- {:?}",
        response
            .json::<String>()
            .await
            .expect("failed to deserialze error")
    );

    let response = CLIENT
        .delete(format!("{app_address}/users"))
        .json(&new_user)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: new user deleted  by staff -- {:?}",
        response
            .json::<String>()
            .await
            .expect("failed to deserialize error")
    );

    let response = CLIENT
        .post(format!("{app_address}/session/logout"))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: logout existing staff succeeds -- {:?}",
        {
            response
                .json::<String>()
                .await
                .expect("failed to deserialize error")
        }
    );
}
