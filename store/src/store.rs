/// Central database connection wrapper.
///
/// Holds a single `PgConnection` used by all model query methods.
use crate::config::Config;
use diesel::prelude::*;

/// Thread-local database store.
///
/// Wraps a Diesel `PgConnection` and provides access to all table operations
/// through the model modules.
pub struct Store {
    /// Active PostgreSQL connection.
    pub conn: PgConnection,
}

impl Store {
    /// Creates a new `Store` by establishing a PostgreSQL connection.
    ///
    /// # Panics
    ///
    /// Panics if `DATABASE_URL` is not set or the connection cannot be established.
    pub fn new() -> Result<Self, ConnectionError> {
        let config = Config::default();
        let conn = PgConnection::establish(&config.db_url)?;

        Ok(Self { conn })
    }
}
