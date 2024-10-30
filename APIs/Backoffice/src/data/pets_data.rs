use actix_web::http;
use chrono::{Local, NaiveDateTime};
use models::entities::pet_events;
use models::entities::pets::{self, Column, Entity as Pets, Model};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter,
};
use tracing::error_span;
use utils::{CodeMessage, Outcome};

pub struct PetsData;

impl PetsData {
    pub async fn insert_pet(
        db: &DatabaseConnection,
        pet_model: Model,
    ) -> Outcome<Model, CodeMessage, CodeMessage> {
        let current_date = Local::now();

        let pet = pets::ActiveModel {
            pet_type_id: ActiveValue::Set(pet_model.pet_type_id),
            name: ActiveValue::Set(pet_model.name),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            user_id: ActiveValue::Set(pet_model.user_id),
            additional_comments: ActiveValue::Set(pet_model.additional_comments),
            ..Default::default()
        };

        match pet.insert(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);
                Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                })
            }
            Ok(val) => Outcome::Success(val),
        }
    }

    pub async fn select_pets(
        db: &DatabaseConnection,
        pet_model: Model,
    ) -> Outcome<Vec<Model>, CodeMessage, CodeMessage> {
        let mut condition = Condition::all();

        if pet_model.pet_id != 0 {
            condition = condition.add(Column::PetId.eq(pet_model.pet_id));
        }

        if pet_model.name.as_ref().is_some_and(|x| !x.is_empty()) {
            condition = condition.add(Column::Name.eq(pet_model.name));
        }

        if pet_model.user_id.is_some_and(|x| x != 0) {
            condition = condition.add(Column::UserId.eq(pet_model.user_id));
        }

        if condition.len() == 0 {
            return Outcome::Failure(CodeMessage {
                http_code: http::StatusCode::BAD_REQUEST,
                message: String::from("No searching parameters"),
            });
        };

        match Pets::find().filter(condition).all(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                })
            }
            Ok(val) => {
                if val.len() < 1 {
                    return Outcome::Failure(CodeMessage {
                        http_code: http::StatusCode::BAD_REQUEST,
                        message: String::from("Pet not found"),
                    });
                }

                Outcome::Success(val)
            }
        }
    }

    pub async fn update_pet(
        db: &DatabaseConnection,
        pet_model: Model,
    ) -> Outcome<Model, CodeMessage, CodeMessage> {
        let current_date = Local::now();

        let mut pet = pets::ActiveModel {
            pet_id: ActiveValue::Unchanged(pet_model.pet_id),
            user_id: ActiveValue::Unchanged(pet_model.user_id),
            latest_update_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        if pet_model.pet_type_id.is_some_and(|x| x != 0) {
            pet.pet_type_id = ActiveValue::set(pet_model.pet_type_id);
        }

        if pet_model.name.as_ref().is_some_and(|x| !x.is_empty()) {
            pet.name = ActiveValue::set(pet_model.name);
        }

        if pet_model
            .additional_comments
            .as_ref()
            .is_some_and(|x| !x.is_empty())
        {
            pet.additional_comments = ActiveValue::set(pet_model.additional_comments);
        }

        let updated_pet = match pet.update(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                });
            }
            Ok(val) => val,
        };

        let updated_pet_id = updated_pet.pet_id;
        let user_id = updated_pet.user_id.unwrap_or(0);

        let pet_event = pet_events::ActiveModel {
            pet_id: ActiveValue::Set(Some(updated_pet_id)),
            details: ActiveValue::Set(Some(format!(
                "Pet {updated_pet_id} updated by existing user {user_id}"
            ))),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        if let Err(err) = pet_event.insert(db).await {
            error_span!("error - database", error = ?err);
        };

        Outcome::Success(updated_pet)
    }
}
