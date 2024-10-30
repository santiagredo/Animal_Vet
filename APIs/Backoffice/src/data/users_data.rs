use actix_web::http;
use chrono::{Local, NaiveDateTime};
use models::entities::user_events;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter,
};
use security::core::SessionCore;
use tracing::error_span;
use utils::{CodeMessage, Outcome};

use models::entities::users::{self, Column, Entity as Users, Model as UserModel};

pub struct UsersDaTa;

impl UsersDaTa {
    pub async fn insert_user(
        db: &DatabaseConnection,
        user_model: UserModel,
        session_core: SessionCore,
    ) -> Outcome<UserModel, CodeMessage, CodeMessage> {
        let current_date = Local::now();

        let user = users::ActiveModel {
            name: ActiveValue::Set(user_model.name),
            email: ActiveValue::Set(user_model.email),
            password: ActiveValue::Set(user_model.password),
            phone_number: ActiveValue::Set(user_model.phone_number),
            document_id: ActiveValue::Set(user_model.document_id),
            role: ActiveValue::Set(user_model.role),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            is_enabled: ActiveValue::Set(user_model.is_enabled),
            ..Default::default()
        };

        let inserted_user = match user.insert(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                });
            }
            Ok(val) => val,
        };

        let inserted_user_id = inserted_user.user_id;
        let existing_user_id = session_core.user.user_id;

        let user_event = user_events::ActiveModel {
            user_id: ActiveValue::Set(Some(inserted_user_id)),
            details: ActiveValue::Set(Some(format!(
                "User id {inserted_user_id} inserted by existing user id {existing_user_id}"
            ))),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        if let Err(err) = user_event.insert(db).await {
            error_span!("error - database", error = ?err);
        }

        Outcome::Success(inserted_user)
    }

    pub async fn select_user(
        db: &DatabaseConnection,
        user_model: UserModel,
    ) -> Outcome<UserModel, CodeMessage, CodeMessage> {
        let mut condition = Condition::all();

        if user_model.user_id != 0 {
            condition = condition.add(Column::UserId.eq(user_model.user_id));
        }

        if user_model.name.as_ref().is_some_and(|x| !x.is_empty()) {
            condition = condition.add(Column::Name.eq(user_model.name));
        }

        if user_model.email.as_ref().is_some_and(|x| !x.is_empty()) {
            condition = condition.add(Column::Email.eq(user_model.email))
        }

        if user_model.password.as_ref().is_some_and(|x| !x.is_empty()) {
            condition = condition.add(Column::Password.eq(user_model.password));
        }

        if user_model
            .phone_number
            .as_ref()
            .is_some_and(|x| !x.is_empty())
        {
            condition = condition.add(Column::PhoneNumber.eq(user_model.phone_number));
        }

        if user_model
            .document_id
            .as_ref()
            .is_some_and(|x| !x.is_empty())
        {
            condition = condition.add(Column::DocumentId.eq(user_model.document_id));
        }

        if user_model.role.is_some() {
            condition = condition.add(Column::Role.eq(user_model.role));
        }

        if user_model.is_enabled.is_some() {
            condition = condition.add(Column::IsEnabled.eq(user_model.is_enabled.unwrap_or(false)));
        }

        if condition.len() > 0 {
            match Users::find().filter(condition).one(db).await {
                Err(err) => {
                    error_span!("error - database", error = ?err);
                    return Outcome::Error(CodeMessage {
                        http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                        message: err.to_string(),
                    });
                }
                Ok(None) => {
                    return Outcome::Failure(CodeMessage {
                        http_code: http::StatusCode::BAD_REQUEST,
                        message: String::from("User not found"),
                    })
                }
                Ok(Some(val)) => return Outcome::Success(val),
            }
        };

        Outcome::Failure(CodeMessage {
            http_code: http::StatusCode::BAD_REQUEST,
            message: String::from("No searching parameters"),
        })
    }

    pub async fn update_user(
        db: &DatabaseConnection,
        user_model: UserModel,
        session_core: SessionCore,
    ) -> Result<users::Model, CodeMessage> {
        let current_date = Local::now();

        let mut user = users::ActiveModel {
            user_id: ActiveValue::Unchanged(user_model.user_id),
            latest_update_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        if user_model
            .name
            .as_ref()
            .is_some_and(|name| !name.is_empty())
        {
            user.name = ActiveValue::Set(user_model.name);
        }

        if user_model
            .email
            .as_ref()
            .is_some_and(|email| !email.is_empty())
        {
            user.email = ActiveValue::Set(user_model.email);
        }

        if user_model
            .password
            .as_ref()
            .is_some_and(|password| !password.is_empty())
        {
            user.password = ActiveValue::Set(user_model.password);
        }

        if user_model
            .phone_number
            .as_ref()
            .is_some_and(|phone_number| !phone_number.is_empty())
        {
            user.phone_number = ActiveValue::Set(user_model.phone_number);
        }

        if user_model
            .document_id
            .as_ref()
            .is_some_and(|document_id| !document_id.is_empty())
        {
            user.document_id = ActiveValue::Set(user_model.document_id);
        }

        if user_model.is_enabled.as_ref().is_some() {
            user.is_enabled = ActiveValue::Set(user_model.is_enabled);
        }

        let updated_user = match user.update(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);
                return Err(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                });
            }
            Ok(val) => val,
        };

        let updated_user_id = updated_user.user_id;
        let existing_user_id = session_core.user.user_id;

        let user_event = user_events::ActiveModel {
            user_id: ActiveValue::Set(Some(updated_user_id)),
            details: ActiveValue::Set(Some(format!(
                "User id {updated_user_id} updated by existing user id {existing_user_id}"
            ))),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        if let Err(err) = user_event.insert(db).await {
            error_span!("error - database", error = ?err);
        };

        Ok(updated_user)
    }

    // pub async fn delete_single_user(
    //     user_model: UserModel,
    //     session_core: SessionCore,
    // ) -> Result<u64, String> {
    //     let db = &get_config().await.db_url;

    //     let rows_affected = match Users::delete_by_id(user_model.user_id).exec(db).await {
    //         Err(err) => {
    //             error_span!("error - database", error = ?err);
    //             return Err(err.to_string());
    //         }
    //         Ok(val) => val.rows_affected,
    //     };

    //     let creation_date = Local::now();
    //     let deleted_user_id = user_model.user_id;
    //     let session_user_id = session_core.user.user_id;

    //     let user_event = user_events::ActiveModel {
    //         user_id: ActiveValue::Set(Some(deleted_user_id)),
    //         details: ActiveValue::Set(format!(
    //             "User id {deleted_user_id} deleted by existing user id {session_user_id}"
    //         )),
    //         creation_date: ActiveValue::Set(creation_date.into()),
    //         ..Default::default()
    //     };

    //     if let Err(err) = user_event.insert(db).await {
    //         error_span!("error - database", error = ?err);
    //     };

    //     Ok(rows_affected)
    // }
}
