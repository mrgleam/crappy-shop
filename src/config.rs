use crate::environment::get_var;

use do_notation::m;

use serde::Deserialize;

pub struct Config {
    pub database: DatabaseConfig,
    pub http_server: HttpServerConfig,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    pub database_url: String,
}

impl DatabaseConfig {
    pub fn new(database_url: String) -> Self {
        DatabaseConfig { database_url }
    }
}

pub struct HttpServerConfig {
    pub host: String,
    pub port: u16,
}

impl HttpServerConfig {
    pub fn new(host: String, port: u16) -> Self {
        HttpServerConfig { host, port }
    }
}

impl Config {
    pub fn load() -> Self {
        fn parse_port(port: String) -> Result<u16, String> {
            port.parse::<u16>()
                .map_err(|_| "Invalid port number".into())
        }
        // get env vars
        dotenvy::dotenv().ok();

        m! {
          database_url <- get_var("DATABASE_URL");
          http_server_host <- get_var("HTTP_SERVER_HOST");
          http_server_port <- get_var("HTTP_SERVER_PORT").and_then(parse_port);
          return Config {
            database: DatabaseConfig::new(database_url),
            http_server: HttpServerConfig::new(http_server_host, http_server_port),
          };
        }
        .expect("Error getting env vars")
    }
}
