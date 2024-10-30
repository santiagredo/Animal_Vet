use backoffice::main_test::spawn_test_app;
use chrono::NaiveTime;
use models::entities::work_days::Model;
use once_cell::sync::Lazy;
use sea_orm::prelude::Uuid;

#[tokio::test]
async fn unverified_user_posting_work_day_fails() {
    let app_address = spawn_test_app().await;

    let client = reqwest::Client::new();

    let work_day = Model {
        ..Default::default()
    };

    let response = client
        .post(format!("{app_address}/work_days"))
        .json(&work_day)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: unverified_user_posting_work_day_fails -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );
}

#[tokio::test]
async fn unverified_user_getting_work_day_fails() {
    let app_address = spawn_test_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{app_address}/work_day"))
        .query(&[("work_day_id", "1")])
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: unverified_user_getting_work_day_fails -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );
}

#[tokio::test]
async fn unverified_user_patching_work_day_fails() {
    let app_address = spawn_test_app().await;

    let client = reqwest::Client::new();

    let work_day = Model {
        ..Default::default()
    };

    let response = client
        .patch(format!("{app_address}/work_days"))
        .json(&work_day)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: unverified_user_patching_work_day_fails -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );
}

#[tokio::test]
async fn unverified_user_deleting_work_day_fails() {
    let app_address = spawn_test_app().await;

    let client = reqwest::Client::new();

    let work_day = Model {
        work_day_id: 1,
        ..Default::default()
    };

    let response = client
        .delete(format!("{app_address}/work_days"))
        .json(&work_day)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: unverified_user_deleting_work_day_fails -- {:?}",
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
async fn verified_user_work_day_crud_operations() {
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
        "failed: verified_user_work_day_crud_operations / login existing staff succeeds -- {:?}",
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
        "failed: verified_user_work_day_crud_operations / post new service -- {:?}",
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

    //new work day
    let mut work_day = Model {
        service_id: Some(service.service_id),
        day_id: Some(1),
        is_enabled: Some(true),
        open_time: Some(NaiveTime::from_hms_opt(0, 10, 0).unwrap()),
        close_time: Some(NaiveTime::from_hms_opt(0, 10, 0).unwrap()),
        lunch_from_time: Some(NaiveTime::from_hms_opt(0, 10, 0).unwrap()),
        lunch_to_time: Some(NaiveTime::from_hms_opt(0, 10, 0).unwrap()),
        ..Default::default()
    };

    let response = CLIENT
        .post(format!("{app_address}/work_days"))
        .json(&work_day)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_work_day_crud_operations / post new work day -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    let stored_work_day = response.json::<Model>().await.unwrap();

    work_day.work_day_id = stored_work_day.work_day_id;

    //repeated work day
    let response = CLIENT
        .post(format!("{app_address}/work_days"))
        .json(&work_day)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: verified_user_work_day_crud_operations / post repeated work day -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    //get work day
    let response = CLIENT
        .get(format!("{app_address}/work_days"))
        .query(&[("work_day_id", work_day.work_day_id)])
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_work_day_crud_operations / get stored work day -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    work_day.open_time = Some(NaiveTime::from_hms_opt(01, 00, 00).unwrap());

    //patch work day
    let response = CLIENT
        .patch(format!("{app_address}/work_days"))
        .json(&work_day)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_work_day_crud_operations / patch stored work day -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    //delete work day
    let response = CLIENT
        .delete(format!("{app_address}/work_days"))
        .json(&work_day)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_work_day_crud_operations / delete stored work day -- {:?}",
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
        "failed: verified_user_work_day_crud_operations / delete stored service -- {:?}",
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
        "failed: verified_user_work_day_crud_operations / logout existing staff succeeds -- {:?}",
        {
            response
                .json::<String>()
                .await
                .expect("failed to deserialize error")
        }
    );
}
