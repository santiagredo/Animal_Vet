use actix_session::Session;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use models::entities::work_days::Model as WorkDayModel;
use security::core::{SessionCore, UserRolesCore};
use utils::{Outcome, CREATE_PERMISSION, DELETE_PERMISSION, READ_PERMISSION, UPDATE_PERMISSION};

use crate::core::WorkDaysCore;

#[post("")]
pub async fn insert_work_day(
    json: web::Json<WorkDayModel>,
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

            let work_day_permissions = match session.role.work_day_permissions {
                None => return HttpResponse::Unauthorized().json("User doesn't have permissions"),
                Some(val) => val,
            };

            if !UserRolesCore::has_permission(work_day_permissions, CREATE_PERMISSION) {
                return HttpResponse::Unauthorized().json("User doesn't have create permissions");
            }

            session
        }
    };

    match WorkDaysCore::insert_work_day(json.0, session_core).await {
        Outcome::Error(err) => HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(val) => HttpResponse::Ok().json(val),
    }
}

#[get("")]
pub async fn select_work_days(
    query: web::Query<WorkDayModel>,
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

            let work_day_permissions = match session.role.work_day_permissions {
                None => return HttpResponse::Unauthorized().json("User doesn't have permissions"),
                Some(val) => val,
            };

            if !UserRolesCore::has_permission(work_day_permissions, READ_PERMISSION) {
                return HttpResponse::Unauthorized().json("User doesn't have read permissions");
            }

            session
        }
    };

    match WorkDaysCore::select_work_days(query.0).await {
        Outcome::Error(err) => HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(val) => HttpResponse::Ok().json(val),
    }
}

#[patch("")]
pub async fn update_work_day(
    json: web::Json<WorkDayModel>,
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

            let work_day_permissions = match session.role.work_day_permissions {
                None => return HttpResponse::Unauthorized().json("User doesn't have permissions"),
                Some(val) => val,
            };

            if !UserRolesCore::has_permission(work_day_permissions, UPDATE_PERMISSION) {
                return HttpResponse::Unauthorized().json("User doesn't have update permissions");
            }

            session
        }
    };

    match WorkDaysCore::update_work_day(json.0, session_core).await {
        Outcome::Error(err) => HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(val) => HttpResponse::Ok().json(val),
    }
}

#[delete("")]
pub async fn delete_work_day(
    json: web::Json<WorkDayModel>,
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

            let work_day_permissions = match session.role.work_day_permissions {
                None => return HttpResponse::Unauthorized().json("User doesn't have permissions"),
                Some(val) => val,
            };

            if !UserRolesCore::has_permission(work_day_permissions, DELETE_PERMISSION) {
                return HttpResponse::Unauthorized().json("User doesn't have delete permissions");
            }

            session
        }
    };

    match WorkDaysCore::delete_work_day(json.0, session_core).await {
        Outcome::Error(err) => HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(val) => HttpResponse::Ok().json(val),
    }
}
