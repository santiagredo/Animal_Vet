use actix_web::http;
use models::entities::pets::Model;
use security::core::PrivateKeyCore;
use utils::{get_config, CodeMessage, Outcome, REMOVED};

use crate::data::PetsData;

use super::PetTypesCore;

pub struct PetsCore;

impl PetsCore {
    pub async fn insert_pet(mut pet_model: Model) -> Outcome<Model, CodeMessage, CodeMessage> {
        let pet_type_id = match pet_model.pet_type_id {
            None => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: String::from("Missing pet type id"),
                })
            }
            Some(0) => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: String::from("Invalid pet type id"),
                })
            }
            Some(val) => val,
        };

        match PetTypesCore::select_pet_types(models::entities::pet_types::Model {
            pet_type_id,
            ..Default::default()
        })
        .await
        {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Failure(fail) => return Outcome::Failure(fail),
            Outcome::Success(_) => (),
        };

        match pet_model.name.as_ref() {
            None => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: String::from("Missing pet name"),
                })
            }
            name if name.as_ref().is_some_and(|x| x.is_empty()) => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: String::from("Pet name is empty"),
                })
            }
            Some(val) => val,
        };

        if let Some(pet_name) = pet_model.name {
            pet_model.name = match PrivateKeyCore::encrypt_content(pet_name).await {
                Outcome::Error(err) => return Outcome::Error(err),
                Outcome::Failure(fail) => return Outcome::Error(fail),
                Outcome::Success(val) => Some(val),
            }
        };

        PetsData::insert_pet(&get_config().await.db_url, pet_model).await
    }

    pub async fn select_pets(pet_model: Model) -> Outcome<Vec<Model>, CodeMessage, CodeMessage> {
        PetsData::select_pets(&get_config().await.db_url, pet_model).await
    }

    pub async fn update_pet(mut pet_model: Model) -> Outcome<Model, CodeMessage, CodeMessage> {
        if let Some(pet_type_id) = pet_model.pet_type_id {
            match PetTypesCore::select_pet_types(models::entities::pet_types::Model {
                pet_type_id,
                ..Default::default()
            })
            .await
            {
                Outcome::Error(err) => return Outcome::Error(err),
                Outcome::Failure(fail) => return Outcome::Failure(fail),
                Outcome::Success(_) => (),
            };
        }

        if let Some(pet_name) = pet_model.name {
            pet_model.name = match PrivateKeyCore::encrypt_content(pet_name).await {
                Outcome::Error(err) => return Outcome::Error(err),
                Outcome::Failure(fail) => return Outcome::Failure(fail),
                Outcome::Success(val) => Some(val),
            };
        }

        PetsData::update_pet(&get_config().await.db_url, pet_model).await
    }

    pub async fn delete_pet(mut pet_model: Model) -> Outcome<Model, CodeMessage, CodeMessage> {
        pet_model.name = Some(String::from(REMOVED));

        PetsData::update_pet(&get_config().await.db_url, pet_model).await
    }
}
