use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use derive_more::{derive::Display, From};
use sea_orm::DbErr;
use serde_json::json;

#[derive(Debug, From, Display)]
pub enum APIError {
    NotFound(String),
    BadRequest(String),
    Server(String),
    #[from]
    Database(DbErr),
}

impl ResponseError for APIError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let mut response = HttpResponse::Ok();
        response.status(self.status_code());
        match self {
            APIError::BadRequest(msg) => response.json(json!({ "error": msg })),
            APIError::Server(msg) => response.json(json!({ "error": msg })),
            APIError::NotFound(msg) => response.json(json!({ "error": msg })),
            APIError::Database(err) => response.json(json!({ "error": err.to_string() })),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            APIError::BadRequest(_) => StatusCode::BAD_REQUEST,
            APIError::Server(_) => StatusCode::INTERNAL_SERVER_ERROR,
            APIError::NotFound(_) => StatusCode::NOT_FOUND,
            APIError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
