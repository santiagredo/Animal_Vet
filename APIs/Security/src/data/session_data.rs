use actix_web::http;
use chrono::{Local, NaiveDateTime};
use models::entities::{
    sessions::{self, Column, Entity as Sessions, Model as SessionModel},
    users::Model as UserModel,
};
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection,
    EntityTrait, QueryFilter,
};
use tracing::error_span;
use utils::{CodeMessage, Outcome};
use uuid::Uuid;

pub struct SessionData;

impl SessionData {
    pub async fn insert_session(
        db: &DatabaseConnection,
        user_model: UserModel,
    ) -> Result<SessionModel, CodeMessage> {
        let current_date = Local::now();

        let session = sessions::ActiveModel {
            user_id: ActiveValue::Set(Some(user_model.user_id)),
            session_uuid: ActiveValue::Set(Some(Uuid::new_v4())),
            is_enabled: ActiveValue::Set(Some(true)),
            creation_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            ..Default::default()
        };

        match session.insert(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                Err(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                })
            }
            Ok(val) => Ok(val),
        }
    }

    pub async fn select_session(
        db: &DatabaseConnection,
        session_model: SessionModel,
    ) -> Outcome<SessionModel, CodeMessage, CodeMessage> {
        let mut condition = Condition::all();

        if session_model.session_id != 0 {
            condition = condition.add(Column::SessionId.eq(session_model.session_id));
        }

        if session_model.user_id.is_some_and(|x| x != 0) {
            condition = condition.add(Column::UserId.eq(session_model.user_id));
        }

        if session_model
            .session_uuid
            .as_ref()
            .is_some_and(|x| !x.is_nil())
        {
            condition = condition.add(Column::SessionUuid.eq(session_model.session_uuid));
        }

        if session_model.is_enabled.is_some() {
            condition = condition.add(Column::IsEnabled.eq(session_model.is_enabled.unwrap()));
        }

        match Sessions::find().filter(condition).one(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                })
            }
            Ok(None) => Outcome::Failure(CodeMessage {
                http_code: http::StatusCode::NOT_FOUND,
                message: String::from("Session not found"),
            }),
            Ok(Some(val)) => Outcome::Success(val),
        }
    }

    pub async fn update_session(
        db: &DatabaseConnection,
        session_model: SessionModel,
    ) -> Result<SessionModel, CodeMessage> {
        let current_date = Local::now();

        let session = sessions::ActiveModel {
            session_id: ActiveValue::Unchanged(session_model.session_id),
            user_id: ActiveValue::Unchanged(session_model.user_id),
            session_uuid: ActiveValue::Unchanged(Some(Uuid::new_v4())),
            is_enabled: ActiveValue::Set(session_model.is_enabled),
            latest_update_date: ActiveValue::Set(Some(NaiveDateTime::new(
                current_date.date_naive(),
                current_date.time(),
            ))),
            closing_date: ActiveValue::Set(session_model.closing_date),
            ..Default::default()
        };

        match session.update(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                Err(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                })
            }
            Ok(val) => Ok(val),
        }
    }

    pub async fn update_multiple_sessions(
        db: &DatabaseConnection,
        session_updates: SessionModel,
        session_conditions: SessionModel,
    ) -> Result<u64, CodeMessage> {
        let mut query = Sessions::update_many();

        // conditions to update multiple rows
        let mut conditions = Condition::all();

        if session_conditions.session_id != 0 {
            conditions = conditions.add(Column::SessionId.eq(session_conditions.session_id));
        }

        if session_conditions.user_id.is_some_and(|x| x != 0) {
            conditions = conditions.add(Column::UserId.eq(session_conditions.user_id.unwrap()));
        }

        if session_conditions.session_uuid.is_some_and(|x| !x.is_nil()) {
            conditions =
                conditions.add(Column::SessionUuid.eq(session_conditions.session_uuid.unwrap()));
        }

        // return failure if no conditions
        if conditions.len() < 1 {
            return Err(CodeMessage {
                http_code: http::StatusCode::BAD_REQUEST,
                message: String::from("No condition parameters"),
            });
        }

        query = query.filter(conditions);

        // new values to insert in filtered rows
        if session_updates.is_enabled.is_some() {
            query = query.col_expr(
                Column::IsEnabled,
                Expr::value(session_updates.is_enabled.unwrap()),
            )
        }

        if session_updates.latest_update_date.is_some() {
            query = query.col_expr(
                Column::LatestUpdateDate,
                Expr::value(session_updates.latest_update_date.unwrap()),
            )
        }

        if session_updates.closing_date.is_some() {
            query = query.col_expr(
                Column::ClosingDate,
                Expr::value(session_updates.closing_date.unwrap()),
            )
        }

        match query.exec(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                Err(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                })
            }
            Ok(val) => Ok(val.rows_affected),
        }
    }
}
