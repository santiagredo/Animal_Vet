use actix_web::http;
use models::entities::unavailable_hours::Model as UnavailableHoursModel;
use security::core::SessionCore;
use utils::{get_config, CodeMessage, Outcome, Validator};

use crate::data::UnavailableHoursData;

pub struct UnavailableHoursCore;

impl UnavailableHoursCore {
    pub async fn insert_unavailable_hours(
        unavailable_hours_model: UnavailableHoursModel,
        session_core: SessionCore,
    ) -> Outcome<UnavailableHoursModel, CodeMessage, CodeMessage> {
        let parsed_unavailable_hours = match UnavailableHoursCore::parse_unavailable_hours(
            unavailable_hours_model,
            true,
        )
        .await
        {
            Err(err) => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: err,
                })
            }
            Ok(val) => val,
        };

        match UnavailableHoursData::select_unavailable_hours(
            &get_config().await.db_url,
            UnavailableHoursModel {
                service_id: parsed_unavailable_hours.service_id.clone(),
                date: parsed_unavailable_hours.date.clone(),
                ..Default::default()
            },
            None,
        )
        .await
        {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Success(_) => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: String::from(
                        "Unavailable hour with same service and date already exists",
                    ),
                })
            }
            Outcome::Failure(_) => (),
        };

        UnavailableHoursData::insert_unavailable_hours(
            &get_config().await.db_url,
            parsed_unavailable_hours,
            session_core,
        )
        .await
    }

    pub async fn select_unavailable_hours(
        unavailable_hours_model: UnavailableHoursModel,
    ) -> Outcome<Vec<UnavailableHoursModel>, CodeMessage, CodeMessage> {
        UnavailableHoursData::select_unavailable_hours(
            &get_config().await.db_url,
            unavailable_hours_model,
            None,
        )
        .await
    }

    pub async fn update_unavailble_hours(
        unavailable_hours_model: UnavailableHoursModel,
        session_core: SessionCore,
    ) -> Outcome<UnavailableHoursModel, CodeMessage, CodeMessage> {
        let parsed_unavailable_hours =
            match UnavailableHoursCore::parse_unavailable_hours(unavailable_hours_model, false)
                .await
            {
                Err(err) => {
                    return Outcome::Failure(CodeMessage {
                        http_code: http::StatusCode::BAD_REQUEST,
                        message: err,
                    })
                }
                Ok(val) => val,
            };

        match UnavailableHoursData::select_unavailable_hours(
            &get_config().await.db_url,
            UnavailableHoursModel {
                service_id: parsed_unavailable_hours.service_id.clone(),
                date: parsed_unavailable_hours.date.clone(),
                ..Default::default()
            },
            None
        )
        .await
        {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Failure(_) => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: String::from("Unavailable hours doesn't exist"),
                })
            }
            Outcome::Success(_) => (),
        };

        UnavailableHoursData::update_unavailble_hours(
            &get_config().await.db_url,
            parsed_unavailable_hours,
            session_core,
        )
        .await
    }

    pub async fn delete_unavailable_hours(
        unavailable_hours_model: UnavailableHoursModel,
        session_core: SessionCore,
    ) -> Outcome<u64, CodeMessage, CodeMessage> {
        UnavailableHoursData::delete_unavailable_hours(
            &get_config().await.db_url,
            unavailable_hours_model,
            session_core,
        )
        .await
    }
}

impl UnavailableHoursCore {
    async fn parse_unavailable_hours(
        unavailable_hours_model: UnavailableHoursModel,
        err_on_none: bool,
    ) -> Result<UnavailableHoursModel, String> {
        let unavailable_hour_id = unavailable_hours_model.unavailable_hour_id;

        let service_id = match unavailable_hours_model.service_id {
            None => None,
            Some(0) => return Err(format!("Service id cannot be zero")),
            Some(val) => Some(val),
        };

        let date = match unavailable_hours_model.date {
            None => return Err(format!("Date cannot be empty")),
            Some(val) => Some(val),
        };

        let start_time = match unavailable_hours_model.start_time {
            time if time.is_none() && err_on_none => {
                return Err(format!("Start time cannot be empty"))
            }
            None => None,
            Some(val) => Some(val),
        };

        let end_time = match unavailable_hours_model.end_time {
            time if time.is_none() && err_on_none => {
                return Err(format!("End time cannot be empty"))
            }
            None => None,
            Some(val) => Some(val),
        };

        let mut reason = match unavailable_hours_model.reason {
            val if val.is_none() && err_on_none => return Err(format!("Reason cannot be empty")),
            None => None,
            Some(val) => Some(val),
        };

        if reason.is_some() {
            reason = match Validator::validate_empty_field(reason, "Reason") {
                Err(err) => return Err(err),
                Ok(val) => Some(val),
            }
        }

        Ok(UnavailableHoursModel {
            unavailable_hour_id,
            service_id,
            date,
            start_time,
            end_time,
            reason,
            ..Default::default()
        })
    }
}
