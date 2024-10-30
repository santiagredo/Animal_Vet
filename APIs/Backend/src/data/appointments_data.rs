use actix_web::http;
use chrono::{Local, NaiveDateTime};
use models::entities::{
    appointment_events,
    appointments::{ActiveModel, Column, Entity as Appointments, Model as AppointmentsModel},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter,
};
use security::core::SessionCore;
use tracing::error_span;
use utils::{CodeMessage, Outcome};

pub struct AppointmentsData;

impl AppointmentsData {
    pub async fn insert_appointment(
        db: &DatabaseConnection,
        appointments_model: AppointmentsModel,
        session_core: SessionCore,
    ) -> Outcome<AppointmentsModel, CodeMessage, CodeMessage> {
        let current_date = Local::now();

        let appointment = ActiveModel {
            user_id: ActiveValue::Set(appointments_model.user_id),
            pet_id: ActiveValue::Set(appointments_model.pet_id),
            date: ActiveValue::Set(appointments_model.date),
            service_id: ActiveValue::Set(appointments_model.service_id),
            creation_date: ActiveValue::set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        let inserted_appointment = match appointment.insert(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                });
            }
            Ok(val) => val,
        };

        let inserted_appointment_id = inserted_appointment.appointment_id;
        let existing_user_id = session_core.user.user_id;

        let appointment_event = appointment_events::ActiveModel {
            appointment_id: ActiveValue::Set(Some(inserted_appointment_id)),
            details: ActiveValue::Set(Some(format!(
                "Appointment id {inserted_appointment_id} inserted by existing user id {existing_user_id}"
            ))),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        if let Err(err) = appointment_event.insert(db).await {
            error_span!("error - database", error = ?err);
        };

        Outcome::Success(inserted_appointment)
    }

    pub async fn select_appointment(
        db: &DatabaseConnection,
        appointments_model: AppointmentsModel,
    ) -> Outcome<AppointmentsModel, CodeMessage, CodeMessage> {
        let mut condition = Condition::all();

        if appointments_model.appointment_id != 0 {
            condition = condition.add(Column::AppointmentId.eq(appointments_model.appointment_id));
        }

        if appointments_model.user_id.is_some_and(|x| x != 0) {
            condition = condition.add(Column::UserId.eq(appointments_model.user_id));
        }

        if appointments_model.pet_id.is_some_and(|x| x != 0) {
            condition = condition.add(Column::PetId.eq(appointments_model.pet_id));
        }

        if appointments_model.date.is_some() {
            condition = condition.add(Column::Date.eq(appointments_model.date));
        }

        if appointments_model.service_id.is_some_and(|x| x != 0) {
            condition = condition.add(Column::ServiceId.eq(appointments_model.service_id));
        }

        if appointments_model.is_canceled.is_some() {
            condition = condition.add(Column::IsCanceled.eq(appointments_model.is_canceled));
        }

        if condition.len() > 0 {
            match Appointments::find().filter(condition).one(db).await {
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
                        message: String::from("Appointment not found"),
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

    pub async fn update_appointment(
        db: &DatabaseConnection,
        appointments_model: AppointmentsModel,
        session_core: SessionCore,
    ) -> Outcome<AppointmentsModel, CodeMessage, CodeMessage> {
        let current_date = Local::now();

        let mut appointment = ActiveModel {
            appointment_id: ActiveValue::Unchanged(appointments_model.appointment_id),
            user_id: ActiveValue::Unchanged(appointments_model.user_id),
            latest_update_date: ActiveValue::set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        if appointments_model.pet_id.is_some_and(|x| x != 0) {
            appointment.pet_id = ActiveValue::set(appointments_model.pet_id);
        }

        if appointments_model.date.is_some() {
            appointment.date = ActiveValue::set(appointments_model.date);
        }

        if appointments_model.service_id.is_some_and(|x| x != 0) {
            appointment.service_id = ActiveValue::Set(appointments_model.service_id);
        }

        if appointments_model.is_canceled.is_some() {
            appointment.is_canceled = ActiveValue::Set(appointments_model.is_canceled);
        }

        if let Some(true) = appointments_model.is_canceled {
            appointment.cancellation_date = ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            )))
        }

        let updated_appointment = match appointment.update(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                if let DbErr::RecordNotFound(_) = err {
                    return Outcome::Failure(CodeMessage {
                        http_code: http::StatusCode::BAD_REQUEST,
                        message: format!("No appointment was updated"),
                    });
                };

                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                });
            }
            Ok(val) => val,
        };

        let updated_appointment_id = updated_appointment.appointment_id;
        let existing_user_id = session_core.user.user_id;

        let appointment_event = appointment_events::ActiveModel {
            appointment_id: ActiveValue::Set(Some(updated_appointment_id)),
            details: ActiveValue::Set(Some(format!(
                "Appointment id {updated_appointment_id} updated by existing user id {existing_user_id}"
            ))),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        if let Err(err) = appointment_event.insert(db).await {
            error_span!("error - database", error = ?err);
        };

        Outcome::Success(updated_appointment)
    }

    // pub async fn delete_appointment(
    //     db: &DatabaseConnection,
    //     appointments_model: AppointmentsModel,
    //     session_core: SessionCore,
    // ) -> Outcome<u64, CodeMessage, CodeMessage> {
    //     let rows_affected = match Appointments::delete_by_id(appointments_model.appointment_id)
    //         .exec(db)
    //         .await
    //     {
    //         Err(err) => {
    //             error_span!("error - database", error = ?err);

    //             if let DbErr::Exec(_) = err {
    //                 return Outcome::Failure(CodeMessage {
    //                     http_code: http::StatusCode::BAD_REQUEST,
    //                     message: err.to_string(),
    //                 });
    //             }

    //             return Outcome::Error(CodeMessage {
    //                 http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
    //                 message: err.to_string(),
    //             });
    //         }
    //         Ok(val) => val.rows_affected,
    //     };

    //     let creation_date = Local::now();
    //     let deleted_appointment_id = appointments_model.appointment_id;
    //     let existing_user_id = session_core.user.user_id;

    //     let appointment_event = appointment_events::ActiveModel {
    //         appointment_id: ActiveValue::Set(Some(appointments_model.appointment_id)),
    //         details: ActiveValue::Set(Some(format!(
    //             "Appointment id {deleted_appointment_id} deleted by existing user id {existing_user_id}"
    //         ))),
    //         creation_date: ActiveValue::Set(Some(creation_date.into())),
    //         ..Default::default()
    //     };

    //     if let Err(err) = appointment_event.insert(db).await {
    //         error_span!("error - database", error = ?err);
    //     };

    //     Outcome::Success(rows_affected)
    // }
}
