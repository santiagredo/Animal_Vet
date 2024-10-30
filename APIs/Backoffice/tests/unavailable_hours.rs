use backoffice::main_test::spawn_test_app;
use chrono::{NaiveDate, NaiveTime};
use models::entities::unavailable_hours::Model;
use once_cell::sync::Lazy;
use sea_orm::prelude::Uuid;

#[tokio::test]
async fn unverified_user_posting_unavailable_hours_fails() {
    let app_address = spawn_test_app().await;

    let client = reqwest::Client::new();

    let unavailable_hours = Model {
        ..Default::default()
    };

    let response = client
        .post(format!("{app_address}/unavailable_hours"))
        .json(&unavailable_hours)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: unverified_user_posting_unavailable_hours_fails -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );
}

#[tokio::test]
async fn unverified_user_getting_unavailable_hours_fails() {
    let app_address = spawn_test_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{app_address}/unavailable_hours"))
        .query(&[("unavailable_hour_id", "1")])
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: unverified_user_getting_unavailable_hours_fails -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );
}

#[tokio::test]
async fn unverified_user_patching_unavailable_hours_fails() {
    let app_address = spawn_test_app().await;

    let client = reqwest::Client::new();

    let unavailable_hours = Model {
        ..Default::default()
    };

    let response = client
        .patch(format!("{app_address}/unavailable_hours"))
        .json(&unavailable_hours)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: unverified_user_patching_unavailable_hours_fails -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );
}

#[tokio::test]
async fn unverified_user_deleting_unavailable_hours_fails() {
    let app_address = spawn_test_app().await;

    let client = reqwest::Client::new();

    let unavailable_hours = Model {
        unavailable_hour_id: 1,
        ..Default::default()
    };

    let response = client
        .delete(format!("{app_address}/unavailable_hours"))
        .json(&unavailable_hours)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: unverified_user_deleting_unavailable_hours_fails -- {:?}",
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
async fn verified_user_unavailable_hours_crud_operations() {
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
        "failed: verified_user_unavailable_hours_crud_operations / login existing staff succeeds -- {:?}",
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
        "failed: verified_user_unavailable_hours_crud_operations / post new service -- {:?}",
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

    //new unavailable hours
    let mut unavailable_hours = Model {
        service_id: Some(service.service_id),
        date: Some(NaiveDate::from_ymd_opt(2030, 01, 01).unwrap()),
        start_time: Some(NaiveTime::from_hms_opt(0, 10, 0).unwrap()),
        end_time: Some(NaiveTime::from_hms_opt(0, 10, 0).unwrap()),
        reason: Some(format!("Test")),
        ..Default::default()
    };

    let response = CLIENT
        .post(format!("{app_address}/unavailable_hours"))
        .json(&unavailable_hours)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_unavailable_hours_crud_operations / post new unavailable hours -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    let stored_unavailable_hours = response.json::<Model>().await.unwrap();

    unavailable_hours.unavailable_hour_id = stored_unavailable_hours.unavailable_hour_id;

    //repeated unavailable hours
    let response = CLIENT
        .post(format!("{app_address}/unavailable_hours"))
        .json(&unavailable_hours)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: verified_user_unavailable_hours_crud_operations / post repeated unavailable hours -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    //get unavailable hours
    let response = CLIENT
        .get(format!("{app_address}/unavailable_hours"))
        .query(&[("unavailable_hour_id", unavailable_hours.unavailable_hour_id)])
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_unavailable_hours_crud_operations / get stored unavailable hours -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    unavailable_hours.start_time = Some(NaiveTime::from_hms_opt(01, 00, 00).unwrap());

    //patch unavailable hours
    let response = CLIENT
        .patch(format!("{app_address}/unavailable_hours"))
        .json(&unavailable_hours)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_unavailable_hours_crud_operations / patch stored unavailable hours -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    //delete unavailable hours
    let response = CLIENT
        .delete(format!("{app_address}/unavailable_hours"))
        .json(&unavailable_hours)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_unavailable_hours_crud_operations / delete stored unavailable hours -- {:?}",
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
        "failed: verified_user_unavailable_hours_crud_operations / delete stored service -- {:?}",
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
        "failed: verified_user_unavailable_hours_crud_operations / logout existing staff succeeds -- {:?}",
        {
            response
                .json::<String>()
                .await
                .expect("failed to deserialize error")
        }
    );
}
