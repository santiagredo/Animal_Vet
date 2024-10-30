use actix_web::http;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::error_span;
use utils::{CodeMessage, Outcome};

use models::entities::users::{self, Column, Entity as Users};

type UserEntity = <users::Entity as sea_orm::EntityTrait>::Model;

pub struct UsersDaTa;

impl UsersDaTa {
    pub async fn select_user(
        db: &DatabaseConnection,
        user_entity: UserEntity,
    ) -> Outcome<UserEntity, CodeMessage, CodeMessage> {

        let mut condition = Condition::all();

        if user_entity.user_id != 0 {
            condition = condition.add(Column::UserId.eq(user_entity.user_id));
        }

        if user_entity.name.as_ref().is_some_and(|x| !x.is_empty()) {
            condition = condition.add(Column::Name.eq(user_entity.name));
        }

        if user_entity.email.as_ref().is_some_and(|x| !x.is_empty()) {
            condition = condition.add(Column::Email.eq(user_entity.email))
        }

        if user_entity.password.as_ref().is_some_and(|x| !x.is_empty()) {
            condition = condition.add(Column::Password.eq(user_entity.password));
        }

        if user_entity
            .phone_number
            .as_ref()
            .is_some_and(|x| !x.is_empty())
        {
            condition = condition.add(Column::PhoneNumber.eq(user_entity.phone_number));
        }

        if user_entity
            .document_id
            .as_ref()
            .is_some_and(|x| !x.is_empty())
        {
            condition = condition.add(Column::DocumentId.eq(user_entity.document_id));
        }

        if user_entity.role.is_some() {
            condition = condition.add(Column::Role.eq(user_entity.role));
        }

        if user_entity.is_enabled.is_some() {
            condition = condition.add(Column::IsEnabled.eq(user_entity.is_enabled.unwrap()));
        }

        match Users::find().filter(condition).one(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                })
            }
            Ok(None) => Outcome::Failure(CodeMessage {
                http_code: http::StatusCode::NOT_FOUND,
                message: String::from("User not found"),
            }),
            Ok(Some(val)) => Outcome::Success(val),
        }
    }
}
