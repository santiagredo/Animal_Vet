use actix_web::http;
use chrono::{Datelike, TimeDelta};
use models::entities::appointments::Model as AppointmentsModel;
use security::core::SessionCore;
use utils::{get_config, CodeMessage, Outcome};

use crate::data::AppointmentsData;

use super::{DaysCore, ServicesCore, SpecialDatesCore, UnavailableHoursCore, WorkDaysCore};

pub struct AppointmentsCore;

impl AppointmentsCore {
    pub async fn insert_appointment(
        appointments_model: AppointmentsModel,
        session_core: SessionCore,
    ) -> Outcome<AppointmentsModel, CodeMessage, CodeMessage> {
        let parsed_appointment = match Self::parse_appointment(appointments_model, true).await {
            Err(err) => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: err,
                })
            }
            Ok(val) => val,
        };

        let validated_appointment = match Self::validate_appointment(parsed_appointment).await {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Failure(fail) => return Outcome::Failure(fail),
            Outcome::Success(val) => val,
        };

        AppointmentsData::insert_appointment(
            &get_config().await.db_url,
            validated_appointment,
            session_core,
        )
        .await
    }

    pub async fn select_appointments(
        appointments_model: AppointmentsModel,
    ) -> Outcome<Vec<AppointmentsModel>, CodeMessage, CodeMessage> {
        AppointmentsData::select_appointments(&get_config().await.db_url, appointments_model, None)
            .await
    }

    pub async fn update_appointment(
        appointments_model: AppointmentsModel,
        session_core: SessionCore,
    ) -> Outcome<AppointmentsModel, CodeMessage, CodeMessage> {
        if appointments_model.is_canceled.is_some_and(|x| x == true)
            || appointments_model.pet_id.is_some()
        {
            return AppointmentsData::update_appointment(
                &get_config().await.db_url,
                AppointmentsModel {
                    appointment_id: appointments_model.appointment_id,
                    pet_id: appointments_model.pet_id,
                    is_canceled: appointments_model.is_canceled,
                    ..Default::default()
                },
                session_core,
            )
            .await;
        }

        return Outcome::Failure(CodeMessage {
            http_code: http::StatusCode::BAD_REQUEST,
            message: format!("Appointment can only be either canceled or pet id updated"),
        });
    }

    pub async fn delete_appointment(
        appointments_model: AppointmentsModel,
        session_core: SessionCore,
    ) -> Outcome<u64, CodeMessage, CodeMessage> {
        AppointmentsData::delete_appointment(
            &get_config().await.db_url,
            appointments_model,
            session_core,
        )
        .await
    }
}

impl AppointmentsCore {
    pub async fn parse_appointment(
        appointments_model: AppointmentsModel,
        err_on_none: bool,
    ) -> Result<AppointmentsModel, String> {
        let appointment_id = appointments_model.appointment_id;

        let user_id = match appointments_model.user_id {
            None => return Err(format!("User id cannot be empty")),
            Some(0) => return Err(format!("User id cannot be zero")),
            Some(val) => Some(val),
        };

        let pet_id = match appointments_model.pet_id {
            None => return Err(format!("Pet id cannot be empty")),
            Some(0) => return Err(format!("Pet id cannot be zero")),
            Some(val) => Some(val),
        };

        let date = match appointments_model.date {
            val if val.is_none() && err_on_none => return Err(format!("Date cannot be empty")),
            None => None,
            Some(val) => Some(val),
        };

        let service_id = match appointments_model.service_id {
            val if val.is_none() && err_on_none => {
                return Err(format!("Service id cannot be empty"))
            }
            None => None,
            Some(0) => return Err(format!("Service id cannot be zero")),
            Some(val) => Some(val),
        };

        let is_canceled = match appointments_model.is_canceled {
            None => Some(false),
            Some(val) => Some(val),
        };

        Ok(AppointmentsModel {
            appointment_id,
            user_id,
            pet_id,
            date,
            service_id,
            is_canceled,
            ..Default::default()
        })
    }

    pub async fn validate_appointment(
        appointments_model: AppointmentsModel,
    ) -> Outcome<AppointmentsModel, CodeMessage, CodeMessage> {
        // check service exists and is enabled
        let service_id = match appointments_model.service_id {
            None => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: format!("Service Id cannot be empty"),
                })
            }
            Some(val) => val,
        };

        let appointment_service =
            match ServicesCore::select_services(models::entities::services::Model {
                service_id,
                ..Default::default()
            })
            .await
            {
                Outcome::Error(err) => return Outcome::Error(err),
                Outcome::Failure(fail) => return Outcome::Failure(fail),
                Outcome::Success(services) => {
                    let owned_services = services.to_owned();

                    owned_services
                        .into_iter()
                        .find(|s| s.service_id == service_id)
                }
            };

        let appointment_service = match appointment_service {
            None => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: format!("Invalid service appointment"),
                })
            }
            Some(service) => {
                if service.is_enabled.is_none() || service.is_enabled.is_some_and(|x| x == false) {
                    return Outcome::Failure(CodeMessage {
                        http_code: http::StatusCode::BAD_REQUEST,
                        message: format!("Requested service is disabled"),
                    });
                }

                service.to_owned()
            }
        };

        // check if appointment with same service, date and time exists
        match AppointmentsData::select_appointments(
            &get_config().await.db_url,
            AppointmentsModel {
                date: appointments_model.date,
                service_id: appointments_model.service_id,
                ..Default::default()
            },
            None,
        )
        .await
        {
            Outcome::Success(_) => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: format!("Appointment date and time already reserved"),
                })
            }
            _ => (),
        }

        // get day of the week id
        let week_day = match appointments_model.date {
            None => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: format!("Date cannot be empty"),
                });
            }
            Some(val) => val.weekday().to_string(),
        };

        let day_id = match DaysCore::select_day(
            models::entities::days::Model {
                day_id: 0,
                name: Some(week_day),
            },
            false,
        )
        .await
        {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Failure(fail) => return Outcome::Failure(fail),
            Outcome::Success(val) => val,
        };

        let day_id = match day_id.get(0) {
            None => return Outcome::Error(CodeMessage { http_code: http::StatusCode::INTERNAL_SERVER_ERROR, message: format!("Failed to retreive day id") }),
            Some(val) => val.day_id
        };

        // get work day for service under week day id
        let work_day = match WorkDaysCore::select_work_days(models::entities::work_days::Model {
            service_id: Some(service_id),
            day_id: Some(day_id),
            ..Default::default()
        })
        .await
        {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Failure(fail) => return Outcome::Failure(fail),
            Outcome::Success(val) => val,
        };

        let work_day = match work_day.get(0) {
            None => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: format!("Work day not found"),
                })
            }
            Some(val) => val.to_owned(),
        };

        // check if week day is enabled
        if work_day.is_enabled.is_some_and(|x| x == false) {
            return Outcome::Failure(CodeMessage {
                http_code: http::StatusCode::BAD_REQUEST,
                message: format!("Week day closed for requested service"),
            });
        }

        let open_time = match work_day.open_time {
            None => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::NOT_IMPLEMENTED,
                    message: format!("Open time not implemented"),
                });
            }
            Some(val) => val,
        };

        let close_time = match work_day.close_time {
            None => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::NOT_IMPLEMENTED,
                    message: format!("Close time not implemented"),
                });
            }
            Some(val) => val,
        };

        let lunch_from_time = match work_day.lunch_from_time {
            None => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::NOT_IMPLEMENTED,
                    message: format!("Lunch From time not implemented"),
                });
            }
            Some(val) => val,
        };

        let lunch_to_time = match work_day.lunch_to_time {
            None => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::NOT_IMPLEMENTED,
                    message: format!("Lunch To time not implemented"),
                });
            }
            Some(val) => val,
        };

        let service_duration = match appointment_service.duration {
            None => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::NOT_IMPLEMENTED,
                    message: format!("Service duration not implemented"),
                });
            }
            Some(val) => val,
        };

        let appointment_date = match appointments_model.date {
            None => {
                return Outcome::Failure(CodeMessage {
                    http_code: http::StatusCode::BAD_REQUEST,
                    message: format!("Date cannot be empty"),
                });
            }
            Some(val) => val,
        };

        // check appointment isn't set before opening time
        if appointment_date.time() < open_time {
            return Outcome::Failure(CodeMessage {
                http_code: http::StatusCode::BAD_REQUEST,
                message: format!("Appointment is set before available hours"),
            });
        }

        // check appointment isn't set after closing time
        if appointment_date.time()
            > close_time
                .overflowing_sub_signed(TimeDelta::minutes(service_duration.into()))
                .0
        {
            return Outcome::Failure(CodeMessage {
                http_code: http::StatusCode::BAD_REQUEST,
                message: format!("Appointment is set before available hours"),
            });
        }

        // check appointment isn't set between lunch
        if appointment_date.time()
            > lunch_from_time
                .overflowing_sub_signed(TimeDelta::minutes(service_duration.into()))
                .0
            && appointment_date.time() < lunch_to_time
        {
            return Outcome::Failure(CodeMessage {
                http_code: http::StatusCode::BAD_REQUEST,
                message: format!("Appointment is set during lunch hours"),
            });
        }

        // get special date
        let special_date =
            match SpecialDatesCore::select_special_dates(models::entities::special_dates::Model {
                date: Some(appointment_date.date()),
                ..Default::default()
            })
            .await
            {
                Outcome::Error(err) => return Outcome::Error(err),
                Outcome::Success(val) => Some(val),
                Outcome::Failure(_) => None,
            };

        match special_date {
            None => {}
            Some(dates) => {
                for val in dates.iter() {
                    // None == all services
                    if val.service_id.is_none() || val.service_id.is_some_and(|x| x == service_id) {
                        match val.is_working_date {
                            Some(true) => (),
                            _ => {
                                return Outcome::Failure(CodeMessage {
                                    http_code: http::StatusCode::BAD_REQUEST,
                                    message: format!("Date is a non working date"),
                                });
                            }
                        };

                        let open_time = match val.open_time {
                            None => {
                                return Outcome::Failure(CodeMessage {
                                    http_code: http::StatusCode::NOT_IMPLEMENTED,
                                    message: format!("Open time not implemented"),
                                });
                            }
                            Some(val) => val,
                        };

                        let close_time = match val.close_time {
                            None => {
                                return Outcome::Failure(CodeMessage {
                                    http_code: http::StatusCode::NOT_IMPLEMENTED,
                                    message: format!("Close time not implemented"),
                                });
                            }
                            Some(val) => val,
                        };

                        let lunch_from_time = match val.lunch_from_time {
                            None => {
                                return Outcome::Failure(CodeMessage {
                                    http_code: http::StatusCode::NOT_IMPLEMENTED,
                                    message: format!("Lunch From time not implemented"),
                                });
                            }
                            Some(val) => val,
                        };

                        let lunch_to_time = match val.lunch_to_time {
                            None => {
                                return Outcome::Failure(CodeMessage {
                                    http_code: http::StatusCode::NOT_IMPLEMENTED,
                                    message: format!("Lunch To time not implemented"),
                                });
                            }
                            Some(val) => val,
                        };

                        // check appointment isn't set before opening time
                        if appointment_date.time() < open_time {
                            return Outcome::Failure(CodeMessage {
                                http_code: http::StatusCode::BAD_REQUEST,
                                message: format!("Appointment is set before available hours"),
                            });
                        }

                        // check appointment isn't set after closing time
                        if appointment_date.time()
                            > close_time
                                .overflowing_sub_signed(TimeDelta::minutes(service_duration.into()))
                                .0
                        {
                            return Outcome::Failure(CodeMessage {
                                http_code: http::StatusCode::BAD_REQUEST,
                                message: format!("Appointment is set before available hours"),
                            });
                        }

                        // check appointment isn't set between lunch
                        if appointment_date.time()
                            > lunch_from_time
                                .overflowing_sub_signed(TimeDelta::minutes(service_duration.into()))
                                .0
                            && appointment_date.time() < lunch_to_time
                        {
                            return Outcome::Failure(CodeMessage {
                                http_code: http::StatusCode::BAD_REQUEST,
                                message: format!("Appointment is set during lunch hours"),
                            });
                        }
                    }
                }
            }
        }

        let unavailable_hours = match UnavailableHoursCore::select_unavailable_hours(
            models::entities::unavailable_hours::Model {
                date: Some(appointment_date.date()),
                ..Default::default()
            },
        )
        .await
        {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Success(val) => Some(val),
            Outcome::Failure(_) => None,
        };

        match unavailable_hours {
            None => {}
            Some(hours) => {
                for val in hours.iter() {
                    // None == all services
                    if val.service_id.is_none() || val.service_id.is_some_and(|x| x == service_id) {
                        let start_time = match val.start_time {
                            None => {
                                return Outcome::Failure(CodeMessage {
                                    http_code: http::StatusCode::NOT_IMPLEMENTED,
                                    message: format!("Start Time not implemented"),
                                });
                            }
                            Some(val) => val,
                        };

                        let end_time = match val.end_time {
                            None => {
                                return Outcome::Failure(CodeMessage {
                                    http_code: http::StatusCode::NOT_IMPLEMENTED,
                                    message: format!("End Time not implemented"),
                                });
                            }
                            Some(val) => val,
                        };

                        if appointment_date.time()
                            > start_time
                                .overflowing_sub_signed(TimeDelta::minutes(service_duration.into()))
                                .0
                            && appointment_date.time() < end_time
                        {
                            return Outcome::Failure(CodeMessage {
                                http_code: http::StatusCode::BAD_REQUEST,
                                message: format!("Appointment is set during unavailable hours"),
                            });
                        }
                    }
                }
            }
        }

        Outcome::Success(appointments_model)
    }
}
