use std::fmt::Debug;

use sea_orm::DbErr;

#[derive(Debug)]
pub enum ProductError {
    NotFound,
    DatabaseError(String),
    Other(String),
}

impl From<DbErr> for ProductError {
    fn from(err: DbErr) -> Self {
        match err {
            DbErr::RecordNotFound(_) => ProductError::NotFound,
            DbErr::Exec(message) => ProductError::DatabaseError(message.to_string()),
            DbErr::Query(message) => ProductError::DatabaseError(message.to_string()),
            DbErr::Custom(message) => ProductError::Other(message),
            DbErr::Conn(message) => {
                ProductError::DatabaseError(format!("Connection Error: {}", message))
            }
            // Handle other variants as needed
            _ => ProductError::Other("An unknown error occurred.".to_string()),
        }
    }
}
