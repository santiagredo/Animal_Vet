use actix_session::Session;
use actix_web::{get, patch, post, web, HttpResponse, Responder};
use models::entities::appointments::Model as AppointmentsModel;
use security::core::{SessionCore, UserRolesCore};
use utils::{Outcome, CREATE_PERMISSION, READ_PERMISSION, UPDATE_PERMISSION};

use crate::core::AppointmentsCore;

#[post("")]
pub async fn insert_appointment(
    json: web::Json<AppointmentsModel>,
    mut session: Session,
) -> impl Responder {
    let session_core = match SessionCore::session_validator(&mut session).await {
        Outcome::Error(err) => return HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => return HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(session) => {
            let appointment_permissions = match session.role.appointment_permissions {
                None => return HttpResponse::Unauthorized().json("User doesn't have permissions"),
                Some(val) => val,
            };

            if !UserRolesCore::has_permission(appointment_permissions, CREATE_PERMISSION) {
                return HttpResponse::Unauthorized().json("User doesn't have create permissions");
            }

            session
        }
    };

    match AppointmentsCore::insert_appointment(json.0, session_core).await {
        Outcome::Error(err) => HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(val) => HttpResponse::Ok().json(val),
    }
}

#[get("")]
pub async fn select_appointment(
    query: web::Query<AppointmentsModel>,
    mut session: Session,
) -> impl Responder {
    let session_core = match SessionCore::session_validator(&mut session).await {
        Outcome::Error(err) => return HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => return HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(session) => {
            let appointment_permissions = match session.role.appointment_permissions {
                None => return HttpResponse::Unauthorized().json("User doesn't have permissions"),
                Some(val) => val,
            };

            if !UserRolesCore::has_permission(appointment_permissions, READ_PERMISSION) {
                return HttpResponse::Unauthorized().json("User doesn't have read permissions");
            }

            session
        }
    };

    match AppointmentsCore::select_appointment(query.0, session_core).await {
        Outcome::Error(err) => HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(val) => HttpResponse::Ok().json(val),
    }
}

#[patch("")]
pub async fn update_appointment(
    json: web::Json<AppointmentsModel>,
    mut session: Session,
) -> impl Responder {
    let session_core = match SessionCore::session_validator(&mut session).await {
        Outcome::Error(err) => return HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => return HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(session) => {
            let appointment_permissions = match session.role.appointment_permissions {
                None => return HttpResponse::Unauthorized().json("User doesn't have permissions"),
                Some(val) => val,
            };

            if !UserRolesCore::has_permission(appointment_permissions, UPDATE_PERMISSION) {
                return HttpResponse::Unauthorized().json("User doesn't have update permissions");
            }

            session
        }
    };

    match AppointmentsCore::update_appointment(json.0, session_core).await {
        Outcome::Error(err) => HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(val) => HttpResponse::Ok().json(val),
    }
}

// #[delete("")]
// pub async fn delete_appointment(
//     json: web::Json<AppointmentsModel>,
//     mut session: Session,
// ) -> impl Responder {
//     let session_core = match SessionCore::session_validator(&mut session).await {
//         Outcome::Error(err) => return HttpResponse::build(err.http_code).json(err.message),
//         Outcome::Failure(fail) => return HttpResponse::build(fail.http_code).json(fail.message),
//         Outcome::Success(session) => {
//             match session.role.is_backoffice_enabled {
//                 Some(true) => (),
//                 _ => {
//                     return HttpResponse::Unauthorized()
//                         .json(format!("User is not authorized to use this endpoint"))
//                 }
//             }

//             let appointment_permissions = match session.role.appointment_permissions {
//                 None => return HttpResponse::Unauthorized().json("User doesn't have permissions"),
//                 Some(val) => val,
//             };

//             if !UserRolesCore::has_permission(appointment_permissions, DELETE_PERMISSION) {
//                 return HttpResponse::Unauthorized().json("User doesn't have delete permissions");
//             }

//             session
//         }
//     };

//     match AppointmentsCore::delete_appointment(json.0, session_core).await {
//         Outcome::Error(err) => HttpResponse::build(err.http_code).json(err.message),
//         Outcome::Failure(fail) => HttpResponse::build(fail.http_code).json(fail.message),
//         Outcome::Success(val) => HttpResponse::Ok().json(val),
//     }
// }
