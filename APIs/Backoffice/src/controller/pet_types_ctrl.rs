use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};
use models::entities::pet_types::Model;
use security::core::SessionCore;
use utils::Outcome;

use crate::core::PetTypesCore;

#[get("")]
pub async fn select_pet_types(
    query: web::Query<Model>,
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

            session
        }
    };

    match PetTypesCore::select_pet_types(query.0).await {
        Outcome::Error(err) => HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(val) => HttpResponse::Ok().json(val),
    }
}
