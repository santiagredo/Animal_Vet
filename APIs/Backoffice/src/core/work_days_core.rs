use actix_web::http;
use chrono::NaiveTime;
use models::entities::work_days::Model as WorkDayModel;
use security::core::SessionCore;
use utils::{get_config, CodeMessage, Outcome};

use crate::data::WorkDaysData;

pub struct WorkDaysCore;

impl WorkDaysCore {
    pub async fn insert_work_day(
        work_day_model: WorkDayModel,
        session_core: SessionCore,
    ) -> Outcome<WorkDayModel, CodeMessage, CodeMessage> {
        let parsed_work_day = match WorkDaysCore::parse_work_day(work_day_model, true).await {
            Err(err) => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: err,
                })
            }
            Ok(val) => val,
        };

        match WorkDaysData::select_work_days(
            &get_config().await.db_url,
            WorkDayModel {
                service_id: parsed_work_day.service_id.clone(),
                day_id: parsed_work_day.day_id.clone(),
                ..Default::default()
            },
        )
        .await
        {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Success(_) => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: String::from("Work day with same service and week day already exists"),
                })
            }
            Outcome::Failure(_) => (),
        };

        WorkDaysData::insert_work_day(&get_config().await.db_url, parsed_work_day, session_core)
            .await
    }

    pub async fn select_work_days(
        work_day_model: WorkDayModel,
    ) -> Outcome<Vec<WorkDayModel>, CodeMessage, CodeMessage> {
        WorkDaysData::select_work_days(&get_config().await.db_url, work_day_model).await
    }

    pub async fn update_work_day(
        work_day_model: WorkDayModel,
        session_core: SessionCore,
    ) -> Outcome<WorkDayModel, CodeMessage, CodeMessage> {
        let parsed_work_day = match WorkDaysCore::parse_work_day(work_day_model, false).await {
            Err(err) => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: err,
                })
            }
            Ok(val) => val,
        };

        WorkDaysData::update_work_day(&get_config().await.db_url, parsed_work_day, session_core)
            .await
    }

    pub async fn delete_work_day(
        work_day_model: WorkDayModel,
        session_core: SessionCore,
    ) -> Outcome<u64, CodeMessage, CodeMessage> {
        WorkDaysData::delete_work_day(&get_config().await.db_url, work_day_model, session_core)
            .await
    }
}

impl WorkDaysCore {
    async fn parse_work_day(
        work_day_model: WorkDayModel,
        err_on_none: bool,
    ) -> Result<WorkDayModel, String> {
        let work_day_id = work_day_model.work_day_id;

        let service_id = match work_day_model.service_id {
            None => return Err(format!("Service id cannot be empty")),
            Some(0) => return Err(format!("Service id cannot be zero")),
            Some(val) => Some(val),
        };

        let day_id = match work_day_model.day_id {
            None => return Err(format!("Day id cannot be empty")),
            Some(0) => return Err(format!("Day id cannot be zero")),
            day if day.is_some_and(|x| x > 7) => {
                return Err(format!("Day id cannot be greater than 7"))
            }
            Some(val) => Some(val),
        };

        let is_enabled = match work_day_model.is_enabled {
            val if val.is_none() && err_on_none => {
                return Err(format!("Is enabled cannot be empty"))
            }
            None => Some(false),
            Some(val) => Some(val),
        };

        let zero_time = match NaiveTime::from_hms_opt(0, 0, 0) {
            None => return Err(format!("Error processing time")),
            Some(val) => val,
        };

        let open_time = match work_day_model.open_time {
            val if val.is_none() && err_on_none => {
                return Err(format!("Open time cannot be empty"))
            }
            None => None,
            time if time.is_some_and(|x| x.eq(&zero_time)) => {
                return Err(format!("Open time cannot be zero"))
            }
            Some(val) => Some(val),
        };

        let close_time = match work_day_model.close_time {
            val if val.is_none() && err_on_none => {
                return Err(format!("Close time cannot be empty"))
            }
            None => None,
            time if time.is_some_and(|x| x.eq(&zero_time)) => {
                return Err(format!("Close time cannot be zero"))
            }
            Some(val) => Some(val),
        };

        let lunch_from_time = match work_day_model.lunch_from_time {
            val if val.is_none() && err_on_none => {
                return Err(format!("Lunch from time cannot be empty"))
            }
            None => None,
            time if time.is_some_and(|x| x.eq(&zero_time)) => {
                return Err(format!("Lunch from time cannot be zero"))
            }
            Some(val) => Some(val),
        };

        let lunch_to_time = match work_day_model.lunch_to_time {
            val if val.is_none() && err_on_none => {
                return Err(format!("Lunch to time cannot be empty"))
            }
            None => None,
            time if time.is_some_and(|x| x.eq(&zero_time)) => {
                return Err(format!("Lunch to time cannot be zero"))
            }
            Some(val) => Some(val),
        };

        Ok(WorkDayModel {
            work_day_id,
            service_id,
            day_id,
            is_enabled,
            open_time,
            close_time,
            lunch_from_time,
            lunch_to_time,
            ..Default::default()
        })
    }
}
