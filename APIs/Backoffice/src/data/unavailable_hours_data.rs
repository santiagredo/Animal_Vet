use actix_web::http;
use chrono::{Local, NaiveDateTime};
use models::entities::{
    unavailable_hour_events,
    unavailable_hours::{
        ActiveModel, Column, Entity as UnavailableHours, Model as UnavailableHoursModel,
    },
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter,
};
use security::core::SessionCore;
use tracing::error_span;
use utils::{CodeMessage, Outcome};

pub struct UnavailableHoursData;

impl UnavailableHoursData {
    pub async fn insert_unavailable_hours(
        db: &DatabaseConnection,
        unavailable_hours_model: UnavailableHoursModel,
        session_core: SessionCore,
    ) -> Outcome<UnavailableHoursModel, CodeMessage, CodeMessage> {
        let current_date = Local::now();

        let unavailable_hours = ActiveModel {
            service_id: ActiveValue::Set(unavailable_hours_model.service_id),
            date: ActiveValue::Set(unavailable_hours_model.date),
            start_time: ActiveValue::Set(unavailable_hours_model.start_time),
            end_time: ActiveValue::Set(unavailable_hours_model.end_time),
            reason: ActiveValue::Set(unavailable_hours_model.reason),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            latest_update_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        let inserted_unavailable_hours = match unavailable_hours.insert(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                });
            }
            Ok(val) => val,
        };

        let inserted_unavailable_hours_id = inserted_unavailable_hours.unavailable_hour_id;
        let existing_user_id = session_core.user.user_id;

        let unavailable_hours_event = unavailable_hour_events::ActiveModel {
            unavailable_hour_id: ActiveValue::Set(Some(inserted_unavailable_hours_id)),
            details: ActiveValue::Set(Some(format!(
                "Unavailable hours id {inserted_unavailable_hours_id} inserted by existing user id {existing_user_id}"
            ))),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        if let Err(err) = unavailable_hours_event.insert(db).await {
            error_span!("error - database", error = ?err);
        };

        Outcome::Success(inserted_unavailable_hours)
    }

    pub async fn select_unavailable_hours(
        db: &DatabaseConnection,
        unavailable_hours_model: UnavailableHoursModel,
        opt_unavailable_hours_model: Option<UnavailableHoursModel>,
    ) -> Outcome<Vec<UnavailableHoursModel>, CodeMessage, CodeMessage> {
        let mut condition = Condition::all();

        if unavailable_hours_model.unavailable_hour_id != 0 {
            condition = condition
                .add(Column::UnavailableHourId.eq(unavailable_hours_model.unavailable_hour_id));
        }

        if unavailable_hours_model.service_id.is_some() {
            condition = condition.add(Column::ServiceId.eq(unavailable_hours_model.service_id));
        }

        if unavailable_hours_model.date.is_some() && opt_unavailable_hours_model.is_none() {
            condition = condition.add(Column::Date.eq(unavailable_hours_model.date));
        }

        if unavailable_hours_model.date.is_some()
            && opt_unavailable_hours_model
                .as_ref()
                .is_some_and(|x| x.date.is_some())
        {
            condition = condition.add(Column::Date.gte(unavailable_hours_model.date));
            condition = condition.add(Column::Date.lte(opt_unavailable_hours_model.unwrap().date));
        }

        if condition.len() > 0 {
            match UnavailableHours::find().filter(condition).all(db).await {
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
                            message: String::from("Unavailable Hours not found"),
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

    pub async fn update_unavailble_hours(
        db: &DatabaseConnection,
        unavailable_hours_model: UnavailableHoursModel,
        session_core: SessionCore,
    ) -> Outcome<UnavailableHoursModel, CodeMessage, CodeMessage> {
        let current_date = Local::now();

        let mut unavailable_hours = ActiveModel {
            unavailable_hour_id: ActiveValue::Unchanged(
                unavailable_hours_model.unavailable_hour_id,
            ),
            latest_update_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        if unavailable_hours_model.service_id.is_some_and(|x| x != 0) {
            unavailable_hours.service_id = ActiveValue::Set(unavailable_hours_model.service_id);
        }

        if unavailable_hours_model.date.is_some() {
            unavailable_hours.date = ActiveValue::Set(unavailable_hours_model.date);
        }

        if unavailable_hours_model.start_time.is_some() {
            unavailable_hours.start_time = ActiveValue::Set(unavailable_hours_model.start_time);
        }

        if unavailable_hours_model.end_time.is_some() {
            unavailable_hours.end_time = ActiveValue::Set(unavailable_hours_model.end_time);
        }

        if unavailable_hours_model
            .reason
            .as_ref()
            .is_some_and(|x| !x.trim().is_empty())
        {
            unavailable_hours.reason = ActiveValue::Set(unavailable_hours_model.reason);
        }

        let updated_unavailable_hours = match unavailable_hours.update(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                if let DbErr::RecordNotFound(_) = err {
                    return Outcome::Failure(CodeMessage {
                        http_code: http::StatusCode::BAD_REQUEST,
                        message: format!("No unavailable hours were updated"),
                    });
                };

                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                });
            }
            Ok(val) => val,
        };

        let updated_unavailable_hours_id = updated_unavailable_hours.unavailable_hour_id;
        let existing_user_id = session_core.user.user_id;

        let unavailable_hours_event = unavailable_hour_events::ActiveModel {
            unavailable_hour_id: ActiveValue::Set(Some(updated_unavailable_hours_id)),
            details: ActiveValue::Set(Some(format!(
                "Unavailable hours id {updated_unavailable_hours_id} updated by existing user id {existing_user_id}"
            ))),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        if let Err(err) = unavailable_hours_event.insert(db).await {
            error_span!("error - database", error = ?err);
        };

        Outcome::Success(updated_unavailable_hours)
    }

    pub async fn delete_unavailable_hours(
        db: &DatabaseConnection,
        unavailable_hours_model: UnavailableHoursModel,
        session_core: SessionCore,
    ) -> Outcome<u64, CodeMessage, CodeMessage> {
        let rows_affected =
            match UnavailableHours::delete_by_id(unavailable_hours_model.unavailable_hour_id)
                .exec(db)
                .await
            {
                Err(err) => {
                    error_span!("error - database", error = ?err);

                    if let DbErr::Exec(_) = err {
                        return Outcome::Failure(CodeMessage {
                            http_code: http::StatusCode::BAD_REQUEST,
                            message: err.to_string(),
                        });
                    }

                    return Outcome::Error(CodeMessage {
                        http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                        message: err.to_string(),
                    });
                }
                Ok(val) => val.rows_affected,
            };

        let current_date = Local::now();
        let deleted_unavailable_hours_id = unavailable_hours_model.unavailable_hour_id;
        let existing_user_id = session_core.user.user_id;

        let unavailable_hours_event = unavailable_hour_events::ActiveModel {
            unavailable_hour_id: ActiveValue::Set(Some(deleted_unavailable_hours_id)),
            details: ActiveValue::Set(Some(format!(
                "Unavailable hours id {deleted_unavailable_hours_id} deleted by existing user id {existing_user_id}"
            ))),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        if let Err(err) = unavailable_hours_event.insert(db).await {
            error_span!("error - database", error = ?err);
        };

        Outcome::Success(rows_affected)
    }
}
