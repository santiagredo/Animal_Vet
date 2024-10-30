use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder};
use chrono::{Local, NaiveDateTime};

use tracing::error_span;
use utils::{Outcome, SESSION_ID, SESSION_UUID, USER_ID};

use crate::core::SessionCore;
use models::entities::sessions::Model as SessionModel;
use models::entities::users::Model;

#[post("/login")]
pub async fn login(json: web::Json<Model>, session: Session) -> impl Responder {
    let stored_session = match SessionCore::insert_session(json.0).await {
        Outcome::Error(err) => return HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => return HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(val) => val,
    };

    if let Err(err) = session.insert(SESSION_ID, stored_session.session_id) {
        error_span!("error - cookie", error = ?err);
        return HttpResponse::InternalServerError().finish();
    }

    if let Err(err) = session.insert(USER_ID, stored_session.user_id) {
        error_span!("error - cookie", error = ?err);
        return HttpResponse::InternalServerError().finish();
    }

    if let Err(err) = session.insert(SESSION_UUID, stored_session.session_uuid) {
        error_span!("error - cookie", error = ?err);
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

#[post("/logout")]
pub async fn logout(session: Session) -> impl Responder {
    let session_details = match SessionCore::session_details_extractor(&session) {
        Outcome::Error(err) => return HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => return HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(val) => val,
    };

    let current_date = Local::now();

    let session_model = SessionModel {
        session_id: session_details.session_id,
        user_id: Some(session_details.user_id),
        session_uuid: Some(session_details.session_uuid),
        is_enabled: Some(false),
        closing_date: Some(NaiveDateTime::new(
            current_date.date_naive(),
            current_date.time(),
        )),
        ..Default::default()
    };

    if let Err(err) = SessionCore::update_session(session_model).await {
        return HttpResponse::build(err.http_code).json(err.message);
    }

    session.remove(SESSION_ID);
    session.remove(USER_ID);
    session.remove(SESSION_UUID);

    session.purge();

    HttpResponse::Ok().finish()
}
