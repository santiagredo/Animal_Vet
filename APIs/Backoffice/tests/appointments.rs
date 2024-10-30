use backoffice::main_test::spawn_test_app;
use chrono::{Datelike, Days, Local, NaiveDateTime, NaiveTime};
use models::entities::appointments::Model as AppointmentsModel;
use once_cell::sync::Lazy;
use sea_orm::prelude::Uuid;

#[tokio::test]
async fn unverified_user_posting_appointment_fails() {
    let app_address = spawn_test_app().await;

    let client = reqwest::Client::new();

    let appointment = AppointmentsModel {
        ..Default::default()
    };

    let response = client
        .post(format!("{app_address}/appointments"))
        .json(&appointment)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: unverified_user_posting_appointment_fails -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );
}

#[tokio::test]
async fn unverified_user_getting_appointment_fails() {
    let app_address = spawn_test_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{app_address}/appointments"))
        .query(&[("appointment_id", "1")])
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: unverified_user_getting_appointment_fails -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );
}

#[tokio::test]
async fn unverified_user_patching_appointment_fails() {
    let app_address = spawn_test_app().await;

    let client = reqwest::Client::new();

    let appointment = AppointmentsModel {
        ..Default::default()
    };

    let response = client
        .patch(format!("{app_address}/appointment"))
        .json(&appointment)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: unverified_user_patching_appointment_fails -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );
}

#[tokio::test]
async fn unverified_user_deleting_appointment_fails() {
    let app_address = spawn_test_app().await;

    let client = reqwest::Client::new();

    let appointment = AppointmentsModel {
        appointment_id: 1,
        ..Default::default()
    };

    let response = client
        .delete(format!("{app_address}/appointments"))
        .json(&appointment)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: unverified_user_deleting_appointment_fails -- {:?}",
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
async fn verified_user_appointment_crud_operations() {
    let app_address = spawn_test_app().await;
    let uuid = Uuid::new_v4().to_string();

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
        "failed: verified_user_appointment_crud_operations / login existing staff succeeds -- {:?}",
        {
            response
                .json::<String>()
                .await
                .expect("failed to deserialize error")
        }
    );

    // new pet
    let pet = models::entities::pets::Model {
        pet_type_id: Some(1),
        name: Some(uuid.clone()),
        user_id: Some(2),
        ..Default::default()
    };

    let response = CLIENT
        .post(format!("{app_address}/pets"))
        .json(&pet)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_appointment_crud_operations / post new pet -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialize error"),
    );

    let pet_data: models::entities::pets::Model = response.json().await.unwrap();

    //new service
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
        "failed: verified_user_appointment_crud_operations / post new service -- {:?}",
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

    // current date plus 7 days
    let date = Local::now().checked_add_days(Days::new(7)).unwrap();

    // select day id
    let response = CLIENT
        .get(format!("{app_address}/days"))
        .query(&[("day_id", "0"), ("name", &date.weekday().to_string())])
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_appointment_crud_operations / get day id -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    let binding = response
        .json::<Vec<models::entities::days::Model>>()
        .await
        .unwrap();

    let stored_day = binding.get(0).unwrap();

    // new workday
    let work_day = models::entities::work_days::Model {
        service_id: Some(service.service_id),
        day_id: Some(stored_day.day_id),
        is_enabled: Some(true),
        open_time: Some(NaiveTime::from_hms_opt(08, 00, 00).unwrap()),
        close_time: Some(NaiveTime::from_hms_opt(17, 00, 00).unwrap()),
        lunch_from_time: Some(NaiveTime::from_hms_opt(12, 00, 00).unwrap()),
        lunch_to_time: Some(NaiveTime::from_hms_opt(13, 00, 00).unwrap()),
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
        "failed: verified_user_appointment_crud_operations / post new work day -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    // new appointment
    let mut appointment = AppointmentsModel {
        user_id: Some(2),
        pet_id: Some(pet_data.pet_id),
        date: Some(NaiveDateTime::new(
            date.date_naive(),
            NaiveTime::from_hms_opt(10, 00, 00).unwrap(),
        )),
        service_id: Some(service.service_id),
        ..Default::default()
    };

    let response = CLIENT
        .post(format!("{app_address}/appointments"))
        .json(&appointment)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_appointment_crud_operations / post new appointment -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    let stored_appointment = response.json::<AppointmentsModel>().await.unwrap();
    appointment.appointment_id = stored_appointment.appointment_id;

    // repeated appointment
    let response = CLIENT
        .post(format!("{app_address}/appointments"))
        .json(&appointment)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: verified_user_appointment_crud_operations / post repeated appointment -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    // current date plus 8 days
    let date = Local::now().checked_add_days(Days::new(8)).unwrap();

    // select day id
    let response = CLIENT
        .get(format!("{app_address}/days"))
        .query(&[("day_id", "0"), ("name", &date.weekday().to_string())])
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_appointment_crud_operations / get day id -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    let binding = response
        .json::<Vec<models::entities::days::Model>>()
        .await
        .unwrap();

    let stored_day = binding.get(0).unwrap();

    // new workday
    let work_day = models::entities::work_days::Model {
        service_id: Some(service.service_id),
        day_id: Some(stored_day.day_id),
        is_enabled: Some(true),
        open_time: Some(NaiveTime::from_hms_opt(08, 00, 00).unwrap()),
        close_time: Some(NaiveTime::from_hms_opt(17, 00, 00).unwrap()),
        lunch_from_time: Some(NaiveTime::from_hms_opt(12, 00, 00).unwrap()),
        lunch_to_time: Some(NaiveTime::from_hms_opt(13, 00, 00).unwrap()),
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
        "failed: verified_user_appointment_crud_operations / post new work day -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    // new special date
    let special_date = models::entities::special_dates::Model {
        service_id: Some(service.service_id),
        date: Some(date.date_naive()),
        is_working_date: Some(false),
        open_time: Some(NaiveTime::from_hms_opt(08, 00, 00).unwrap()),
        close_time: Some(NaiveTime::from_hms_opt(17, 00, 00).unwrap()),
        lunch_from_time: Some(NaiveTime::from_hms_opt(12, 00, 00).unwrap()),
        lunch_to_time: Some(NaiveTime::from_hms_opt(13, 00, 00).unwrap()),
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
        "failed: verified_user_appointment_crud_operations / post new special date -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    // appointment on special date fails
    appointment.date = Some(NaiveDateTime::new(date.date_naive(), date.time()));

    let response = CLIENT
        .post(format!("{app_address}/appointments"))
        .json(&appointment)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: verified_user_appointment_crud_operations / post appointment on special date -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    // current date plus 9 days
    let date = Local::now().checked_add_days(Days::new(9)).unwrap();

    // select day id
    let response = CLIENT
        .get(format!("{app_address}/days"))
        .query(&[("day_id", "0"), ("name", &date.weekday().to_string())])
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_appointment_crud_operations / get day id -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    let binding = response
        .json::<Vec<models::entities::days::Model>>()
        .await
        .unwrap();

    let stored_day = binding.get(0).unwrap();

    // new workday
    let work_day = models::entities::work_days::Model {
        service_id: Some(service.service_id),
        day_id: Some(stored_day.day_id),
        is_enabled: Some(true),
        open_time: Some(NaiveTime::from_hms_opt(08, 00, 00).unwrap()),
        close_time: Some(NaiveTime::from_hms_opt(17, 00, 00).unwrap()),
        lunch_from_time: Some(NaiveTime::from_hms_opt(12, 00, 00).unwrap()),
        lunch_to_time: Some(NaiveTime::from_hms_opt(13, 00, 00).unwrap()),
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
        "failed: verified_user_appointment_crud_operations / post new work day -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    //new unavailable hours
    let mut unavailable_hours = models::entities::unavailable_hours::Model {
        service_id: Some(service.service_id),
        date: Some(date.date_naive()),
        start_time: Some(NaiveTime::from_hms_opt(08, 00, 00).unwrap()),
        end_time: Some(NaiveTime::from_hms_opt(10, 00, 00).unwrap()),
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
        "failed: verified_user_appointment_crud_operations / post new unavailable hours -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    let stored_unavailable_hours = response
        .json::<models::entities::unavailable_hours::Model>()
        .await
        .unwrap();

    unavailable_hours.unavailable_hour_id = stored_unavailable_hours.unavailable_hour_id;

    // appointment on unavailable hours fails
    appointment.date = Some(NaiveDateTime::new(
        date.date_naive(),
        NaiveTime::from_hms_opt(09, 00, 00).unwrap(),
    ));

    let response = CLIENT
        .post(format!("{app_address}/appointments"))
        .json(&appointment)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: verified_user_appointment_crud_operations / post appointment on unavailable hours -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    // select appointment
    let response = CLIENT
        .get(format!("{app_address}/appointments"))
        .query(&[("appointment_id", appointment.appointment_id)])
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_appointment_crud_operations / get appointment -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );

    // patch appointment
    let appointment = AppointmentsModel {
        appointment_id: appointment.appointment_id,
        is_canceled: Some(true),
        ..Default::default()
    };

    let response = CLIENT
        .patch(format!("{app_address}/appointments"))
        .json(&appointment)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_appointment_crud_operations / patch appointment -- {:?}",
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
        "failed: verified_user_appointment_crud_operations / logout existing staff succeeds -- {:?}",
        {
            response
                .json::<String>()
                .await
                .expect("failed to deserialize error")
        }
    );
}
