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
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
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
            created_at: Some(value.created_at.to_utc()),
            updated_at: Some(value.updated_at.to_utc()),
            token: None,
        }
    }
}

impl Into<user::ActiveModel> for User {
    fn into(self) -> user::ActiveModel {
        user::ActiveModel {
            id: self
                .id
                .map(sea_orm::ActiveValue::Set)
                .unwrap_or(sea_orm::ActiveValue::NotSet),
            email: sea_orm::ActiveValue::Set(self.email),
            password: sea_orm::ActiveValue::Set(self.password),
            created_at: self
                .created_at
                .map(|date| sea_orm::ActiveValue::Set(date.into()))
                .unwrap_or(sea_orm::ActiveValue::NotSet),
            updated_at: self
                .updated_at
                .map(|date| sea_orm::ActiveValue::Set(date.into()))
                .unwrap_or(sea_orm::ActiveValue::NotSet),
        }
    }
}
