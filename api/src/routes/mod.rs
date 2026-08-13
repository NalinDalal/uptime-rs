//! HTTP route handlers for the Uptime API.
//!
//! Organized into two sub-modules:
//!
//! - [`user`]: Authentication, webhooks, and maintenance endpoints.
//! - [`website`]: Website CRUD, incidents, and public status page endpoints.

pub mod user;
pub mod website;
