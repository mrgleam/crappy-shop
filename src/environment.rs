use std::env;

pub fn get_var(key: &str) -> Result<String, String> {
    env::var(key).map_err(|_| format!("{} is not set in .env file", key))
}
