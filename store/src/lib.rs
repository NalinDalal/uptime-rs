//! Database store crate for the Uptime API.
//!
//! Provides a Diesel-based PostgreSQL connection wrapper (`Store`) and
//! model-level CRUD helpers for users, websites, ticks, incidents, and
//! maintenance windows.
//!
//! # Modules
//!
//! - [`config`]: Loads database configuration from environment variables.
//! - [`store`]: Central `Store` struct wrapping a `PgConnection`.
//! - [`schema`]: Auto-generated Diesel schema definitions.
//! - [`models`]: Table models and query implementations.

pub mod config;
pub mod models;
pub mod schema;
pub mod store;
