use std::collections::HashMap;

use actix_web::http;
use bcrypt::{hash, DEFAULT_COST};
use mailgun_rs::{EmailAddress, Mailgun, MailgunRegion, Message};
use models::entities::{password_reset::Model as PasswordResetModel, users::Model as UserModel};
use utils::{get_config, CodeMessage, Outcome, MAILGUN_API_KEY, MAILGUN_DOMAIN};
use uuid::Uuid;

use crate::data::{PasswordResetData, UsersDaTa};

use super::{PrivateKeyCore, SettingsCore};

pub struct PasswordResetCore;

impl PasswordResetCore {
    pub async fn insert_reset_token(user_model: UserModel) -> Result<String, CodeMessage> {
        let mut stored_user = match UsersDaTa::select_user(
            &get_config().await.db_url,
            UserModel {
                user_id: 0,
                email: user_model.email,
                ..Default::default()
            },
        )
        .await
        {
            Outcome::Error(err) => return Err(err),
            Outcome::Failure(_) => return Ok(format!("Check your inbox to reset your password")),
            Outcome::Success(val) => val,
        };

        if let Some(name) = stored_user.name {
            stored_user.name = match PrivateKeyCore::decrypt_content(name).await {
                Outcome::Error(err) => return Err(err),
                Outcome::Failure(fail) => return Err(fail),
                Outcome::Success(val) => Some(val),
            }
        };

        let stored_reset_token_model = PasswordResetData::insert_reset_token(
            &get_config().await.db_url,
            stored_user.user_id,
            Uuid::new_v4().to_string(),
        )
        .await?;

        let reset_token = match stored_reset_token_model.token {
            None => {
                return Err(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: format!("Failed to retreive reset token"),
                })
            }
            Some(val) => val,
        };

        let mailgun_api_key =
            match SettingsCore::select_setting(models_settings::entities::settings::Model {
                name: format!("{MAILGUN_API_KEY}"),
                ..Default::default()
            })
            .await
            {
                Outcome::Error(err) => return Err(err),
                Outcome::Failure(fail) => return Err(fail),
                Outcome::Success(val) => val.value,
            };

        let mailgun_api_domain =
            match SettingsCore::select_setting(models_settings::entities::settings::Model {
                name: format!("{MAILGUN_DOMAIN}"),
                ..Default::default()
            })
            .await
            {
                Outcome::Error(err) => return Err(err),
                Outcome::Failure(fail) => return Err(fail),
                Outcome::Success(val) => val.value,
            };

        Self::send_template(
            stored_user,
            &reset_token,
            &mailgun_api_key,
            &mailgun_api_domain,
        )
    }

    pub async fn select_reset_tokens(
        password_reset_model: PasswordResetModel,
    ) -> Outcome<Vec<PasswordResetModel>, CodeMessage, CodeMessage> {
        PasswordResetData::select_reset_tokens(&get_config().await.db_url, password_reset_model)
            .await
    }

    pub async fn update_user_password(
        password_reset_model: PasswordResetModel,
        mut user_model: UserModel,
    ) -> Outcome<String, CodeMessage, CodeMessage> {
        let stored_user = match UsersDaTa::select_user(
            &get_config().await.db_url,
            UserModel {
                email: user_model.email.clone(),
                ..Default::default()
            },
        )
        .await
        {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Failure(fail) => return Outcome::Failure(fail),
            Outcome::Success(val) => val,
        };

        let stored_reset_token = match Self::select_reset_tokens(password_reset_model).await {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Failure(fail) => return Outcome::Failure(fail),
            Outcome::Success(val) => val,
        };

        let stored_reset_token = match stored_reset_token.get(0) {
            None => {
                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: format!("Invalid reset token"),
                })
            }
            Some(val) => val.to_owned(),
        };

        if Some(stored_user.user_id) != stored_reset_token.user_id {
            return Outcome::Failure(CodeMessage {
                http_code: http::StatusCode::BAD_REQUEST,
                message: format!("Invalid reset token for given user id"),
            });
        }

        if let Some(password) = user_model.password {
            user_model.password = match hash(password, DEFAULT_COST) {
                Err(err) => {
                    return Outcome::Error(CodeMessage {
                        http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                        message: err.to_string(),
                    })
                }
                Ok(val) => Some(val),
            }
        };

        match PasswordResetData::update_user_password(&get_config().await.db_url, user_model).await
        {
            Outcome::Error(err) => Outcome::Error(err),
            Outcome::Failure(fail) => Outcome::Failure(fail),
            Outcome::Success(val) => Outcome::Success(format!(
                "User password for {} has been updated",
                val.email.unwrap()
            )),
        }
    }
}

impl PasswordResetCore {
    fn send_template(
        user_model: UserModel,
        reset_token: &str,
        key: &str,
        domain: &str,
    ) -> Result<String, CodeMessage> {
        let user_name = match user_model.name {
            None => String::new(),
            Some(val) => val,
        };

        let recipient_email = match user_model.email {
            None => {
                return Err(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: format!("Failed to deliver email"),
                })
            }
            Some(val) => val,
        };

        let mut template_vars = HashMap::new();
        template_vars.insert(String::from("user_name"), user_name);
        template_vars.insert(String::from("reset_token"), reset_token.to_owned());

        let recipient = EmailAddress::address(&recipient_email);
        let message = Message {
            to: vec![recipient],
            subject: String::from("Animalvet Password Reset"),
            template: String::from("password reset"),
            template_vars,
            ..Default::default()
        };

        let client = Mailgun {
            api_key: String::from(key),
            domain: String::from(domain),
            message,
        };

        let sender =
            EmailAddress::name_address("AnimalVet", &format!("postmaster@{MAILGUN_DOMAIN}"));

        match client.send(MailgunRegion::US, &sender) {
            Err(err) => Err(CodeMessage {
                http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                message: err.to_string(),
            }),
            Ok(_) => Ok(format!("Check your inbox to reset your password")),
        }
    }
}
