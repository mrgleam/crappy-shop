use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use serde_json::json;

#[derive(Debug)]
pub enum APIError {
    NotFound(String),
    BadRequest(String),
    Server(String),
}

impl std::fmt::Display for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            APIError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            APIError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            APIError::Server(msg) => write!(f, "Server Error: {}", msg),
        }
    }
}

impl ResponseError for APIError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let mut response = HttpResponse::Ok();
        response.status(self.status_code());
        match self {
            APIError::BadRequest(msg) => response.json(json!({ "error": msg })),
            APIError::Server(msg) => response.json(json!({ "error": msg })),
            APIError::NotFound(msg) => response.json(json!({ "error": msg })),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            APIError::BadRequest(_) => StatusCode::BAD_REQUEST,
            APIError::Server(_) => StatusCode::INTERNAL_SERVER_ERROR,
            APIError::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}
