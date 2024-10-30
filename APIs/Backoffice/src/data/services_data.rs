use actix_web::http;
use chrono::{Local, NaiveDateTime};
use models::entities::{
    service_events,
    services::{self, Column, Entity as Services, Model as ServiceModel},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter,
};
use security::core::SessionCore;
use tracing::error_span;
use utils::{CodeMessage, Outcome};

pub struct ServicesData;

impl ServicesData {
    pub async fn insert_service(
        db: &DatabaseConnection,
        service_model: ServiceModel,
        session_core: SessionCore,
    ) -> Outcome<ServiceModel, CodeMessage, CodeMessage> {
        let current_date = Local::now();

        let service = services::ActiveModel {
            name: ActiveValue::Set(service_model.name),
            duration: ActiveValue::Set(service_model.duration),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            is_enabled: ActiveValue::Set(service_model.is_enabled),
            latest_update_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        let inserted_service = match service.insert(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                });
            }
            Ok(val) => val,
        };

        let inserted_service_id = inserted_service.service_id;
        let existing_user_id = session_core.user.user_id;

        let service_event = service_events::ActiveModel {
            service_id: ActiveValue::Set(Some(inserted_service_id)),
            details: ActiveValue::Set(Some(format!(
                "Service id {inserted_service_id} inserted by existing user id {existing_user_id}"
            ))),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        if let Err(err) = service_event.insert(db).await {
            error_span!("error - database", error = ?err);
        };

        Outcome::Success(inserted_service)
    }

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

    pub async fn update_service(
        db: &DatabaseConnection,
        service_model: ServiceModel,
        session_core: SessionCore,
    ) -> Outcome<ServiceModel, CodeMessage, CodeMessage> {
        let current_date = Local::now();

        let mut service = services::ActiveModel {
            service_id: ActiveValue::Unchanged(service_model.service_id),
            latest_update_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        if service_model.name.as_ref().is_some_and(|x| !x.is_empty()) {
            service.name = ActiveValue::Set(service_model.name);
        }

        if service_model.duration.is_some_and(|x| x != 0) {
            service.duration = ActiveValue::Set(service_model.duration);
        }

        if service_model.is_enabled.is_some() {
            service.is_enabled = ActiveValue::Set(service_model.is_enabled);
        }

        let updated_service = match service.update(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                if let DbErr::RecordNotFound(_) = err {
                    return Outcome::Failure(CodeMessage {
                        http_code: http::StatusCode::BAD_REQUEST,
                        message: format!("No service was updated"),
                    });
                };

                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                });
            }
            Ok(val) => val,
        };

        let updated_service_id = updated_service.service_id;
        let existing_user_id = session_core.user.user_id;

        let service_event = service_events::ActiveModel {
            service_id: ActiveValue::Set(Some(updated_service_id)),
            details: ActiveValue::Set(Some(format!(
                "Service id {updated_service_id} updated by existing user id {existing_user_id}"
            ))),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        if let Err(err) = service_event.insert(db).await {
            error_span!("error - database", error = ?err);
        };

        Outcome::Success(updated_service)
    }

    pub async fn delete_service(
        db: &DatabaseConnection,
        service_model: ServiceModel,
        session_core: SessionCore,
    ) -> Outcome<u64, CodeMessage, CodeMessage> {
        let rows_affected = match Services::delete_by_id(service_model.service_id)
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
        let deleted_service_id = service_model.service_id;
        let existing_user_id = session_core.user.user_id;

        let service_event = service_events::ActiveModel {
            service_id: ActiveValue::Set(Some(deleted_service_id)),
            details: ActiveValue::Set(Some(format!(
                "Service id {deleted_service_id} deleted by existing user id {existing_user_id}"
            ))),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        if let Err(err) = service_event.insert(db).await {
            error_span!("error - database", error = ?err);
        };

        Outcome::Success(rows_affected)
    }
}
