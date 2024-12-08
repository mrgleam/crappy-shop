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
pub struct Created {}

impl Created {
    pub fn into() -> HttpResponse {
        HttpResponse::Created().into()
    }
}
