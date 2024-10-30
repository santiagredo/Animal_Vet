use actix_web::http;
use models::entities::services::Model as ServiceModel;
use security::core::SessionCore;
use utils::{get_config, CodeMessage, Outcome, Validator};

use crate::data::ServicesData;

pub struct ServicesCore;

impl ServicesCore {
    pub async fn insert_service(
        service_model: ServiceModel,
        session_core: SessionCore,
    ) -> Outcome<ServiceModel, CodeMessage, CodeMessage> {
        let parsed_service = match ServicesCore::parse_service(service_model, true).await {
            Err(err) => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: err,
                })
            }
            Ok(val) => val,
        };

        match ServicesData::select_services(
            &get_config().await.db_url,
            ServiceModel {
                name: parsed_service.name.clone(),
                ..Default::default()
            },
        )
        .await
        {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Success(_) => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: String::from("Service already exists"),
                })
            }
            Outcome::Failure(_) => (),
        };

        ServicesData::insert_service(&get_config().await.db_url, parsed_service, session_core).await
    }

    pub async fn select_services(
        service_model: ServiceModel,
    ) -> Outcome<Vec<ServiceModel>, CodeMessage, CodeMessage> {
        ServicesData::select_services(&get_config().await.db_url, service_model).await
    }

    pub async fn update_service(
        service_model: ServiceModel,
        session_core: SessionCore,
    ) -> Outcome<ServiceModel, CodeMessage, CodeMessage> {
        let parsed_service = match ServicesCore::parse_service(service_model, false).await {
            Err(err) => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: err,
                })
            }
            Ok(val) => val,
        };

        match ServicesData::select_services(
            &get_config().await.db_url,
            ServiceModel {
                name: parsed_service.name.clone(),
                ..Default::default()
            },
        )
        .await
        {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Success(val) => {
                for service in val {
                    if service.name != parsed_service.name {
                        return Outcome::Failure(CodeMessage {
                            http_code: http::StatusCode::BAD_REQUEST,
                            message: String::from("Service already exists"),
                        });
                    };
                }
            }
            Outcome::Failure(_) => (),
        };

        ServicesData::update_service(&get_config().await.db_url, parsed_service, session_core).await
    }

    pub async fn delete_service(
        service_model: ServiceModel,
        session_core: SessionCore,
    ) -> Outcome<u64, CodeMessage, CodeMessage> {
        ServicesData::delete_service(&get_config().await.db_url, service_model, session_core).await
    }
}

impl ServicesCore {
    async fn parse_service(
        service_model: ServiceModel,
        err_on_none: bool,
    ) -> Result<ServiceModel, String> {
        let service_id = service_model.service_id;

        let mut name = match service_model.name {
            val if val.is_none() && err_on_none => return Err(format!("Name cannot be empty")),
            None => None,
            Some(val) => Some(val),
        };

        if name.is_some() {
            name = match Validator::validate_empty_field(name, "Name") {
                Err(err) => return Err(err),
                Ok(val) => Some(val),
            }
        }

        let duration = match service_model.duration {
            val if val.is_none() && err_on_none => return Err(format!("Duration cannot be empty")),
            None => None,
            Some(val) => Some(val),
        };

        let is_enabled = match service_model.is_enabled {
            val if val.is_none() && err_on_none => {
                return Err(format!("Is enabled cannot be empty"))
            }
            None => false,
            Some(val) => val,
        };

        Ok(ServiceModel {
            service_id,
            name,
            duration,
            is_enabled: Some(is_enabled),
            ..Default::default()
        })
    }
}
