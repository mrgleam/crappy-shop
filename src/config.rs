use crate::{database::DatabaseConfig, environment::get_var};

use do_notation::m;

pub struct Config {
    pub database: DatabaseConfig,
}

impl Config {
    fn parse_port(port: String) -> Result<u16, String> {
        port.parse::<u16>()
            .map_err(|_| "Invalid port number".into())
    }
    pub fn new() -> Self {
        // get env vars
        dotenvy::dotenv().ok();

        let database = m! {
          host <- get_var("DATABASE_HOST");
          port <- get_var("DATABASE_PORT").and_then(Self::parse_port);
          username <- get_var("DATABASE_USERNAME");
          password <- get_var("DATABASE_PASSWORD");
          name <- get_var("DATABASE_NAME");

          return DatabaseConfig::new(host, port, username, password, name);
        }
        .expect("Error getting env vars");

        Config { database }
    }
}
