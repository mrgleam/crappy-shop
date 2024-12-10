pub mod error;
pub mod response;
pub mod user;

use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::config::AuthenticationConfig;

pub struct AppState {
    pub conn: Arc<DatabaseConnection>,
    pub authentication_config: Arc<AuthenticationConfig>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            conn: Arc::clone(&self.conn),
            authentication_config: Arc::clone(&self.authentication_config),
        }
    }
}
