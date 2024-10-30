use actix_web::http;
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};
use tracing::error_span;
use utils::{get_config, CodeMessage, Outcome};

use models::entities::{
    prelude::UserRoles,
    user_roles::{Column, Model},
};
pub struct UserRolesData {}

impl UserRolesData {
    pub async fn select_role(user_role: Model) -> Outcome<Model, CodeMessage, CodeMessage> {
        let db = &get_config().await.db_url;

        let mut condition = Condition::all();

        if user_role.user_role_id != 0 {
            condition = condition.add(Column::UserRoleId.eq(user_role.user_role_id));
        }

        if user_role.name.as_ref().is_some_and(|x| !x.is_empty()) {
            condition = condition.add(Column::Name.eq(user_role.name));
        }

        if user_role.is_enabled.is_some() {
            condition = condition.add(Column::IsEnabled.eq(user_role.is_enabled.unwrap_or(false)));
        }

        if condition.len() > 0 {
            match UserRoles::find().filter(condition).one(db).await {
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
                        message: String::from("User role not found"),
                    })
                }
                Ok(Some(val)) => return Outcome::Success(val),
            }
        }

        Outcome::Failure(CodeMessage {
            http_code: http::StatusCode::BAD_REQUEST,
            message: String::from("No searching parameters"),
        })
    }
}
