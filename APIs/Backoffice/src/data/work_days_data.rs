use actix_web::http;
use chrono::{Local, NaiveDateTime};
use models::entities::{
    work_day_events,
    work_days::{self, Column, Entity as WorkDay, Model as WorkDayModel},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter,
};
use security::core::SessionCore;
use tracing::error_span;
use utils::{CodeMessage, Outcome};

pub struct WorkDaysData;

impl WorkDaysData {
    pub async fn insert_work_day(
        db: &DatabaseConnection,
        work_day_model: WorkDayModel,
        session_core: SessionCore,
    ) -> Outcome<WorkDayModel, CodeMessage, CodeMessage> {
        let current_date = Local::now();

        let work_day = work_days::ActiveModel {
            service_id: ActiveValue::Set(work_day_model.service_id),
            day_id: ActiveValue::Set(work_day_model.day_id),
            is_enabled: ActiveValue::Set(work_day_model.is_enabled),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(current_date.date_naive(), current_date.time()))),
            latest_update_date: ActiveValue::Set(Some(NaiveDateTime::new(current_date.date_naive(), current_date.time()))),
            open_time: ActiveValue::set(work_day_model.open_time),
            close_time: ActiveValue::Set(work_day_model.close_time),
            lunch_from_time: ActiveValue::Set(work_day_model.lunch_from_time),
            lunch_to_time: ActiveValue::Set(work_day_model.lunch_to_time),
            ..Default::default()
        };

        let inserted_work_day = match work_day.insert(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                });
            }
            Ok(val) => val,
        };

        let inserted_work_day_id = inserted_work_day.work_day_id;
        let existing_user_id = session_core.user.user_id;

        let work_day_event = work_day_events::ActiveModel {
            work_day_id: ActiveValue::Set(Some(inserted_work_day_id)),
            details: ActiveValue::Set(Some(format!(
                "Work day id {inserted_work_day_id} inserted by existing user id {existing_user_id}"
            ))),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(current_date.date_naive(), current_date.time()))),
            ..Default::default()
        };

        if let Err(err) = work_day_event.insert(db).await {
            error_span!("error - database", error = ?err);
        };

        Outcome::Success(inserted_work_day)
    }

    pub async fn select_work_days(
        db: &DatabaseConnection,
        work_day_model: WorkDayModel,
    ) -> Outcome<Vec<WorkDayModel>, CodeMessage, CodeMessage> {
        let mut condtion = Condition::all();

        if work_day_model.work_day_id != 0 {
            condtion = condtion.add(Column::WorkDayId.eq(work_day_model.work_day_id));
        }

        if work_day_model.service_id.is_some() {
            condtion = condtion.add(Column::ServiceId.eq(work_day_model.service_id));
        }

        if work_day_model.day_id.is_some() {
            condtion = condtion.add(Column::DayId.eq(work_day_model.day_id));
        }

        if work_day_model.is_enabled.is_some() {
            condtion = condtion.add(Column::IsEnabled.eq(work_day_model.is_enabled));
        }

        if condtion.len() > 0 {
            match WorkDay::find().filter(condtion).all(db).await {
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
                            message: format!("Work days not found"),
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

    pub async fn update_work_day(
        db: &DatabaseConnection,
        work_day_model: WorkDayModel,
        session_core: SessionCore,
    ) -> Outcome<WorkDayModel, CodeMessage, CodeMessage> {
        let current_date = Local::now();

        let mut work_day = work_days::ActiveModel {
            work_day_id: ActiveValue::Unchanged(work_day_model.work_day_id),
            latest_update_date: ActiveValue::Set(Some(NaiveDateTime::new(current_date.date_naive(), current_date.time()))),
            ..Default::default()
        };

        if work_day_model.service_id.is_some_and(|x| x != 0) {
            work_day.service_id = ActiveValue::set(work_day_model.service_id);
        }

        if work_day_model.day_id.is_some_and(|x| x != 0) {
            work_day.day_id = ActiveValue::set(work_day_model.day_id);
        }

        if work_day_model.is_enabled.is_some() {
            work_day.is_enabled = ActiveValue::Set(work_day_model.is_enabled);
        }

        if work_day_model.is_enabled.is_some() {
            work_day.is_enabled = ActiveValue::Set(work_day_model.is_enabled);
        }

        if work_day_model.open_time.is_some() {
            work_day.open_time = ActiveValue::Set(work_day_model.open_time);
        }

        if work_day_model.close_time.is_some() {
            work_day.close_time = ActiveValue::Set(work_day_model.close_time);
        }

        if work_day_model.lunch_from_time.is_some() {
            work_day.lunch_from_time = ActiveValue::Set(work_day_model.lunch_from_time);
        }

        if work_day_model.lunch_to_time.is_some() {
            work_day.lunch_to_time = ActiveValue::Set(work_day_model.lunch_to_time);
        }

        let updated_work_day = match work_day.update(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                if let DbErr::RecordNotFound(_) = err {
                    return Outcome::Failure(CodeMessage {
                        http_code: http::StatusCode::BAD_REQUEST,
                        message: format!("No work day was updated"),
                    });
                };

                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                });
            }
            Ok(val) => val,
        };

        let updated_work_day_id = updated_work_day.work_day_id;
        let existing_user_id = session_core.user.user_id;

        let work_day_event = work_day_events::ActiveModel {
            work_day_id: ActiveValue::Set(Some(updated_work_day_id)),
            details: ActiveValue::Set(Some(format!(
                "Work day id {updated_work_day_id} updated by existing user id {existing_user_id}"
            ))),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(current_date.date_naive(), current_date.time()))),
            ..Default::default()
        };

        if let Err(err) = work_day_event.insert(db).await {
            error_span!("error - database", error = ?err);
        };

        Outcome::Success(updated_work_day)
    }

    pub async fn delete_work_day(
        db: &DatabaseConnection,
        work_day_model: WorkDayModel,
        session_core: SessionCore,
    ) -> Outcome<u64, CodeMessage, CodeMessage> {
        let rows_affected = match WorkDay::delete_by_id(work_day_model.work_day_id)
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
        let deleted_work_day_id = work_day_model.work_day_id;
        let existing_user_id = session_core.user.user_id;

        let work_day_event = work_day_events::ActiveModel {
            work_day_id: ActiveValue::Set(Some(deleted_work_day_id)),
            details: ActiveValue::Set(Some(format!(
                "Work day id {deleted_work_day_id} deleted by existing user id {existing_user_id}"
            ))),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(current_date.date_naive(), current_date.time()))),
            ..Default::default()
        };

        if let Err(err) = work_day_event.insert(db).await {
            error_span!("error - database", error = ?err);
        };

        Outcome::Success(rows_affected)
    }
}
