use backoffice::main_test::spawn_test_app;
use chrono::{NaiveDate, NaiveTime};
use models::entities::special_dates::Model;
use once_cell::sync::Lazy;
use sea_orm::prelude::Uuid;

#[tokio::test]
async fn unverified_user_posting_special_date_fails() {
    let app_address = spawn_test_app().await;

    let client = reqwest::Client::new();

    let special_date = Model {
        ..Default::default()
    };

    let response = client
        .post(format!("{app_address}/special_dates"))
        .json(&special_date)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: unverified_user_posting_special_date_fails -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );
}

#[tokio::test]
async fn unverified_user_getting_special_date_fails() {
    let app_address = spawn_test_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{app_address}/special_dates"))
        .query(&[("special_date_id", "1")])
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: unverified_user_getting_special_date_fails -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );
}

#[tokio::test]
async fn unverified_user_patching_special_date_fails() {
    let app_address = spawn_test_app().await;

    let client = reqwest::Client::new();

    let special_date = Model {
        ..Default::default()
    };

    let response = client
        .patch(format!("{app_address}/special_dates"))
        .json(&special_date)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: unverified_user_patching_special_date_fails -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );
}

#[tokio::test]
async fn unverified_user_deleting_special_date_fails() {
    let app_address = spawn_test_app().await;

    let client = reqwest::Client::new();

    let special_date = Model {
        special_date_id: 1,
        ..Default::default()
    };

    let response = client
        .delete(format!("{app_address}/special_dates"))
        .json(&special_date)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: unverified_user_deleting_special_date_fails -- {:?}",
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
async fn verified_user_special_date_crud_operations() {
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
        "failed: verified_user_special_date_crud_operations / login existing staff succeeds -- {:?}",
        {
            response
                .json::<String>()
                .await
                .expect("failed to deserialize error")
        }
    );

    //new service
    let uuid = Uuid::new_v4().to_string();

    let mut service = models::entities::services::Model {
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
        "failed: verified_user_special_date_crud_operations / post new service -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    let stored_service = response
        .json::<models::entities::services::Model>()
        .await
        .unwrap();

    service.service_id = stored_service.service_id;

    //new special date
    let mut special_date = Model {
        service_id: Some(service.service_id),
        date: Some(NaiveDate::from_ymd_opt(2030, 01, 01).unwrap()),
        is_working_date: Some(true),
        open_time: Some(NaiveTime::from_hms_opt(0, 10, 0).unwrap()),
        close_time: Some(NaiveTime::from_hms_opt(0, 10, 0).unwrap()),
        lunch_from_time: Some(NaiveTime::from_hms_opt(0, 10, 0).unwrap()),
        lunch_to_time: Some(NaiveTime::from_hms_opt(0, 10, 0).unwrap()),
        reason: Some(format!("Test")),
        ..Default::default()
    };

    let response = CLIENT
        .post(format!("{app_address}/special_dates"))
        .json(&special_date)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_special_date_crud_operations / post new special date -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    let stored_special_date = response.json::<Model>().await.unwrap();

    special_date.special_date_id = stored_special_date.special_date_id;

    //repeated special date
    let response = CLIENT
        .post(format!("{app_address}/special_dates"))
        .json(&special_date)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: verified_user_special_date_crud_operations / post repeated special dates -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    //get special date
    let response = CLIENT
        .get(format!("{app_address}/special_dates"))
        .query(&[("special_date_id", special_date.special_date_id)])
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_special_date_crud_operations / get stored special date -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    special_date.open_time = Some(NaiveTime::from_hms_opt(01, 00, 00).unwrap());

    //patch special date
    let response = CLIENT
        .patch(format!("{app_address}/special_dates"))
        .json(&special_date)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_special_date_crud_operations / patch stored special date -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    //delete special date
    let response = CLIENT
        .delete(format!("{app_address}/special_dates"))
        .json(&special_date)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_special_date_crud_operations / delete stored special date -- {:?}",
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
        "failed: verified_user_special_date_crud_operations / delete stored service -- {:?}",
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
        "failed: verified_user_special_date_crud_operations / logout existing staff succeeds -- {:?}",
        {
            response
                .json::<String>()
                .await
                .expect("failed to deserialize error")
        }
    );
}
