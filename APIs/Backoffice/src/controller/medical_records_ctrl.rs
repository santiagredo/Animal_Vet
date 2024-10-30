use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder};
use models::entities::medical_records::Model as MedicalRecordsModel;
use security::core::{SessionCore, UserRolesCore};
use utils::{Outcome, CREATE_PERMISSION, READ_PERMISSION};

use crate::core::MedicalRecordsCore;

#[post("")]
pub async fn insert_medical_record(
    json: web::Json<MedicalRecordsModel>,
    mut session: Session,
) -> impl Responder {
    let session_core = match SessionCore::session_validator(&mut session).await {
        Outcome::Error(err) => return HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => return HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(session) => {
            match session.role.is_backoffice_enabled {
                Some(true) => (),
                _ => {
                    return HttpResponse::Unauthorized()
                        .json(format!("User is not authorized to use this endpoint"))
                }
            }

            let pet_permissions = match session.role.pet_permissions {
                None => return HttpResponse::Unauthorized().json("User doesn't have permissions"),
                Some(val) => val,
            };

            if !UserRolesCore::has_permission(pet_permissions, CREATE_PERMISSION) {
                return HttpResponse::Unauthorized().json("User doesn't have create permissions");
            }

            session
        }
    };

    match MedicalRecordsCore::insert_medical_record(json.0, session_core).await {
        Outcome::Error(err) => HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(val) => HttpResponse::Ok().json(val),
    }
}

#[get("")]
pub async fn select_medical_records(
    query: web::Query<MedicalRecordsModel>,
    mut session: Session,
) -> impl Responder {
    match SessionCore::session_validator(&mut session).await {
        Outcome::Error(err) => return HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => return HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(session) => {
            match session.role.is_backoffice_enabled {
                Some(true) => (),
                _ => {
                    return HttpResponse::Unauthorized()
                        .json(format!("User is not authorized to use this endpoint"))
                }
            }

            let pet_permissions = match session.role.pet_permissions {
                None => return HttpResponse::Unauthorized().json("User doesn't have permissions"),
                Some(val) => val,
            };

            if !UserRolesCore::has_permission(pet_permissions, READ_PERMISSION) {
                return HttpResponse::Unauthorized().json("User doesn't have create permissions");
            }

            session
        }
    };

    match MedicalRecordsCore::select_medical_records(query.0).await {
        Outcome::Error(err) => HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(val) => HttpResponse::Ok().json(val),
    }
}
