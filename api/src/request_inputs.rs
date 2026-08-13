//! Request body types for API endpoints.
//!
//! All types derive `Serialize` and `Deserialize` for JSON parsing.

use serde::{Deserialize, Serialize};

/// Request body for creating a new website monitor.
#[derive(Serialize, Deserialize)]
pub struct CreateWebsiteInput {
    /// Target URL to monitor.
    pub url: String,
}

/// Request body for user signup and signin.
#[derive(Serialize, Deserialize)]
pub struct CreateUserInput {
    pub username: String,
    pub password: String,
}

/// Request body for creating a maintenance window.
#[derive(Serialize, Deserialize)]
pub struct CreateMaintenanceInput {
    pub website_id: String,
    pub title: String,
    pub description: Option<String>,
    pub starts_at: String,
    pub ends_at: Option<String>,
}

/// Request body for updating a webhook URL.
#[derive(Serialize, Deserialize)]
pub struct UpdateWebhookInput {
    pub url: String,
}
