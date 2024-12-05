use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use log;

use crate::config::DatabaseConfig;

pub async fn new(config: DatabaseConfig) -> DatabaseConnection {
    let mut opt = ConnectOptions::new(config.database_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("public"); // Setting default PostgreSQL schema

    Database::connect(opt)
        .await
        .expect("Cannot connect to database")
}
