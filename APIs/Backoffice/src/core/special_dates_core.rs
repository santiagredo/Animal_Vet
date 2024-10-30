use actix_web::http;
use models::entities::special_dates::Model as SpecialDateModel;
use security::core::SessionCore;
use utils::{get_config, CodeMessage, Outcome, Validator};

use crate::data::SpecialDatesData;

pub struct SpecialDatesCore;

impl SpecialDatesCore {
    pub async fn insert_special_date(
        special_date_model: SpecialDateModel,
        session_core: SessionCore,
    ) -> Outcome<SpecialDateModel, CodeMessage, CodeMessage> {
        let parsed_special_date =
            match SpecialDatesCore::parse_special_date(special_date_model, true).await {
                Err(err) => {
                    return Outcome::Failure(CodeMessage {
                        http_code: http::StatusCode::BAD_REQUEST,
                        message: err,
                    })
                }
                Ok(val) => val,
            };

        match SpecialDatesData::select_special_dates(
            &get_config().await.db_url,
            SpecialDateModel {
                service_id: parsed_special_date.service_id.clone(),
                date: parsed_special_date.date.clone(),
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
                    message: String::from("Special date with same service and date already exists"),
                })
            }
            Outcome::Failure(_) => (),
        };

        SpecialDatesData::insert_special_date(
            &get_config().await.db_url,
            parsed_special_date,
            session_core,
        )
        .await
    }

    pub async fn select_special_dates(
        special_date_model: SpecialDateModel,
    ) -> Outcome<Vec<SpecialDateModel>, CodeMessage, CodeMessage> {
        SpecialDatesData::select_special_dates(&get_config().await.db_url, special_date_model, None)
            .await
    }

    pub async fn update_special_date(
        special_date_model: SpecialDateModel,
        session_core: SessionCore,
    ) -> Outcome<SpecialDateModel, CodeMessage, CodeMessage> {
        let parsed_special_date =
            match SpecialDatesCore::parse_special_date(special_date_model, false).await {
                Err(err) => {
                    return Outcome::Failure(CodeMessage {
                        http_code: http::StatusCode::BAD_REQUEST,
                        message: err,
                    })
                }
                Ok(val) => val,
            };

        match SpecialDatesData::select_special_dates(
            &get_config().await.db_url,
            SpecialDateModel {
                service_id: parsed_special_date.service_id.clone(),
                date: parsed_special_date.date.clone(),
                ..Default::default()
            },
            None,
        )
        .await
        {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Failure(_) => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: String::from("Special date doesn't exist"),
                })
            }
            Outcome::Success(_) => (),
        };

        SpecialDatesData::update_special_date(
            &get_config().await.db_url,
            parsed_special_date,
            session_core,
        )
        .await
    }

    pub async fn delete_special_date(
        special_date_model: SpecialDateModel,
        session_core: SessionCore,
    ) -> Outcome<u64, CodeMessage, CodeMessage> {
        SpecialDatesData::delete_special_date(
            &get_config().await.db_url,
            special_date_model,
            session_core,
        )
        .await
    }
}

impl SpecialDatesCore {
    async fn parse_special_date(
        special_date_model: SpecialDateModel,
        err_on_none: bool,
    ) -> Result<SpecialDateModel, String> {
        let special_date_id = special_date_model.special_date_id;

        let service_id = match special_date_model.service_id {
            // val if val.is_none() && err_on_none => return Err(format!("Service id cannot be empty")),
            None => None,
            Some(0) => return Err(format!("Service id cannot be zero")),
            Some(val) => Some(val),
        };

        let date = match special_date_model.date {
            None => return Err(format!("Date cannot be empty")),
            Some(val) => Some(val),
        };

        let is_working_date = match special_date_model.is_working_date {
            val if val.is_none() && err_on_none => {
                return Err(format!("Is working date cannot be empty"))
            }
            None => None,
            Some(val) => Some(val),
        };

        let open_time = match special_date_model.open_time {
            time if time.is_none() && err_on_none => {
                return Err(format!("Open time cannot be empty"))
            }
            None => None,
            Some(val) => Some(val),
        };

        let close_time = match special_date_model.close_time {
            time if time.is_none() && err_on_none => {
                return Err(format!("Close time cannot be empty"))
            }
            None => None,
            Some(val) => Some(val),
        };

        let lunch_from_time = match special_date_model.lunch_from_time {
            time if time.is_none() && err_on_none => {
                return Err(format!("Lunch From time cannot be empty"))
            }
            None => None,
            Some(val) => Some(val),
        };

        let lunch_to_time = match special_date_model.lunch_to_time {
            time if time.is_none() && err_on_none => {
                return Err(format!("Lunch To time cannot be empty"))
            }
            None => None,
            Some(val) => Some(val),
        };

        let mut reason = match special_date_model.reason {
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

        Ok(SpecialDateModel {
            special_date_id,
            service_id,
            date,
            is_working_date,
            open_time,
            close_time,
            lunch_from_time,
            lunch_to_time,
            reason,
            ..Default::default()
        })
    }
}
