use backoffice::main_test::spawn_test_app;
use models::entities::medical_records::Model;
use once_cell::sync::Lazy;

#[tokio::test]
async fn unverified_user_posting_medical_record_fails() {
    let app_address = spawn_test_app().await;

    let client = reqwest::Client::new();

    let medical_record = Model {
        ..Default::default()
    };

    let response = client
        .post(format!("{app_address}/medical_records"))
        .json(&medical_record)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: unverified_user_posting_medical_records_fails -- {:?}",
        response
            .json::<String>()
            .await
            .expect("Failed to deserialze error")
    );
}

#[tokio::test]
async fn unverified_user_getting_medical_records_fails() {
    let app_address = spawn_test_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{app_address}/medical_records"))
        .query(&[("medical_record_id", "1")])
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_client_error(),
        "failed: unverified_user_getting_medical_records_fails -- {:?}",
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
async fn verified_user_medical_records_crud_operations() {
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

    // post pet
    let mut new_pet = models::entities::pets::Model {
        pet_type_id: Some(1),
        name: Some(String::from("Test pet name")),
        ..Default::default()
    };

    let response = CLIENT
        .post(format!("{app_address}/pets"))
        .json(&new_pet)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_medical_records_crud_operations / post new pet failed -- {:?}",
        response
    );

    let stored_pet = response
        .json::<models::entities::pets::Model>()
        .await
        .unwrap();

    new_pet.pet_id = stored_pet.pet_id;

    // post medical records
    let new_medical_record = Model {
        pet_id: Some(new_pet.pet_id),
        comments: Some(format!("Test")),
        ..Default::default()
    };

    let response = CLIENT
        .post(format!("{app_address}/medical_records"))
        .json(&new_medical_record)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_medical_records_crud_operations / post new medical record failed -- {:?}",
        response
    );

    let stored_medical_record = response.json::<Model>().await.unwrap();

    // get medical records
    let response = CLIENT
        .get(format!("{app_address}/medical_records"))
        .query(&[("medical_record_id", stored_medical_record.medical_record_id)])
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "failed: verified_user_medical_records_crud_operations / get medical record failed -- {:?}",
        response
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
