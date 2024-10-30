use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};
use models::entities::services::Model as ServiceModel;
use security::core::{SessionCore, UserRolesCore};
use utils::{Outcome, READ_PERMISSION};

use crate::core::ServicesCore;

#[get("")]
pub async fn select_services(
    query: web::Query<ServiceModel>,
    mut session: Session,
) -> impl Responder {
    match SessionCore::session_validator(&mut session).await {
        Outcome::Error(err) => return HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => return HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(session) => {
            let service_permissions = match session.role.service_permissions {
                None => return HttpResponse::Unauthorized().json("User doesn't have permissions"),
                Some(val) => val,
            };

            if !UserRolesCore::has_permission(service_permissions, READ_PERMISSION) {
                return HttpResponse::Unauthorized().json("User doesn't have read permissions");
            }
        }
    };

    match ServicesCore::select_services(query.0).await {
        Outcome::Error(err) => HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(val) => HttpResponse::Ok().json(val),
    }
}
