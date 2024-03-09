use std::env;

use dotenvy::dotenv;

pub struct Config {
    database_url: String,
}

impl Config {
    pub fn load() -> Config {
        dotenv().ok();
        Config {
            database_url: env::var("DATABASE_URL").unwrap(),
        }
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }
}
