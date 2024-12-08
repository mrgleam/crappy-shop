use std::fmt::{Debug, Display, Formatter};

use bcrypt::BcryptError;
use sea_orm::DbErr;
use validator::{ValidationErrors, ValidationErrorsKind};

#[derive(Debug)]
pub enum UserError {
    NotFound,
    DatabaseError(String),
    HashingFailed(String),
    VerificationFailed(String),
    Other(String),
}

impl Display for UserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::NotFound => write!(f, "User Not Found"),
            UserError::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
            UserError::HashingFailed(msg) => write!(f, "Hashing Error: {}", msg),
            UserError::VerificationFailed(msg) => write!(f, "Verification Error: {}", msg),
            UserError::Other(msg) => write!(f, "Other Error: {}", msg),
        }
    }
}

impl From<DbErr> for UserError {
    fn from(err: DbErr) -> Self {
        match err {
            DbErr::RecordNotFound(_) => UserError::NotFound,
            DbErr::Exec(message) => UserError::DatabaseError(message.to_string()),
            DbErr::Query(message) => UserError::DatabaseError(message.to_string()),
            DbErr::Custom(message) => UserError::Other(message),
            DbErr::Conn(message) => {
                UserError::DatabaseError(format!("Connection Error: {}", message))
            }
            // Handle other variants as needed
            _ => UserError::Other("An unknown error occurred.".to_string()),
        }
    }
}

impl From<BcryptError> for UserError {
    fn from(err: BcryptError) -> Self {
        match err {
            BcryptError::InvalidHash(_) => {
                UserError::HashingFailed("Invalid hash format.".to_string())
            }
            BcryptError::InvalidPrefix(_) => {
                UserError::HashingFailed("Invalid hash prefix.".to_string())
            }
            BcryptError::CostNotAllowed(_) => {
                UserError::HashingFailed("Invalid number of cost.".to_string())
            }
            BcryptError::Io(e) => UserError::Other(format!("IO Error: {}", e)),
            _ => UserError::Other("An unknown bcrypt error occurred.".to_string()),
        }
    }
}

impl From<ValidationErrors> for UserError {
    fn from(errors: ValidationErrors) -> Self {
        let error_messages: Vec<String> = errors
            .errors()
            .into_iter()
            .map(|(field, error)| format!("{}: {}", field, extract_validation_messages(error)))
            .collect();
        UserError::VerificationFailed(error_messages.join(", "))
    }
}

// Recursive function to extract validation error messages
fn extract_validation_messages(errors_kind: &ValidationErrorsKind) -> String {
    match errors_kind {
        ValidationErrorsKind::Struct(errors) => {
            format!("Struct: {}", UserError::from(*errors.clone()))
        }
        ValidationErrorsKind::List(errors_map) => {
            let messages: Vec<String> = errors_map
                .iter()
                .map(|(index, errors)| format!("[{}]: {}", index, UserError::from(*errors.clone())))
                .collect();
            messages.join(", ")
        }
        ValidationErrorsKind::Field(errors) => {
            let messages: Vec<String> = errors
                .iter()
                .map(|error| {
                    format!(
                        "{}",
                        error.message.clone().unwrap_or("unknown error".into())
                    )
                })
                .collect();
            messages.join(", ")
        }
    }
}
