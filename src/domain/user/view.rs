use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::aggregate::User;

// The view for a UserView query, for a standard http application this should
// be designed to reflect the response dto that will be returned to a user.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserView {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<User> for UserView {
    fn from(user: User) -> Self {
        UserView {
            id: user.id.map(|id| id.to_string()),
            email: user.email,
            updated_at: Some(user.updated_at),
            ..Default::default()
        }
    }
}
