use actix_web::http;
use chrono::{Local, NaiveDateTime};
use models::entities::{
    special_date_events,
    special_dates::{self, Column, Entity as SpecialDate, Model as SpecialDateModel},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter,
};
use security::core::SessionCore;
use tracing::error_span;
use utils::{CodeMessage, Outcome};

pub struct SpecialDatesData;

impl SpecialDatesData {
    pub async fn insert_special_date(
        db: &DatabaseConnection,
        special_date_model: SpecialDateModel,
        session_core: SessionCore,
    ) -> Outcome<SpecialDateModel, CodeMessage, CodeMessage> {
        let current_date = Local::now();

        let special_date = special_dates::ActiveModel {
            service_id: ActiveValue::Set(special_date_model.service_id),
            date: ActiveValue::Set(special_date_model.date),
            is_working_date: ActiveValue::Set(special_date_model.is_working_date),
            open_time: ActiveValue::Set(special_date_model.open_time),
            close_time: ActiveValue::Set(special_date_model.close_time),
            lunch_from_time: ActiveValue::Set(special_date_model.lunch_from_time),
            lunch_to_time: ActiveValue::Set(special_date_model.lunch_to_time),
            creation_date: ActiveValue::set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            latest_update_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            reason: ActiveValue::Set(special_date_model.reason),
            ..Default::default()
        };

        let inserted_special_date = match special_date.insert(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                });
            }
            Ok(val) => val,
        };

        let inserted_special_date_id = inserted_special_date.special_date_id;
        let existing_user_id = session_core.user.user_id;

        let special_date_event = special_date_events::ActiveModel {
            special_date_id: ActiveValue::Set(Some(inserted_special_date_id)),
            details: ActiveValue::Set(Some(format!(
                "Special date id {inserted_special_date_id} inserted by existing user id {existing_user_id}"
            ))),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        if let Err(err) = special_date_event.insert(db).await {
            error_span!("error - database", error = ?err);
        };

        Outcome::Success(inserted_special_date)
    }

    pub async fn select_special_dates(
        db: &DatabaseConnection,
        special_date_model: SpecialDateModel,
        opt_special_date_model: Option<SpecialDateModel>,
    ) -> Outcome<Vec<SpecialDateModel>, CodeMessage, CodeMessage> {
        let mut condtion = Condition::all();

        if special_date_model.special_date_id != 0 {
            condtion = condtion.add(Column::SpecialDateId.eq(special_date_model.special_date_id));
        }

        if special_date_model.service_id.is_some() {
            condtion = condtion.add(Column::ServiceId.eq(special_date_model.service_id));
        }

        if special_date_model.date.is_some() && opt_special_date_model.is_none() {
            condtion = condtion.add(Column::Date.eq(special_date_model.date));
        }

        if special_date_model.date.is_some()
            && opt_special_date_model
                .as_ref()
                .is_some_and(|x| x.date.is_some())
        {
            condtion = condtion.add(Column::Date.gte(special_date_model.date));
            condtion = condtion.add(Column::Date.lte(opt_special_date_model.unwrap().date));
        }

        if condtion.len() > 0 {
            match SpecialDate::find().filter(condtion).all(db).await {
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
                            message: String::from("Special Dates not found"),
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

    pub async fn update_special_date(
        db: &DatabaseConnection,
        special_date_model: SpecialDateModel,
        session_core: SessionCore,
    ) -> Outcome<SpecialDateModel, CodeMessage, CodeMessage> {
        let current_date = Local::now();

        let mut special_date = special_dates::ActiveModel {
            special_date_id: ActiveValue::Unchanged(special_date_model.special_date_id),
            latest_update_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        if special_date_model.service_id.is_some_and(|x| x != 0) {
            special_date.service_id = ActiveValue::Set(special_date_model.service_id);
        }

        if special_date_model.date.is_some() {
            special_date.date = ActiveValue::Set(special_date_model.date);
        }

        if special_date_model.is_working_date.is_some() {
            special_date.is_working_date = ActiveValue::Set(special_date_model.is_working_date);
        }

        if special_date_model.open_time.is_some() {
            special_date.open_time = ActiveValue::Set(special_date_model.open_time);
        }

        if special_date_model.close_time.is_some() {
            special_date.close_time = ActiveValue::Set(special_date_model.close_time);
        }

        if special_date_model.lunch_from_time.is_some() {
            special_date.lunch_from_time = ActiveValue::Set(special_date_model.lunch_from_time);
        }

        if special_date_model.lunch_to_time.is_some() {
            special_date.lunch_to_time = ActiveValue::Set(special_date_model.lunch_to_time);
        }

        if special_date_model
            .reason
            .as_ref()
            .is_some_and(|x| !x.trim().is_empty())
        {
            special_date.reason = ActiveValue::Set(special_date_model.reason);
        }

        let updated_special_date = match special_date.update(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                if let DbErr::RecordNotFound(_) = err {
                    return Outcome::Failure(CodeMessage {
                        http_code: http::StatusCode::BAD_REQUEST,
                        message: format!("No special date was updated"),
                    });
                };

                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                });
            }
            Ok(val) => val,
        };

        let updated_special_date_id = updated_special_date.special_date_id;
        let existing_user_id = session_core.user.user_id;

        let special_date_event = special_date_events::ActiveModel {
            special_date_id: ActiveValue::Set(Some(updated_special_date_id)),
            details: ActiveValue::Set(Some(format!(
                "Special date id {updated_special_date_id} updated by existing user id {existing_user_id}"
            ))),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        if let Err(err) = special_date_event.insert(db).await {
            error_span!("error - database", error = ?err);
        };

        Outcome::Success(updated_special_date)
    }

    pub async fn delete_special_date(
        db: &DatabaseConnection,
        special_date_model: SpecialDateModel,
        session_core: SessionCore,
    ) -> Outcome<u64, CodeMessage, CodeMessage> {
        let rows_affected = match SpecialDate::delete_by_id(special_date_model.special_date_id)
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
        let deleted_special_date_id = special_date_model.special_date_id;
        let existing_user_id = session_core.user.user_id;

        let special_date_event = special_date_events::ActiveModel {
            special_date_id: ActiveValue::Set(Some(deleted_special_date_id)),
            details: ActiveValue::Set(Some(format!(
                "Special date id {deleted_special_date_id} deleted by existing user id {existing_user_id}"
            ))),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        if let Err(err) = special_date_event.insert(db).await {
            error_span!("error - database", error = ?err);
        };

        Outcome::Success(rows_affected)
    }
}
