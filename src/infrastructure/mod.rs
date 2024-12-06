pub mod response;
pub mod user;

use std::sync::Arc;

use sea_orm::DatabaseConnection;

pub struct AppState {
    pub conn: Arc<DatabaseConnection>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            conn: Arc::clone(&self.conn),
        }
    }
}
