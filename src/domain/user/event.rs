use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UserEvent {
    Created {
        id: i32,
        email: String,
        date: DateTime<Utc>,
    },
    Updated {
        id: i32,
        email: String,
        date: DateTime<Utc>,
    },
    LoggedIn {
        token: String,
    },
}
