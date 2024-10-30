use actix_session::Session;
use actix_web::http;
use bcrypt::verify;
use tracing::error_span;
use utils::{
    get_config, CodeMessage, Outcome, SessionDetails, Validator, SESSION_ID, SESSION_UUID, USER_ID,
};
use uuid::Uuid;

use super::UserRolesCore;
use crate::data::{SessionData, UsersDaTa};
use models::entities::sessions::Model as SessionModel;
use models::entities::user_roles::Model as UserRoleModel;
use models::entities::users::Model as UserModel;

pub struct SessionCore {
    pub user: UserModel,
    pub session: SessionModel,
    pub role: UserRoleModel,
}

impl SessionCore {
    pub async fn insert_session(
        mut user_model: UserModel,
    ) -> Outcome<SessionDetails, CodeMessage, CodeMessage> {
        let email = match Validator::validate_empty_field(user_model.email.clone(), "Email") {
            Err(err) => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: err,
                })
            }
            Ok(val) => val,
        };

        user_model.email = Some(email);

        let password =
            match Validator::validate_empty_field(user_model.password.clone(), "Password") {
                Err(err) => {
                    return Outcome::Failure(CodeMessage {
                        http_code: http::StatusCode::BAD_REQUEST,
                        message: err,
                    })
                }
                Ok(val) => val,
            };

        user_model.password = None;

        let stored_user =
            match UsersDaTa::select_user(&get_config().await.db_url, user_model).await {
                Outcome::Error(err) => return Outcome::Error(err),
                Outcome::Failure(fail) => return Outcome::Failure(fail),
                Outcome::Success(val) => val,
            };

        let stored_user_password = match stored_user.password.clone() {
            None => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: format!("Stored password is empty"),
                })
            }
            Some(val) => val,
        };

        match verify(password, &stored_user_password) {
            Err(err) => {
                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                })
            }
            Ok(false) => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: format!("Invalid password"),
                })
            }
            Ok(true) => (),
        }

        match stored_user.is_enabled {
            Some(true) => (),
            _ => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::UNAUTHORIZED,
                    message: String::from("User disabled"),
                })
            }
        };

        match SessionData::insert_session(&get_config().await.db_url, stored_user).await {
            Err(err) => Outcome::Error(err),
            Ok(val) => {
                let user_id = match val.user_id {
                    None => 0,
                    Some(val) => val,
                };

                let session_uuid = match val.session_uuid {
                    None => Uuid::nil(),
                    Some(val) => val,
                };

                Outcome::Success(SessionDetails {
                    session_id: val.session_id,
                    user_id,
                    session_uuid,
                })
            }
        }
    }

    pub async fn select_session(
        session_model: SessionModel,
    ) -> Outcome<SessionModel, CodeMessage, CodeMessage> {
        SessionData::select_session(&get_config().await.db_url, session_model).await
    }

    pub async fn update_session(session_model: SessionModel) -> Result<SessionModel, CodeMessage> {
        SessionData::update_session(&get_config().await.db_url, session_model).await
    }
}

impl SessionCore {
    pub fn session_details_extractor(
        session: &Session,
    ) -> Outcome<SessionDetails, CodeMessage, CodeMessage> {
        let session_id: i32 = match session.get::<i32>("session_id") {
            Err(err) => {
                error_span!("error - cookie", error = ?err);

                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                });
            }
            Ok(None) => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: String::from("Missing session id"),
                })
            }
            Ok(Some(val)) => val,
        };

        let user_id: i32 = match session.get::<i32>("user_id") {
            Err(err) => {
                error_span!("error - cookie", error = ?err);

                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                });
            }
            Ok(None) => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: String::from("Missing user id"),
                })
            }
            Ok(Some(val)) => val,
        };

        let session_uuid: Uuid = match session.get::<Uuid>("session_uuid") {
            Err(err) => {
                error_span!("error - cookie", error = ?err);

                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                });
            }
            Ok(None) => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: String::from("Missing session uuid"),
                })
            }
            Ok(Some(val)) => val,
        };

        Outcome::Success(SessionDetails {
            session_id,
            user_id,
            session_uuid,
        })
    }

    pub async fn session_validator(
        session: &mut Session,
    ) -> Outcome<Self, CodeMessage, CodeMessage> {
        let session_details = match Self::session_details_extractor(&session) {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Failure(fail) => return Outcome::Failure(fail),
            Outcome::Success(val) => val,
        };

        let stored_user = match UsersDaTa::select_user(
            &get_config().await.db_url,
            UserModel {
                user_id: session_details.user_id,
                ..Default::default()
            },
        )
        .await
        {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Failure(fail) => return Outcome::Failure(fail),
            Outcome::Success(val) => val,
        };

        match stored_user.is_enabled {
            Some(true) => (),
            _ => {
                session.remove(SESSION_ID);
                session.remove(USER_ID);
                session.remove(SESSION_UUID);

                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::UNAUTHORIZED,
                    message: String::from("User is disabled"),
                });
            }
        }

        let stored_session = match Self::select_session(SessionModel {
            session_id: session_details.session_id,
            session_uuid: Some(session_details.session_uuid),
            ..Default::default()
        })
        .await
        {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Failure(fail) => return Outcome::Failure(fail),
            Outcome::Success(val) => val,
        };

        match stored_session.is_enabled {
            Some(true) => (),
            _ => {
                session.remove(SESSION_ID);
                session.remove(USER_ID);
                session.remove(SESSION_UUID);

                session.purge();

                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::UNAUTHORIZED,
                    message: String::from("Session expired"),
                });
            }
        }

        let stored_user_role = match UserRolesCore::select_role(UserRoleModel {
            user_role_id: stored_user.role.unwrap_or(0),
            ..Default::default()
        })
        .await
        {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Failure(fail) => return Outcome::Failure(fail),
            Outcome::Success(val) => val,
        };

        Outcome::Success(Self {
            user: stored_user,
            session: stored_session,
            role: stored_user_role,
        })
    }
}
