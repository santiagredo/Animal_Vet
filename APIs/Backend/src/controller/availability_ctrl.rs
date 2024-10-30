use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};
use models::entities::services::Model as ServiceModel;
use security::core::SessionCore;
use utils::Outcome;

use backoffice::core::AvailabilityCore;

#[get("")]
pub async fn select_availability(
    query: web::Query<ServiceModel>,
    mut session: Session,
) -> impl Responder {
    match SessionCore::session_validator(&mut session).await {
        Outcome::Error(err) => return HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => return HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(session) => session,
    };

    match AvailabilityCore::select_availability(query.0).await {
        Outcome::Error(err) => HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(val) => HttpResponse::Ok().json(val),
    }
}
