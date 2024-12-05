use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use log;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub name: String,
}

impl DatabaseConfig {
    pub fn new(host: String, port: u16, username: String, password: String, name: String) -> Self {
        DatabaseConfig {
            host,
            port,
            username,
            password,
            name,
        }
    }
}

pub async fn new(config: DatabaseConfig) -> DatabaseConnection {
    let connection_string = format!(
        "postgres://{username}:{password}@{host}:{port}/{name}",
        host = config.host,
        port = config.port,
        username = config.username,
        password = urlencoding::encode(&config.password),
        name = config.name,
    );

    let mut opt = ConnectOptions::new(connection_string);
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
