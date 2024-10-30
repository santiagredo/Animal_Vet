use actix_web::http;
use models::entities::services::{Column, Entity as Services, Model as ServiceModel};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::error_span;
use utils::{CodeMessage, Outcome};

pub struct ServicesData;

impl ServicesData {
    pub async fn select_services(
        db: &DatabaseConnection,
        service_model: ServiceModel,
    ) -> Outcome<Vec<ServiceModel>, CodeMessage, CodeMessage> {
        let mut condtion = Condition::all();

        if service_model.service_id != 0 {
            condtion = condtion.add(Column::ServiceId.eq(service_model.service_id));
        }

        if service_model.name.as_ref().is_some_and(|x| !x.is_empty()) {
            condtion = condtion.add(Column::Name.eq(service_model.name));
        }

        if service_model.duration.is_some_and(|x| x != 0) {
            condtion = condtion.add(Column::Duration.eq(service_model.duration));
        }

        if service_model.is_enabled.is_some() {
            condtion = condtion.add(Column::IsEnabled.eq(service_model.is_enabled));
        }

        if condtion.len() > 0 {
            match Services::find().filter(condtion).all(db).await {
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
                            message: format!("Services not found"),
                        });
                    }

                    return Outcome::Success(val);
                }
            }
        }

        Outcome::Failure(CodeMessage {
            http_code: http::StatusCode::BAD_REQUEST,
            message: String::from("No searching parameters"),
        })
    }
}
