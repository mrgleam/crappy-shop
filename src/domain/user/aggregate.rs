use chrono::{DateTime, Utc};
use entity::user;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Default)]
pub struct User {
    pub id: Option<i32>,
    #[validate(email(message = "Invalid email"))]
    pub email: String,
    #[validate(length(
        min = 8,
        max = 20,
        message = "Password must be between 8 and 20 characters"
    ))]
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub token: Option<String>,
}

impl User {
    pub fn new() -> Self {
        Self::default()
    }
}

impl From<user::Model> for User {
    fn from(value: user::Model) -> Self {
        User {
            id: Some(value.id),
            email: value.email,
            password: value.password,
            created_at: value.created_at.to_utc(),
            updated_at: value.updated_at.to_utc(),
            token: None,
        }
    }
}
