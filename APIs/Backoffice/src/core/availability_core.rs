use actix_web::http;
use chrono::{Datelike, Days, Local, NaiveDate, NaiveDateTime, TimeDelta};
use models::entities::services::Model as ServiceModel;
use sea_orm::prelude::Time;
use utils::{get_config, Availability, CodeMessage, Outcome};

use crate::data::{AppointmentsData, SpecialDatesData, UnavailableHoursData};

use super::{DaysCore, ServicesCore, WorkDaysCore};

pub struct AvailabilityCore;

impl AvailabilityCore {
    pub async fn select_availability(
        service_model: ServiceModel,
    ) -> Outcome<Vec<Availability>, CodeMessage, CodeMessage> {
        let service = match ServicesCore::select_services(service_model).await {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Failure(fail) => return Outcome::Failure(fail),
            Outcome::Success(val) => val,
        };

        let service = match service.get(0) {
            None => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: format!("Invalid service id"),
                })
            }
            Some(val) => val.to_owned(),
        };

        let days = match DaysCore::select_day(
            models::entities::days::Model {
                ..Default::default()
            },
            true,
        )
        .await
        {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Failure(fail) => return Outcome::Failure(fail),
            Outcome::Success(val) => val,
        };

        let work_days = match WorkDaysCore::select_work_days(models::entities::work_days::Model {
            service_id: Some(service.service_id),
            ..Default::default()
        })
        .await
        {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Failure(_) => Vec::new(),
            Outcome::Success(val) => val,
        };

        let current_date = Local::now();
        let date_two_weeks_ahead = Local::now().checked_add_days(Days::new(13)).unwrap();

        let special_dates = match SpecialDatesData::select_special_dates(
            &get_config().await.db_url,
            models::entities::special_dates::Model {
                date: Some(
                    NaiveDate::from_ymd_opt(
                        current_date.year(),
                        current_date.month(),
                        current_date.day(),
                    )
                    .unwrap(),
                ),
                ..Default::default()
            },
            Some(models::entities::special_dates::Model {
                date: Some(
                    NaiveDate::from_ymd_opt(
                        date_two_weeks_ahead.year(),
                        date_two_weeks_ahead.month(),
                        date_two_weeks_ahead.day(),
                    )
                    .unwrap(),
                ),
                ..Default::default()
            }),
        )
        .await
        {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Failure(_) => Vec::new(),
            Outcome::Success(val) => val,
        };

        let unavailable_hours = match UnavailableHoursData::select_unavailable_hours(
            &get_config().await.db_url,
            models::entities::unavailable_hours::Model {
                date: Some(
                    NaiveDate::from_ymd_opt(
                        current_date.year(),
                        current_date.month(),
                        current_date.day(),
                    )
                    .unwrap(),
                ),
                ..Default::default()
            },
            Some(models::entities::unavailable_hours::Model {
                service_id: Some(service.service_id),
                date: Some(
                    NaiveDate::from_ymd_opt(
                        date_two_weeks_ahead.year(),
                        date_two_weeks_ahead.month(),
                        date_two_weeks_ahead.day(),
                    )
                    .unwrap(),
                ),
                ..Default::default()
            }),
        )
        .await
        {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Failure(_) => Vec::new(),
            Outcome::Success(val) => val,
        };

        let appointments = match AppointmentsData::select_appointments(
            &get_config().await.db_url,
            models::entities::appointments::Model {
                service_id: Some(service.service_id),
                date: Some(NaiveDateTime::new(
                    current_date.date_naive(),
                    current_date.time(),
                )),
                is_canceled: Some(false),
                ..Default::default()
            },
            Some(models::entities::appointments::Model {
                service_id: Some(service.service_id),
                date: Some(NaiveDateTime::new(
                    date_two_weeks_ahead.date_naive(),
                    date_two_weeks_ahead.time(),
                )),
                is_canceled: Some(false),
                ..Default::default()
            }),
        )
        .await
        {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Failure(_) => Vec::new(),
            Outcome::Success(val) => val,
        };

        let mut available_dates: Vec<Availability> = Vec::new();

        for day_sum in 0..=13 {
            let date = current_date.checked_add_days(Days::new(day_sum)).unwrap();
            let week_day = date.weekday().to_string();

            let day = days
                .iter()
                .find(|x| x.name.as_ref().is_some_and(|y| y.starts_with(&week_day)))
                .unwrap();

            let work_day = match work_days.iter().find(|x| x.day_id == Some(day.day_id)) {
                None => continue,
                Some(val) => val,
            };

            if work_day.is_enabled.is_some_and(|x| x == true) {
                let mut day_availability = Availability {
                    date: date.date_naive(),
                    service_id: service.service_id,
                    open_time: work_day.open_time.unwrap().into(),
                    close_time: work_day.close_time.unwrap().into(),
                    lunch_from_time: work_day.lunch_from_time.unwrap().into(),
                    lunch_to_time: work_day.lunch_to_time.unwrap().into(),
                    ..Default::default()
                };

                Self::generate_time_slots(&mut day_availability, &service);

                available_dates.push(day_availability);
            }
        }

        for special_date in special_dates {
            if special_date.is_working_date.is_some_and(|x| x == false)
                && (special_date
                    .service_id
                    .is_some_and(|x| x == service.service_id)
                    || special_date.service_id.is_none())
            {
                let date_to_remove = match available_dates
                    .iter()
                    .position(|x| Some(x.date) == special_date.date)
                {
                    None => continue,
                    Some(val) => val,
                };

                available_dates.remove(date_to_remove);

                continue;
            }

            let available_date = match available_dates
                .iter()
                .position(|x| Some(x.date) == special_date.date)
            {
                None => continue,
                Some(val) => &mut available_dates[val],
            };

            match special_date.open_time == Some(available_date.open_time)
                && special_date.lunch_from_time == Some(available_date.lunch_from_time)
                && special_date.lunch_to_time == Some(available_date.lunch_to_time)
                && special_date.close_time == Some(available_date.close_time)
            {
                true => continue,
                false => {
                    available_date.open_time =
                        special_date.open_time.unwrap_or(available_date.open_time);

                    available_date.lunch_from_time = special_date
                        .lunch_from_time
                        .unwrap_or(available_date.lunch_from_time);

                    available_date.lunch_to_time = special_date
                        .lunch_to_time
                        .unwrap_or(available_date.lunch_to_time);

                    available_date.close_time =
                        special_date.close_time.unwrap_or(available_date.close_time);

                    Self::generate_time_slots(
                        available_date,
                        &ServiceModel {
                            service_id: service.service_id,
                            duration: service.duration,
                            ..Default::default()
                        },
                    );
                }
            };
        }

        for unavailable_hour in unavailable_hours {
            if unavailable_hour.service_id.is_none()
                || unavailable_hour
                    .service_id
                    .is_some_and(|x| x == service.service_id)
            {
                let date_to_update = match available_dates
                    .iter()
                    .position(|x| Some(x.date) == unavailable_hour.date)
                {
                    None => continue,
                    Some(val) => &mut available_dates[val],
                };

                date_to_update.time_slots.retain(|x| {
                    Some(x) < unavailable_hour.start_time.as_ref()
                        || Some(x) >= unavailable_hour.end_time.as_ref()
                });
            }
        }

        for appointment in appointments {
            let appointment_date = match appointment.date {
                None => continue,
                Some(val) => NaiveDate::from_ymd_opt(val.year(), val.month(), val.day()),
            };

            let appointment_date = match appointment_date {
                None => continue,
                Some(val) => val,
            };

            let date_to_update = match available_dates
                .iter()
                .position(|x| x.date == appointment_date)
            {
                None => continue,
                Some(val) => &mut available_dates[val],
            };

            let appointment_hour = match appointment.date {
                None => continue,
                Some(val) => val.time(),
            };

            date_to_update.time_slots.retain(|x| {
                x < &appointment_hour
                    || x >= &appointment_hour
                        .overflowing_add_signed(TimeDelta::minutes(
                            service.duration.unwrap_or(15).into(),
                        ))
                        .0
            })
        }

        Outcome::Success(available_dates)
    }
}

impl AvailabilityCore {
    fn generate_time_slots(availability: &mut Availability, service: &ServiceModel) {
        let mut time_slots: Vec<Time> = Vec::new();

        let service_duration = service.duration.unwrap_or(15);

        let mut start_time = availability.open_time;

        while start_time
            <= availability
                .lunch_from_time
                .clone()
                .overflowing_sub_signed(TimeDelta::minutes(service_duration.into()))
                .0
        {
            time_slots.push(start_time);

            start_time = start_time
                .overflowing_add_signed(TimeDelta::minutes(service_duration.into()))
                .0;
        }

        let mut start_time = availability.lunch_to_time;

        while start_time
            <= availability
                .close_time
                .clone()
                .overflowing_sub_signed(TimeDelta::minutes(service_duration.into()))
                .0
        {
            time_slots.push(start_time);

            start_time = start_time
                .overflowing_add_signed(TimeDelta::minutes(service_duration.into()))
                .0;
        }

        availability.time_slots = time_slots;
    }
}
