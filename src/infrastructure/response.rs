use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Default<T: Serialize> {
    data: T,
}

impl<T: Serialize> Default<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
    pub fn json(&self) -> HttpResponse {
        HttpResponse::Ok().json(&self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: String) -> Self {
        Self { message }
    }
    pub fn json(&self) -> HttpResponse {
        HttpResponse::InternalServerError().json(&self)
    }
}