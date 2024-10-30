use actix_web::http;
use models::entities::appointments::Model as AppointmentsModel;
use security::core::SessionCore;
use utils::{get_config, CodeMessage, Outcome};

use crate::data::AppointmentsData;

pub struct AppointmentsCore;

impl AppointmentsCore {
    pub async fn insert_appointment(
        appointments_model: AppointmentsModel,
        session_core: SessionCore,
    ) -> Outcome<AppointmentsModel, CodeMessage, CodeMessage> {
        let mut parsed_appointment =
            match backoffice::core::AppointmentsCore::parse_appointment(appointments_model, true)
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

        parsed_appointment.user_id = Some(session_core.user.user_id);

        let validated_appointment = match backoffice::core::AppointmentsCore::validate_appointment(
            parsed_appointment,
        )
        .await
        {
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

    pub async fn select_appointment(
        mut appointments_model: AppointmentsModel,
        session_core: SessionCore,
    ) -> Outcome<AppointmentsModel, CodeMessage, CodeMessage> {
        appointments_model.user_id = Some(session_core.user.user_id);

        AppointmentsData::select_appointment(&get_config().await.db_url, appointments_model).await
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
                    user_id: Some(session_core.user.user_id),
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

    // pub async fn delete_appointment(
    //     appointments_model: AppointmentsModel,
    //     session_core: SessionCore,
    // ) -> Outcome<u64, CodeMessage, CodeMessage> {
    //     AppointmentsData::delete_appointment(
    //         &get_config().await.db_url,
    //         appointments_model,
    //         session_core,
    //     )
    //     .await
    // }
}
