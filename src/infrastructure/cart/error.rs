use crate::{domain::cart::error::CartError, infrastructure::error::APIError};

impl From<CartError> for APIError {
    fn from(err: CartError) -> Self {
        match err {
            CartError::NotFound => APIError::NotFound("User Not Found".to_string()),
            CartError::DatabaseError(msg) => APIError::Server(msg),
            CartError::Other(msg) => APIError::Server(msg),
        }
    }
}
