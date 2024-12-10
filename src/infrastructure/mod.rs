pub mod error;
pub mod product;
pub mod response;
pub mod user;

use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::config::AuthenticationConfig;

#[derive(Clone)]
pub struct AppState {
    pub conn: Arc<DatabaseConnection>,
    pub authentication_config: Arc<AuthenticationConfig>,
}
