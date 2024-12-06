use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct UserError(String);

impl From<&str> for UserError {
    fn from(msg: &str) -> Self {
        Self(msg.to_string())
    }
}

impl Display for UserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for UserError {}
