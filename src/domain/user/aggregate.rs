use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct User {
    pub id: Option<i32>,
    pub email: String,
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
