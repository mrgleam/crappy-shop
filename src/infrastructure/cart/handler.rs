use actix_web::{web, Responder, ResponseError};

use crate::{
    domain::cart::error::CartError,
    infrastructure::{error::APIError, AppState},
};

use super::repository::CartRepository;

pub async fn add_item(db: web::Data<AppState>) -> impl Responder {
    let _ = CartRepository::new(db.conn.clone());
    APIError::from(CartError::DatabaseError("Server Error".to_string())).error_response()
}
