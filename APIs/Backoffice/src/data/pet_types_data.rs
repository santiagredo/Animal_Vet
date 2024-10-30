use actix_web::http;
use chrono::{Local, NaiveDateTime};
use models::entities::{
    pet_type_events,
    pet_types::{self, Column, Model},
    prelude::PetTypes,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DbErr, EntityTrait, QueryFilter,
};
use security::core::SessionCore;
use tracing::error_span;
use utils::{get_config, CodeMessage, Outcome};

pub struct PetTypesData;

impl PetTypesData {
    pub async fn insert_pet_type(
        pet_type_model: Model,
        session_core: SessionCore,
    ) -> Outcome<Model, CodeMessage, CodeMessage> {
        let db = &get_config().await.db_url;

        let current_date = Local::now();

        let pet_type = pet_types::ActiveModel {
            name: ActiveValue::Set(pet_type_model.name),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            is_enabled: ActiveValue::Set(pet_type_model.is_enabled),
            ..Default::default()
        };

        let inserted_pet_type = match pet_type.insert(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                });
            }
            Ok(val) => val,
        };

        let inserted_pet_type_id = inserted_pet_type.pet_type_id;
        let existing_user_id = session_core.user.user_id;

        let pet_type_event = pet_type_events::ActiveModel {
            pet_type_id: ActiveValue::Set(Some(inserted_pet_type_id)),
            details: ActiveValue::Set(Some(format!("Pet type id {inserted_pet_type_id} inserted by existing user id {existing_user_id}"))),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        if let Err(err) = pet_type_event.insert(db).await {
            error_span!("error - database", error = ?err);
        }

        Outcome::Success(inserted_pet_type)
    }

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
                            message: format!("Pet types not found"),
                        });
                    }

                    return Outcome::Success(val);
                }
            }
        };

        Outcome::Failure(CodeMessage {
            http_code: http::StatusCode::BAD_REQUEST,
            message: String::from("No searching parameters"),
        })
    }

    pub async fn update_pet_type(
        pet_type_model: Model,
        session_core: SessionCore,
    ) -> Outcome<Model, CodeMessage, CodeMessage> {
        let db = &get_config().await.db_url;

        let current_date = Local::now();

        let mut pet_type = pet_types::ActiveModel {
            pet_type_id: ActiveValue::Unchanged(pet_type_model.pet_type_id),
            latest_update_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        if pet_type_model.name.as_ref().is_some_and(|x| !x.is_empty()) {
            pet_type.name = ActiveValue::set(pet_type_model.name);
        }

        if pet_type_model.is_enabled.as_ref().is_some() {
            pet_type.is_enabled = ActiveValue::set(pet_type_model.is_enabled);
        }

        let updated_pet_type = match pet_type.update(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                if let DbErr::RecordNotFound(_) = err {
                    return Outcome::Failure(CodeMessage {
                        http_code: http::StatusCode::BAD_REQUEST,
                        message: format!("No pet type was updated"),
                    });
                }

                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                });
            }
            Ok(val) => val,
        };

        let updated_pet_type_id = updated_pet_type.pet_type_id;
        let existing_user_id = session_core.user.user_id;

        let pet_type_event = pet_type_events::ActiveModel {
            pet_type_id: ActiveValue::Set(Some(updated_pet_type_id)),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            details: ActiveValue::Set(Some(format!("Existing pet type id {updated_pet_type_id} updated by existing user {existing_user_id}"))),
            ..Default::default()
        };

        if let Err(err) = pet_type_event.update(db).await {
            error_span!("error - database", error = ?err);
        }

        Outcome::Success(updated_pet_type)
    }

    pub async fn delete_pet_type(
        pet_type_model: Model,
        session_core: SessionCore,
    ) -> Outcome<u64, CodeMessage, CodeMessage> {
        let db = &get_config().await.db_url;

        let rows_affected = match PetTypes::delete_by_id(pet_type_model.pet_type_id)
            .exec(db)
            .await
        {
            Err(err) => {
                error_span!("error - database", error = ?err);

                if let DbErr::Exec(_) = err {
                    return Outcome::Failure(CodeMessage {
                        http_code: http::StatusCode::BAD_REQUEST,
                        message: format!("No pet type was deleted"),
                    });
                }

                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                });
            }
            Ok(val) => val.rows_affected,
        };

        let current_date = Local::now();
        let deleted_pet_type_id = pet_type_model.pet_type_id;
        let session_user_id = session_core.user.user_id;

        let pet_type_event = pet_type_events::ActiveModel {
            pet_type_id: ActiveValue::Set(Some(deleted_pet_type_id)),
            details: ActiveValue::Set(Some(format!(
                "Pet type id {deleted_pet_type_id} deleted by existing user id {session_user_id}"
            ))),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        if let Err(err) = pet_type_event.insert(db).await {
            error_span!("error - database", error = ?err);
        };

        Outcome::Success(rows_affected)
    }
}
