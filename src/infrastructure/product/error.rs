use crate::{domain::product::error::ProductError, infrastructure::error::APIError};

impl From<ProductError> for APIError {
    fn from(err: ProductError) -> Self {
        match err {
            ProductError::NotFound => APIError::NotFound("Product Not Found".to_string()),
            ProductError::DatabaseError(msg) => APIError::Server(msg),
            ProductError::Other(msg) => APIError::Server(msg),
        }
    }
}
