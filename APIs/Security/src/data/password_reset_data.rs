use actix_web::http;
use chrono::{Local, NaiveDateTime};
use models::entities::password_reset::{self, Column, Model as PasswordResetModel};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter,
};
use tracing::error_span;
use utils::{CodeMessage, Outcome};

pub struct PasswordResetData;

impl PasswordResetData {
    pub async fn insert_reset_token(
        db: &DatabaseConnection,
        user_id: i32,
        reset_token: String,
    ) -> Result<PasswordResetModel, CodeMessage> {
        let current_date = Local::now();

        let password_reset = password_reset::ActiveModel {
            user_id: ActiveValue::Set(Some(user_id)),
            is_enabled: ActiveValue::Set(Some(true)),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            token: ActiveValue::Set(Some(reset_token)),
            ..Default::default()
        };

        match password_reset.insert(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Err(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                });
            }
            Ok(val) => Ok(val),
        }
    }

    pub async fn select_reset_tokens(
        db: &DatabaseConnection,
        password_reset_model: PasswordResetModel,
    ) -> Outcome<Vec<PasswordResetModel>, CodeMessage, CodeMessage> {
        let mut condition = Condition::all();

        if password_reset_model.id != 0 {
            condition = condition.add(Column::Id.eq(password_reset_model.id))
        }

        if password_reset_model.user_id.is_some_and(|x| x != 0) {
            condition = condition.add(Column::UserId.eq(password_reset_model.user_id))
        }

        if password_reset_model.is_enabled.as_ref().is_some() {
            condition = condition.add(Column::IsEnabled.eq(password_reset_model.is_enabled))
        }

        if password_reset_model
            .token
            .as_ref()
            .is_some_and(|x| !x.is_empty())
        {
            condition = condition.add(Column::Token.eq(password_reset_model.token))
        }

        if condition.len() > 0 {
            match models::entities::password_reset::Entity::find()
                .filter(condition)
                .all(db)
                .await
            {
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
                            message: format!("Reset Tokens not found"),
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

    pub async fn update_reset_token(
        db: &DatabaseConnection,
        password_reset_model: PasswordResetModel,
    ) -> Outcome<PasswordResetModel, CodeMessage, CodeMessage> {
        let current_date = Local::now();

        let password_reset = password_reset::ActiveModel {
            id: ActiveValue::Unchanged(password_reset_model.id),
            update_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            is_enabled: ActiveValue::Set(Some(false)),
            ..Default::default()
        };

        match password_reset.update(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                if let DbErr::RecordNotFound(_) = err {
                    return Outcome::Failure(CodeMessage {
                        http_code: http::StatusCode::BAD_REQUEST,
                        message: format!("No reset token was updated"),
                    });
                };

                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                });
            }
            Ok(val) => Outcome::Success(val),
        }
    }

    pub async fn update_user_password(
        db: &DatabaseConnection,
        user_model: models::entities::users::Model,
    ) -> Outcome<models::entities::users::Model, CodeMessage, CodeMessage> {
        let current_date = Local::now();

        let mut user = models::entities::users::ActiveModel {
            email: ActiveValue::Unchanged(user_model.email),
            latest_update_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        if user_model
            .password
            .as_ref()
            .is_some_and(|password| !password.is_empty())
        {
            user.password = ActiveValue::Set(user_model.password);
        }

        let updated_user = match user.update(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                if let DbErr::RecordNotFound(_) = err {
                    return Outcome::Failure(CodeMessage {
                        http_code: http::StatusCode::BAD_REQUEST,
                        message: format!("No user was updated"),
                    });
                };

                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                });
            }
            Ok(val) => val,
        };

        let current_date = Local::now();
        let updated_user_id = updated_user.user_id;

        let user_event = models::entities::user_events::ActiveModel {
            user_id: ActiveValue::Set(Some(updated_user_id)),
            details: ActiveValue::Set(Some(format!(
                "User {updated_user_id} updated by existing user {updated_user_id} -- password reset"
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

        Outcome::Success(updated_user)
    }
}
