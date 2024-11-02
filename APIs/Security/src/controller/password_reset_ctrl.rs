use actix_web::{get, post, web, HttpResponse, Responder};
use models::entities::{password_reset::Model as PasswordResetModel, users::Model as UserModel};
use utils::Outcome;

use crate::core::PasswordResetCore;

#[get("")]
pub async fn insert_reset_token(query: web::Query<UserModel>) -> impl Responder {
    match PasswordResetCore::insert_reset_token(query.0).await {
        Err(err) => HttpResponse::build(err.http_code).json(err.message),
        Ok(val) => HttpResponse::Ok().json(val),
    }
}

#[post("/{reset_token}")]
pub async fn update_user_password(
    query: web::Path<String>,
    json: web::Json<UserModel>,
) -> impl Responder {
    match PasswordResetCore::update_user_password(
        PasswordResetModel {
            token: Some(query.to_string()),
            ..Default::default()
        },
        json.0,
    )
    .await
    {
        Outcome::Error(err) => HttpResponse::build(err.http_code).json(err.message),
        Outcome::Failure(fail) => HttpResponse::build(fail.http_code).json(fail.message),
        Outcome::Success(val) => HttpResponse::Ok().json(val),
    }
}
