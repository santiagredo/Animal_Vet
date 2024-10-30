use actix_web::http;
use models::entities::{
    pet_types::{Column, Model},
    prelude::PetTypes,
};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};
use tracing::error_span;
use utils::{get_config, CodeMessage, Outcome};

pub struct PetTypesData;

impl PetTypesData {
    pub async fn select_pet_types(
        pet_type_model: Model,
    ) -> Outcome<Vec<Model>, CodeMessage, CodeMessage> {
        let db = &get_config().await.db_url;

        let mut condition = Condition::all();

        if pet_type_model.pet_type_id != 0 {
            condition = condition.add(Column::PetTypeId.eq(pet_type_model.pet_type_id));
        }

        if pet_type_model.name.as_ref().is_some_and(|x| !x.is_empty()) {
            condition = condition.add(Column::Name.eq(pet_type_model.name));
        }

        if let Some(is_enabled) = pet_type_model.is_enabled {
            condition = condition.add(Column::IsEnabled.eq(is_enabled));
        }

        if condition.len() > 0 {
            match PetTypes::find().filter(condition).all(db).await {
                Err(err) => {
                    error_span!("error - database", error = ?err);
                    return Outcome::Error(CodeMessage {
                        http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                        message: err.to_string(),
                    });
                }
                Ok(val) => {
                    if val.len() < 1 {
                        return Outcome::Failure(CodeMessage {
                            http_code: http::StatusCode::BAD_REQUEST,
                            message: format!("Pet types not found")
                        });
                    };

                    return Outcome::Success(val)
                }
            }
        }

        Outcome::Failure(CodeMessage {
            http_code: http::StatusCode::BAD_REQUEST,
            message: String::from("No searching parameters"),
        })
    }
}
