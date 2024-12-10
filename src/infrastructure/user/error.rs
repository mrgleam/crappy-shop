use crate::{domain::user::error::UserError, infrastructure::error::APIError};

impl From<UserError> for APIError {
    fn from(err: UserError) -> Self {
        match err {
            UserError::NotFound => APIError::NotFound("User Not Found".to_string()),
            UserError::DatabaseError(_) => APIError::Server("Internal Server Error".to_string()),
            UserError::HashingFailed(msg) => APIError::Server(msg),
            UserError::VerificationFailed(msg) => APIError::BadRequest(msg),
            UserError::TokenCreationFailed(msg) => APIError::Server(msg),
            UserError::Other(msg) => APIError::Server(msg),
        }
    }
}
