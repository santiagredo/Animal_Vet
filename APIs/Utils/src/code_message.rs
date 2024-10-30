use actix_web::http::StatusCode;

pub struct CodeMessage {
    pub http_code: StatusCode,
    pub message: String
}