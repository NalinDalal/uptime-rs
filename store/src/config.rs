/// Application configuration loaded from environment variables.
///
/// Currently supports `DATABASE_URL` for PostgreSQL connectivity.
use std::env;

use dotenvy::dotenv;

/// Configuration settings for the store layer.
pub struct Config {
    /// PostgreSQL connection URL.
    pub db_url: String,
}

impl Default for Config {
    fn default() -> Self {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| panic!("Please provide the database_url environment variable"));
        Self { db_url }
    }
}
