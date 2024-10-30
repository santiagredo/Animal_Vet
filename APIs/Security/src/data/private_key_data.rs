use actix_web::http;
use models_settings::entities::privkey::{Column, Entity as PrivateKey, Model};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::error_span;
use utils::{CodeMessage, Outcome};

pub struct PrivateKeyData;

impl PrivateKeyData {
    pub async fn select_private_key(
        db: &DatabaseConnection,
    ) -> Outcome<Model, CodeMessage, CodeMessage> {
        let mut condition = Condition::all();

        condition = condition.add(Column::IsEnabled.eq(true));

        if condition.len() > 0 {
            match PrivateKey::find().filter(condition).one(db).await {
                Err(err) => {
                    error_span!("error - database", error = ?err);

                    return Outcome::Error(CodeMessage {
                        http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                        message: err.to_string(),
                    });
                }
                Ok(None) => {
                    error_span!("error - database", error = format!("Private key not found"));

                    return Outcome::Error(CodeMessage {
                        http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                        message: format!("Private key not found"),
                    });
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
