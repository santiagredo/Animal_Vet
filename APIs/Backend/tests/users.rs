use backend::main_test::spawn_test_app;
use models::entities::users::Model;
use once_cell::sync::Lazy;
use sea_orm::prelude::Uuid;

#[cfg(test)]
mod new_user_empty_fields {
    use backend::main_test::spawn_test_app;
    use models::entities::users::Model;

    #[tokio::test]
    async fn new_user_all_fields_empty_fails() {
        let app_address = spawn_test_app().await;

        let client = reqwest::Client::new();

        let new_user = Model {
            ..Default::default()
        };

        let response = client
            .post(format!("{app_address}/users"))
            .json(&new_user)
            .send()
            .await
            .expect("Failed to execute request");

        assert!(response.status().is_client_error());
    }

    #[tokio::test]
    async fn new_user_name_not_empty_fails() {
        let app_address = spawn_test_app().await;

        let client = reqwest::Client::new();

        let new_user = Model {
            name: Some(String::from("test user")),
            ..Default::default()
        };

        let response = client
            .post(format!("{app_address}/users"))
            .json(&new_user)
            .send()
            .await
            .expect("Failed to execute request");

        assert!(response.status().is_client_error());
    }

    #[tokio::test]
    async fn new_user_email_not_empty_fails() {
        let app_address = spawn_test_app().await;

        let client = reqwest::Client::new();

        let new_user = Model {
            name: Some(String::from("test user")),
            email: Some(String::from("test_user@test.com")),
            ..Default::default()
        };

        let response = client
            .post(format!("{app_address}/users"))
            .json(&new_user)
            .send()
            .await
            .expect("Failed to execute request");

        assert!(response.status().is_client_error());
    }

    #[tokio::test]
    async fn new_user_password_not_empty_fails() {
        let app_address = spawn_test_app().await;

        let client = reqwest::Client::new();

        let new_user = Model {
            name: Some(String::from("test user")),
            email: Some(String::from("test_user@test.com")),
            password: Some(String::from("test")),
            ..Default::default()
        };

        let response = client
            .post(format!("{app_address}/users"))
            .json(&new_user)
            .send()
            .await
            .expect("Failed to execute request");

        assert!(response.status().is_client_error());
    }

    #[tokio::test]
    async fn new_user_phone_number_not_empty_fails() {
        let app_address = spawn_test_app().await;

        let client = reqwest::Client::new();

        let new_user = Model {
            name: Some(String::from("test user")),
            email: Some(String::from("test_user@test.com")),
            password: Some(String::from("test")),
            phone_number: Some(String::from("3004006000")),
            ..Default::default()
        };

        let response = client
            .post(format!("{app_address}/users"))
            .json(&new_user)
            .send()
            .await
            .expect("Failed to execute request");

        assert!(response.status().is_client_error());
    }
}

#[tokio::test]
async fn new_user_crud_operations() {
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

    // new user repeated email fails
    let response = CLIENT
        .post(format!("{app_address}/users"))
        .json(&new_user)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: new_user_repeated_email_fails -- {:?}",
        { response }
    );

    // new user can login
    let login_info = Model {
        email: Some(format!("{uuid}@test.com")),
        password: Some(format!("user_password")),
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
        "failed: new_user_can_login -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialize error")
    );

    // new user can get itself
    let response = CLIENT
        .get(format!("{app_address}/users"))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: new_user_can_select -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialize error")
    );

    //new user can update itself
    let updated_user = Model {
        name: Some(String::from("test user 2")),
        password: Some(String::from("test")),
        phone_number: Some(String::from("3004006000")),
        document_id: Some(String::from("1000400600")),
        ..Default::default()
    };

    let response = CLIENT
        .patch(format!("{app_address}/users"))
        .json(&updated_user)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: new_user_can_update -- {:?}",
        response
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
