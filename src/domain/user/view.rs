use serde::{Deserialize, Serialize};

// The view for a UserView query, for a standard http application this should
// be designed to reflect the response dto that will be returned to a user.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserView {
    pub id: Option<String>,
    pub email: String,
}
