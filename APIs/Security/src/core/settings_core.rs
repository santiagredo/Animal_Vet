use actix_web::http;
use models_settings::entities::settings::Model as SettingsModel;
use utils::{get_config, CodeMessage, Outcome};

use crate::data::SettingsData;

pub struct SettingsCore;

impl SettingsCore {
    pub async fn insert_setting(
        settings_model: SettingsModel,
    ) -> Outcome<SettingsModel, CodeMessage, CodeMessage> {
        match SettingsData::select_setting(
            &get_config().await.settings_db_url,
            SettingsModel {
                name: settings_model.name.clone(),
                ..Default::default()
            },
        )
        .await
        {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Success(_) => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: format!("Setting with same name already exists"),
                })
            }
            Outcome::Failure(_) => (),
        };

        SettingsData::insert_setting(&get_config().await.settings_db_url, settings_model).await
    }

    pub async fn select_setting(
        settings_model: SettingsModel,
    ) -> Outcome<SettingsModel, CodeMessage, CodeMessage> {
        SettingsData::select_setting(&get_config().await.settings_db_url, settings_model).await
    }

    pub async fn delete_setting(settings_model: SettingsModel) -> Result<u64, String> {
        SettingsData::delete_setting(&get_config().await.settings_db_url, settings_model).await
    }
}
