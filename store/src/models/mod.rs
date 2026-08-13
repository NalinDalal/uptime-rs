//! Database models and query implementations.
//!
//! Each module corresponds to a database table and provides Diesel-based
//! CRUD operations through [`Store`](crate::store::Store).
//!
//! - [`user`]: User accounts and webhook settings.
//! - [`website`]: Monitored websites and ticks.
//! - [`incident`]: Downtime incidents.
//! - [`maintenance`]: Scheduled maintenance windows.

pub mod user;
pub mod website;
pub mod incident;
pub mod maintenance;
