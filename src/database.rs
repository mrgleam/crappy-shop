use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use log;

pub async fn new() -> DatabaseConnection {
        let mut opt = ConnectOptions::new("postgres://postgres:mysecretpassword@localhost:5432/crappy");
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info)
            .set_schema_search_path("public"); // Setting default PostgreSQL schema
        
        Database::connect(opt).await.expect("Cannot connect to database")
}
