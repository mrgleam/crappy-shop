use std::fmt::Debug;

use sea_orm::DbErr;

#[derive(Debug)]
pub enum CartError {
    NotFound,
    DatabaseError(String),
    Other(String),
}

impl From<DbErr> for CartError {
    fn from(err: DbErr) -> Self {
        match err {
            DbErr::RecordNotFound(_) => CartError::NotFound,
            DbErr::Exec(message) => CartError::DatabaseError(message.to_string()),
            DbErr::Query(message) => CartError::DatabaseError(message.to_string()),
            DbErr::Custom(message) => CartError::Other(message),
            DbErr::Conn(message) => {
                CartError::DatabaseError(format!("Connection Error: {}", message))
            }
            // Handle other variants as needed
            _ => CartError::Other("An unknown error occurred.".to_string()),
        }
    }
}
