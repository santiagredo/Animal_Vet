#[cfg(test)]
mod unverified_user_pet_requests {

    use backend::main_test::spawn_test_app;
    use models::entities::pets::Model as PetModel;

    #[tokio::test]
    async fn posting_pet_fails() {
        let app_address = spawn_test_app().await;

        let client = reqwest::Client::new();

        let pet_info = PetModel {
            pet_type_id: Some(1),
            name: Some(String::from("Test pet name")),
            user_id: Some(1),
            ..Default::default()
        };

        let response = client
            .post(format!("{app_address}/pets"))
            .json(&pet_info)
            .send()
            .await
            .expect("Failed to execute request");

        assert!(
            response.status().is_client_error(),
            "failed: posting_pet_fails -- {:?}",
            response
        );
    }

    #[tokio::test]
    async fn getting_pet_fails() {
        let app_address = spawn_test_app().await;

        let client = reqwest::Client::new();

        let response = client
            .post(format!("{app_address}/pets"))
            .query(&[("pet_id", "1")])
            .send()
            .await
            .expect("Failed to execute request");

        assert!(
            response.status().is_client_error(),
            "failed: getting_pet_fails -- {:?}",
            response
        );
    }

    #[tokio::test]
    async fn patching_pet_fails() {
        let app_address = spawn_test_app().await;

        let client = reqwest::Client::new();

        let pet_info = PetModel {
            pet_type_id: Some(1),
            name: Some(String::from("Test pet name update")),
            user_id: Some(1),
            ..Default::default()
        };

        let response = client
            .patch(format!("{app_address}/pets"))
            .json(&pet_info)
            .send()
            .await
            .expect("Failed to execute request");

        assert!(
            response.status().is_client_error(),
            "failed: patching_pet_fails -- {:?}",
            response
        );
    }

    #[tokio::test]
    async fn deleting_pet_fails() {
        let app_address = spawn_test_app().await;

        let client = reqwest::Client::new();

        let pet_info = PetModel {
            pet_id: 1,
            pet_type_id: Some(1),
            name: Some(String::from("Test pet name update")),
            user_id: Some(1),
            ..Default::default()
        };

        let response = client
            .delete(format!("{app_address}/pets"))
            .json(&pet_info)
            .send()
            .await
            .expect("Failed to execute request");

        assert!(
            response.status().is_client_error(),
            "failed: deleting_pet_fails -- {:?}",
            response
        );
    }
}

#[cfg(test)]
mod verified_user_pet_requests {
    use backend::main_test::spawn_test_app;
    use models::entities::{pets::Model as PetModel, users::Model as UserModel};
    use once_cell::sync::{Lazy, OnceCell};

    static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
        reqwest::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap()
    });

    static PET: OnceCell<PetModel> = OnceCell::new();

    async fn login_response() {
        let app_address = spawn_test_app().await;

        let login_info = UserModel {
            email: Some(String::from("tests_user@tests.com")),
            password: Some(String::from("test")),
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
            "failed: verified_user_pet_requests / login_response -- {:?}",
            login_response.json::<String>().await.expect("Failed to deserialize error")
        );
    }

    async fn pet_name_empty_fails() {
        let app_address = spawn_test_app().await;

        let pet_info = PetModel {
            pet_type_id: Some(1),
            name: Some(String::new()),
            ..Default::default()
        };

        let response = CLIENT
            .post(format!("{app_address}/pets"))
            .json(&pet_info)
            .send()
            .await
            .expect("Failed to execute request");

        assert!(
            response.status().is_client_error(),
            "failed: verified_user_pet_requests / pet_name_empty_fails -- {:?}",
            response
        );
    }

    async fn wrong_pet_type_id_fails() {
        let app_address = spawn_test_app().await;

        let pet_info = PetModel {
            pet_type_id: Some(2147483647),
            name: Some(String::from("Test pet name")),
            ..Default::default()
        };

        let response = CLIENT
            .post(format!("{app_address}/pets"))
            .json(&pet_info)
            .send()
            .await
            .expect("Failed to execute request");

        assert!(
            response.status().is_client_error(),
            "failed: verified_user_pet_requests / wrong_pet_type_id_fails -- {:?}",
            response
        );
    }

    async fn user_can_post_pet() {
        let app_address = spawn_test_app().await;

        let pet_info = PetModel {
            pet_type_id: Some(1),
            name: Some(String::from("Test pet name")),
            ..Default::default()
        };

        let response = CLIENT
            .post(format!("{app_address}/pets"))
            .json(&pet_info)
            .send()
            .await
            .expect("Failed to execute request");

        assert!(
            response.status().is_success(),
            "failed: verified_user_pet_requests / user_can_post_pet -- {:?}",
            response,
        );

        let pet_data: PetModel = response.json().await.unwrap();

        PET.set(pet_data).unwrap();
    }

    async fn user_can_get_pet() {
        let app_address = spawn_test_app().await;

        let pet_id = PET.get().unwrap().pet_id;

        let response = CLIENT
            .get(format!("{app_address}/pets"))
            .query(&[("pet_id", pet_id)])
            .send()
            .await
            .expect("Failed to execute request");

        assert!(
            response.status().is_success(),
            "failed: verified_user_pet_requests / user_can_get_pet -- {:?}",
            response
        );
    }

    async fn user_can_patch_pet() {
        let app_address = spawn_test_app().await;

        let pet_id = PET.get().unwrap().pet_id;

        let pet_info = PetModel {
            pet_id,
            pet_type_id: Some(2),
            name: Some(String::from("Test pet name updated")),
            ..Default::default()
        };

        let response = CLIENT
            .patch(format!("{app_address}/pets"))
            .json(&pet_info)
            .send()
            .await
            .expect("Failed to execute request");

        assert!(
            response.status().is_success(),
            "failed: verified_user_pet_requests / user_can_patch_pet -- {:?}",
            response
        );
    }

    async fn user_can_delete_pet() {
        let app_address = spawn_test_app().await;

        let pet_id = PET.get().unwrap().pet_id;

        let pet_info = PetModel {
            pet_id,
            ..Default::default()
        };

        let response = CLIENT
            .delete(format!("{app_address}/pets"))
            .json(&pet_info)
            .send()
            .await
            .expect("Failed to execute request");

        assert!(
            response.status().is_success(),
            "failed: verified_user_pet_requests / user_can_delete_pet -- {:?}",
            response
        );
    }

    #[tokio::test]
    async fn exec_tests() {
        login_response().await;
        pet_name_empty_fails().await;
        wrong_pet_type_id_fails().await;
        user_can_post_pet().await;
        user_can_get_pet().await;
        user_can_patch_pet().await;
        user_can_delete_pet().await;
    }
}
