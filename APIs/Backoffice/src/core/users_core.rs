use crate::data::UsersDaTa;
use actix_web::http;
use bcrypt::{hash, DEFAULT_COST};
use chrono::{Local, NaiveDateTime};
use models::entities::sessions::Model as SessionModel;
use models::entities::users::Model;
use security::core::{PrivateKeyCore, SessionCore, UserRolesCore};
use security::data::SessionData;
use utils::{get_config, CodeMessage, Outcome, Validator, REMOVED};

pub struct UsersCore;

impl UsersCore {
    pub async fn insert_user(
        new_user: Model,
        session_core: SessionCore,
    ) -> Outcome<Model, CodeMessage, CodeMessage> {
        // New user needs to pass all field validations
        let mut parsed_user = match UsersCore::parse_user(new_user, true).await {
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

        UsersDaTa::insert_user(&get_config().await.db_url, parsed_user, session_core).await
    }

    pub async fn select_user(
        user_entity: Model,
    ) -> Outcome<Model, CodeMessage, CodeMessage> {
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
        let mut parsed_user = match UsersCore::parse_user(user_entity, false).await {
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

        match UsersDaTa::update_user(&get_config().await.db_url, parsed_user, session_core)
            .await
        {
            Err(err) => Outcome::Error(err),
            Ok(val) => Outcome::Success(val),
        }
    }

    pub async fn delete_user(
        mut user_model: Model,
        session_core: SessionCore,
    ) -> Outcome<Model, CodeMessage, CodeMessage> {
        let stored_user = match UsersCore::select_user(Model {
            user_id: user_model.user_id,
            ..Default::default()
        })
        .await
        {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Failure(fail) => return Outcome::Failure(fail),
            Outcome::Success(val) => val,
        };

        let stored_user_role = match stored_user.role {
            None => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: String::from("Missing stored user role id"),
                })
            }
            Some(val) => val,
        };

        match UserRolesCore::select_role(models::entities::user_roles::Model {
            user_role_id: stored_user_role,
            ..Default::default()
        })
        .await
        {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Failure(fail) => return Outcome::Failure(fail),
            Outcome::Success(role) => {
                if role.hierarchy_level.is_none() {
                    return Outcome::Failure(CodeMessage {
                        http_code: http::StatusCode::BAD_REQUEST,
                        message: String::from("Missing stored user role"),
                    });
                };

                let session_hierarchy_level = match session_core.role.hierarchy_level {
                    None => {
                        return Outcome::Failure(CodeMessage {
                            http_code: http::StatusCode::BAD_REQUEST,
                            message: String::from("Missing hierarchy level from session user"),
                        })
                    }
                    Some(val) => val,
                };

                if role
                    .hierarchy_level
                    .is_some_and(|level| level <= session_hierarchy_level)
                {
                    return Outcome::Failure(CodeMessage {
                        http_code: http::StatusCode::BAD_REQUEST,
                        message: String::from(
                            "User can't delete another user with same or lower hierarchy level",
                        ),
                    });
                }
            }
        };

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

        match UsersDaTa::update_user(&get_config().await.db_url, user_model, session_core)
            .await
        {
            Err(err) => Outcome::Error(err),
            Ok(val) => Outcome::Success(val),
        }
    }
}

impl UsersCore {
    pub async fn parse_user(new_user: Model, err_on_none: bool) -> Result<Model, String> {
        let user_id = new_user.user_id;

        let mut name = match new_user.name {
            val if val.is_none() && err_on_none => return Err(format!("Name cannot be empty")),
            None => None,
            Some(val) => Some(val),
        };

        if name.is_some() {
            name = match Validator::validate_empty_field(name, "Name") {
                Err(err) => return Err(err),
                Ok(val) => Some(val),
            }
        }

        let mut email = match new_user.email {
            val if val.is_none() && err_on_none => return Err(format!("Email cannot be empty")),
            None => None,
            Some(val) => Some(val),
        };

        if email.is_some() {
            email = match Validator::validate_empty_field(email, "Email") {
                Err(err) => return Err(err),
                Ok(val) => Some(val),
            }
        }

        let mut password = match new_user.password {
            val if val.is_none() && err_on_none => return Err(format!("Password cannot be empty")),
            None => None,
            Some(val) => Some(val),
        };

        if password.is_some() {
            password = match Validator::validate_empty_field(password, "Password") {
                Err(err) => return Err(err),
                Ok(val) => Some(val),
            }
        }

        let mut phone_number = match new_user.phone_number {
            val if val.is_none() && err_on_none => {
                return Err(format!("Phone number cannot be empty"))
            }
            None => None,
            Some(val) => Some(val),
        };

        if phone_number.is_some() {
            phone_number = match Validator::validate_empty_field(phone_number, "Phone number") {
                Err(err) => return Err(err),
                Ok(val) => Some(val),
            }
        }

        let mut document_id = match new_user.document_id {
            val if val.is_none() && err_on_none => {
                return Err(format!("Document ID cannot be empty"))
            }
            None => None,
            Some(val) => Some(val),
        };

        if document_id.is_some() {
            document_id = match Validator::validate_empty_field(document_id, "Document ID") {
                Err(err) => return Err(err),
                Ok(val) => Some(val),
            }
        }

        let role = match new_user.role {
            val if val.is_none() && err_on_none => {
                return Err(String::from("Missing new user role id"))
            }
            None => None,
            Some(val) => Some(val),
        };

        let is_enabled = match new_user.is_enabled {
            None => None,
            Some(val) => Some(val),
        };

        Ok(Model {
            user_id,
            name,
            email,
            password,
            phone_number,
            document_id,
            role,
            is_enabled,
            ..Default::default()
        })
    }
}
