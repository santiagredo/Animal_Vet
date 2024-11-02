use actix_web::http;
use models_settings::entities::settings::{
    self, Column, Entity as Settings, Model as SettingsModel,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter,
};
use tracing::error_span;
use utils::{CodeMessage, Outcome};

pub struct SettingsData;

impl SettingsData {
    pub async fn insert_setting(
        db: &DatabaseConnection,
        settings_model: SettingsModel,
    ) -> Outcome<SettingsModel, CodeMessage, CodeMessage> {
        let setting = settings::ActiveModel {
            name: sea_orm::ActiveValue::Set(settings_model.name),
            value: ActiveValue::Set(settings_model.value),
            ..Default::default()
        };

        match setting.insert(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                });
            }
            Ok(val) => Outcome::Success(val),
        }
    }

    pub async fn select_setting(
        db: &DatabaseConnection,
        settings_model: SettingsModel,
    ) -> Outcome<SettingsModel, CodeMessage, CodeMessage> {
        let mut condition = Condition::all();

        if settings_model.id != 0 {
            condition = condition.add(Column::Id.eq(settings_model.id))
        }

        if !settings_model.name.is_empty() {
            condition = condition.add(Column::Name.eq(settings_model.name))
        }

        if !settings_model.value.is_empty() {
            condition = condition.add(Column::Value.eq(settings_model.value))
        }

        if condition.len() > 0 {
            match Settings::find().filter(condition).one(db).await {
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
                        message: String::from("Setting not found"),
                    })
                }
                Ok(Some(val)) => return Outcome::Success(val),
            }
        };

        Outcome::Failure(CodeMessage {
            http_code: http::StatusCode::BAD_REQUEST,
            message: String::from("No searching parameters"),
        })
    }

    pub async fn delete_setting(
        db: &DatabaseConnection,
        settings_model: SettingsModel,
    ) -> Result<u64, String> {
        match Settings::delete_by_id(settings_model.id).exec(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Err(err.to_string());
            }
            Ok(val) => Ok(val.rows_affected),
        }
    }
}
