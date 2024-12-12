use crate::domain::product::view::ProductView;
use crate::infrastructure::error::APIError;
use crate::infrastructure::response;
use crate::infrastructure::AppState;
use actix_web::{web, Responder};

use super::repository::ProductRepository;
pub async fn index(db: web::Data<AppState>) -> impl Responder {
    let products: Vec<ProductView> = ProductRepository::new(db.conn.clone())
        .find_all()
        .await
        .iter()
        .map(|product| ProductView {
            id: Some(product.id.to_string()),
            name: product.name.clone(),
            price: product.price,
            description: product.description.clone(),
            created_at: Some(product.created_at.to_utc()),
            updated_at: Some(product.updated_at.to_utc()),
        })
        .collect();
    response::Default::new(products).json()
}

pub async fn get_by_id(
    path: web::Path<i32>,
    db: web::Data<AppState>,
) -> Result<impl Responder, APIError> {
    let product_id = path.into_inner();

    let product = ProductRepository::new(db.conn.clone())
        .find_by_id(product_id)
        .await
        .map(|product| ProductView {
            id: Some(product.id.to_string()),
            name: product.name.clone(),
            price: product.price,
            description: product.description.clone(),
            created_at: Some(product.created_at.to_utc()),
            updated_at: Some(product.updated_at.to_utc()),
        })?;

    // Ok(web::Json(response::Default::new(product)))

    Ok(response::Default::new(product).json())
}
