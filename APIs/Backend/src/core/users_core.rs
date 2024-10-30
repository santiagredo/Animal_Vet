use crate::data::UsersDaTa;
use actix_web::http;
use bcrypt::{hash, DEFAULT_COST};
use chrono::{Local, NaiveDateTime};
use models::entities::sessions::Model as SessionModel;
use models::entities::users::Model;
use security::core::{PrivateKeyCore, SessionCore, UserRolesCore};
use security::data::SessionData;
use utils::{get_config, CodeMessage, Outcome, REMOVED};

pub struct UsersCore;

impl UsersCore {
    pub async fn insert_user(mut new_user: Model) -> Outcome<Model, CodeMessage, CodeMessage> {
        let user_role_id = match UserRolesCore::select_role(models::entities::user_roles::Model {
            name: Some(format!("user")),
            ..Default::default()
        })
        .await
        {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Failure(fail) => return Outcome::Failure(fail),
            Outcome::Success(val) => val.user_role_id,
        };

        new_user.role = Some(user_role_id);
        new_user.is_enabled = Some(true);

        // New user needs to pass all field validations
        let mut parsed_user = match backoffice::core::UsersCore::parse_user(new_user, true).await {
            Err(err) => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: err,
                })
            }
            Ok(val) => val,
        };

        if let Some(name) = parsed_user.name {
            parsed_user.name = match PrivateKeyCore::encrypt_content(name).await {
                Outcome::Error(err) => return Outcome::Error(err),
                Outcome::Failure(fail) => return Outcome::Error(fail),
                Outcome::Success(val) => Some(val),
            }
        };

        match UsersDaTa::select_user(
            &get_config().await.db_url,
            Model {
                email: parsed_user.email.clone(),
                ..Default::default()
            },
        )
        .await
        {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Success(_) => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: String::from("Email already registered"),
                })
            }
            Outcome::Failure(_) => (),
        }

        if let Some(password) = parsed_user.password {
            parsed_user.password = match hash(password, DEFAULT_COST) {
                Err(err) => {
                    return Outcome::Error(CodeMessage {
                        http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                        message: err.to_string(),
                    })
                }
                Ok(val) => Some(val),
            }
        };

        if let Some(phone_number) = parsed_user.phone_number {
            parsed_user.phone_number = match PrivateKeyCore::encrypt_content(phone_number).await {
                Outcome::Error(err) => return Outcome::Error(err),
                Outcome::Failure(fail) => return Outcome::Error(fail),
                Outcome::Success(val) => Some(val),
            }
        };

        if let Some(document_id) = parsed_user.document_id {
            parsed_user.document_id = match PrivateKeyCore::encrypt_content(document_id).await {
                Outcome::Error(err) => return Outcome::Error(err),
                Outcome::Failure(fail) => return Outcome::Error(fail),
                Outcome::Success(val) => Some(val),
            }
        };

        UsersDaTa::insert_user(&get_config().await.db_url, parsed_user).await
    }

    pub async fn select_user(user_entity: Model) -> Outcome<Model, CodeMessage, CodeMessage> {
        let mut stored_user =
            match UsersDaTa::select_user(&get_config().await.db_url, user_entity).await {
                Outcome::Error(err) => return Outcome::Error(err),
                Outcome::Failure(fail) => return Outcome::Failure(fail),
                Outcome::Success(val) => val,
            };

        if let Some(name) = stored_user.name {
            stored_user.name = match PrivateKeyCore::decrypt_content(name).await {
                Outcome::Error(err) => return Outcome::Error(err),
                Outcome::Failure(fail) => return Outcome::Error(fail),
                Outcome::Success(val) => Some(val),
            }
        };

        if stored_user.password.is_some() {
            stored_user.password = None;
        };

        if let Some(phone_number) = stored_user.phone_number {
            stored_user.phone_number = match PrivateKeyCore::decrypt_content(phone_number).await {
                Outcome::Error(err) => return Outcome::Error(err),
                Outcome::Failure(fail) => return Outcome::Error(fail),
                Outcome::Success(val) => Some(val),
            }
        };

        if let Some(document_id) = stored_user.document_id {
            stored_user.document_id = match PrivateKeyCore::decrypt_content(document_id).await {
                Outcome::Error(err) => return Outcome::Error(err),
                Outcome::Failure(fail) => return Outcome::Error(fail),
                Outcome::Success(val) => Some(val),
            }
        };

        Outcome::Success(stored_user)
    }

    pub async fn update_user(
        user_entity: Model,
        session_core: SessionCore,
    ) -> Outcome<Model, CodeMessage, CodeMessage> {
        let mut parsed_user =
            match backoffice::core::UsersCore::parse_user(user_entity, false).await {
                Err(err) => {
                    return Outcome::Failure(CodeMessage {
                        http_code: http::StatusCode::BAD_REQUEST,
                        message: err,
                    })
                }
                Ok(val) => val,
            };

        parsed_user.user_id = session_core.user.user_id;

        if let Some(name) = parsed_user.name {
            parsed_user.name = match PrivateKeyCore::encrypt_content(name).await {
                Outcome::Error(err) => return Outcome::Error(err),
                Outcome::Failure(fail) => return Outcome::Error(fail),
                Outcome::Success(val) => Some(val),
            }
        };

        match UsersDaTa::select_user(
            &get_config().await.db_url,
            Model {
                email: parsed_user.email.clone(),
                ..Default::default()
            },
        )
        .await
        {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Success(val) => {
                if val.user_id != parsed_user.user_id {
                    return Outcome::Failure(CodeMessage {
                        http_code: http::StatusCode::BAD_REQUEST,
                        message: String::from("Email already registered"),
                    });
                }
            }
            Outcome::Failure(_) => (),
        }

        if let Some(password) = parsed_user.password {
            parsed_user.password = match hash(password, DEFAULT_COST) {
                Err(err) => {
                    return Outcome::Error(CodeMessage {
                        http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                        message: err.to_string(),
                    })
                }
                Ok(val) => Some(val),
            }
        };

        if let Some(phone_number) = parsed_user.phone_number {
            parsed_user.phone_number = match PrivateKeyCore::encrypt_content(phone_number).await {
                Outcome::Error(err) => return Outcome::Error(err),
                Outcome::Failure(fail) => return Outcome::Error(fail),
                Outcome::Success(val) => Some(val),
            }
        };

        if let Some(document_id) = parsed_user.document_id {
            parsed_user.document_id = match PrivateKeyCore::encrypt_content(document_id).await {
                Outcome::Error(err) => return Outcome::Error(err),
                Outcome::Failure(fail) => return Outcome::Error(fail),
                Outcome::Success(val) => Some(val),
            }
        };

        match UsersDaTa::update_user(&get_config().await.db_url, parsed_user).await {
            Err(err) => Outcome::Error(err),
            Ok(val) => Outcome::Success(val),
        }
    }

    pub async fn delete_user(mut user_model: Model) -> Outcome<Model, CodeMessage, CodeMessage> {
        let current_date = Local::now();

        let session_updates = SessionModel {
            is_enabled: Some(false),
            latest_update_date: Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            )),
            closing_date: Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            )),
            ..Default::default()
        };

        let session_conditions = SessionModel {
            user_id: Some(user_model.user_id),
            ..Default::default()
        };

        match SessionData::update_multiple_sessions(
            &get_config().await.db_url,
            session_updates,
            session_conditions,
        )
        .await
        {
            Err(err) => return Outcome::Error(err),
            Ok(_) => (),
        };

        user_model.name = Some(String::from(REMOVED));
        user_model.email = Some(String::from(REMOVED));
        user_model.password = Some(String::from(REMOVED));
        user_model.phone_number = Some(String::from(REMOVED));
        user_model.document_id = Some(String::from(REMOVED));
        user_model.is_enabled = Some(false);

        match UsersDaTa::update_user(&get_config().await.db_url, user_model).await {
            Err(err) => Outcome::Error(err),
            Ok(val) => Outcome::Success(val),
        }
    }
}
