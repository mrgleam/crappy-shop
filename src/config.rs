use crate::environment::get_var;

use do_notation::m;

pub struct Config {
    pub database: DatabaseConfig,
    pub http_server: HttpServerConfig,
    pub authentication: AuthenticationConfig,
}

pub struct DatabaseConfig {
    pub database_url: String,
}

impl DatabaseConfig {
    pub fn new(database_url: String) -> Self {
        DatabaseConfig { database_url }
    }
}

#[derive(Clone)]
pub struct HttpServerConfig {
    pub host: String,
    pub port: u16,
}

impl HttpServerConfig {
    pub fn new(host: String, port: u16) -> Self {
        HttpServerConfig { host, port }
    }
}

pub struct AuthenticationConfig {
    pub secret: String,
    pub expired_time: i64,
}

impl AuthenticationConfig {
    pub fn new(secret: String, expired_time: i64) -> Self {
        AuthenticationConfig {
            secret,
            expired_time,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        fn parse_port(port: String) -> Result<u16, String> {
            port.parse::<u16>()
                .map_err(|_| "Invalid port number".into())
        }

        fn parse_expired(expired: String) -> Result<i64, String> {
            expired
                .parse::<i64>()
                .map_err(|_| "Invalid expired number".into())
        }

        // get env vars
        dotenvy::dotenv().ok();

        m! {
          database_url <- get_var("DATABASE_URL");
          http_server_host <- get_var("HTTP_SERVER_HOST");
          http_server_port <- get_var("HTTP_SERVER_PORT").and_then(parse_port);
          authentication_secret <- get_var("AUTHENTICATION_SECRET");
          authentication_expired <- get_var("AUTHENTICATION_EXPIRED_TIME").and_then(parse_expired);
          return Config {
            database: DatabaseConfig::new(database_url),
            http_server: HttpServerConfig::new(http_server_host, http_server_port),
            authentication: AuthenticationConfig::new(authentication_secret, authentication_expired),
          };
        }
        .expect("Error getting env vars")
    }
}
