use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum UserCommand {
    Create {
        email: String,
        password: String,
    },
    Update {
        id: i32,
        email: String,
        password: String,
    },
}

impl UserCommand {
    pub fn create(email: String, password: String) -> Self {
        UserCommand::Create { email, password }
    }

    pub fn update(id: i32, email: String, password: String) -> Self {
        UserCommand::Update {
            id,
            email,
            password,
        }
    }
}
