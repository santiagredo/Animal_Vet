use crate::core::UsersCore;
use actix_session::Session;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use models::entities::users::Model;
use security::core::SessionCore;
use utils::Outcome;

#[post("")]
pub async fn insert_user(json: web::Json<Model>) -> impl Responder {
    match UsersCore::insert_user(json.0).await {
        Outcome::Error(err) => HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(val) => HttpResponse::Ok().json(val),
    }
}

#[get("")]
pub async fn select_user(mut session: Session) -> impl Responder {
    let session = match SessionCore::session_validator(&mut session).await {
        Outcome::Error(err) => return HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => return HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(session) => session,
    };

    match UsersCore::select_user(Model {
        user_id: session.user.user_id,
        ..Default::default()
    })
    .await
    {
        Outcome::Error(err) => HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(val) => HttpResponse::Ok().json(val),
    }
}

#[patch("")]
pub async fn update_user(json: web::Json<Model>, mut session: Session) -> impl Responder {
    let session_core = match SessionCore::session_validator(&mut session).await {
        Outcome::Error(err) => return HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => return HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(session) => session,
    };

    match UsersCore::update_user(json.0, session_core).await {
        Outcome::Error(err) => HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(val) => HttpResponse::Ok().json(val),
    }
}

#[delete("")]
pub async fn delete_user(mut session: Session) -> impl Responder {
    let session_core = match SessionCore::session_validator(&mut session).await {
        Outcome::Error(err) => return HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => return HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(session) => session,
    };

    let user_model = Model {
        user_id: session_core.user.user_id,
        ..Default::default()
    };

    match UsersCore::delete_user(user_model).await {
        Outcome::Error(err) => HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(val) => HttpResponse::Ok().json(val),
    }
}
