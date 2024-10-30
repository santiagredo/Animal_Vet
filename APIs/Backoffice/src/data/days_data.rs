use actix_web::http;
use models::entities::days::{Column, Entity as Days, Model as DaysModel};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::error_span;
use utils::{CodeMessage, Outcome};

pub struct DaysData;

impl DaysData {
    pub async fn select_day(
        db: &DatabaseConnection,
        days_model: DaysModel,
        find_all: bool,
    ) -> Outcome<Vec<DaysModel>, CodeMessage, CodeMessage> {
        let mut condition = Condition::all();

        if days_model.day_id != 0 {
            condition = condition.add(Column::DayId.eq(days_model.day_id));
        }

        if days_model
            .name
            .as_ref()
            .is_some_and(|x| !x.trim().is_empty())
        {
            condition = condition.add(Column::Name.eq(days_model.name));
        }

        if find_all {
            condition = condition.add(Column::Name.is_not_null());
        }

        if condition.len() > 0 {
            match Days::find().filter(condition).all(db).await {
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
                            message: String::from("Day not found"),
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
