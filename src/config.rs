use crate::{database::DatabaseConfig, environment::get_var};

use do_notation::m;

pub struct Config {
    pub database: DatabaseConfig,
}

impl Config {
    pub fn new() -> Self {
        // get env vars
        dotenvy::dotenv().ok();

        let database = m! {
          database_url <- get_var("DATABASE_URL");
          return DatabaseConfig::new(database_url);
        }
        .expect("Error getting env vars");

        Config { database }
    }
}
