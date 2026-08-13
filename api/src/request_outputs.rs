//! Response body types returned by API endpoints.
//!
//! All types derive `Serialize` for JSON responses.

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// A single website health check tick.
#[derive(Serialize, Deserialize, Clone)]
pub struct TickOutput {
    pub id: String,
    pub response_time_ms: i32,
    pub status: String,
    pub http_status: Option<i32>,
    pub created_at: NaiveDateTime,
}

/// A website with its most recent ticks included.
#[derive(Serialize, Deserialize, Clone)]
pub struct WebsiteWithTicksOutput {
    pub id: String,
    pub url: String,
    pub user_id: String,
    pub time_added: NaiveDateTime,
    pub component: Option<String>,
    pub ticks: Vec<TickOutput>,
}

/// Response body for creating a website.
#[derive(Serialize, Deserialize)]
pub struct CreateWebsiteOutput {
    pub id: String,
}

/// Response body for user creation.
#[derive(Serialize, Deserialize)]
pub struct CreateUserOutput {
    pub id: String,
}

/// Response body for successful signin.
#[derive(Serialize, Deserialize)]
pub struct SigninOutput {
    pub jwt: String,
}

/// Response body for getting a single website.
#[derive(Serialize, Deserialize)]
pub struct GetWebsiteOutput {
    pub url: String,
    pub id: String,
    pub user_id: String,
    pub ticks: Vec<TickOutput>,
}

/// Response body for listing a user's websites.
#[derive(Serialize, Deserialize)]
pub struct GetWebsitesOutput {
    pub websites: Vec<WebsiteWithTicksOutput>,
}

/// Summary of an incident for list responses.
#[derive(Serialize, Deserialize)]
pub struct IncidentOutput {
    pub id: String,
    pub website_url: String,
    pub started_at: NaiveDateTime,
    pub ended_at: Option<NaiveDateTime>,
    pub region_id: String,
}

/// Response body for listing incidents.
#[derive(Serialize, Deserialize)]
pub struct GetIncidentsOutput {
    pub incidents: Vec<IncidentOutput>,
}

/// Summary of a maintenance window.
#[derive(Serialize, Deserialize)]
pub struct MaintenanceOutput {
    pub id: String,
    pub website_url: String,
    pub title: String,
    pub description: String,
    pub starts_at: NaiveDateTime,
    pub ends_at: Option<NaiveDateTime>,
    pub status: String,
}

/// Response body for listing maintenance windows.
#[derive(Serialize, Deserialize)]
pub struct GetMaintenancesOutput {
    pub maintenances: Vec<MaintenanceOutput>,
}

/// A single entry in the public history timeline.
#[derive(Serialize, Deserialize, Clone)]
pub struct HistoryOutput {
    #[serde(rename = "type")]
    pub history_type: String,
    pub id: String,
    pub website_url: String,
    pub started_at: NaiveDateTime,
    pub ended_at: Option<NaiveDateTime>,
    pub title: Option<String>,
    pub status: Option<String>,
}

/// Response body for the public status history endpoint.
#[derive(Serialize, Deserialize)]
pub struct PublicHistoryOutput {
    pub history: Vec<HistoryOutput>,
}

/// Uptime percentages for a single website across multiple time windows.
#[derive(Serialize, Deserialize)]
pub struct WebsiteStatOutput {
    pub website_id: String,
    pub periods: StatPeriodsOutput,
}

/// Uptime percentages for a website.
#[derive(Serialize, Deserialize)]
pub struct StatPeriodsOutput {
    pub d1: Option<f64>,
    pub d7: Option<f64>,
    pub d30: Option<f64>,
}

/// A component group on the public status page.
#[derive(Serialize, Deserialize)]
pub struct ComponentOutput {
    pub name: String,
    pub websites: Vec<WebsiteWithTicksOutput>,
    pub stats: StatPeriodsOutput,
    pub status: String,
}

/// Full public status page payload.
#[derive(Serialize, Deserialize)]
pub struct PublicStatusOutput {
    pub components: Vec<ComponentOutput>,
    pub incidents: Vec<IncidentOutput>,
    pub maintenances: Vec<MaintenanceOutput>,
    pub websites: Vec<WebsiteWithTicksOutput>,
    pub stats: Vec<WebsiteStatOutput>,
}

/// Response body for fetching the user's webhook URL.
#[derive(Serialize, Deserialize)]
pub struct GetWebhookOutput {
    pub url: Option<String>,
}

/// Response body for updating the webhook URL.
#[derive(Serialize, Deserialize)]
pub struct UpdateWebhookOutput {
    pub ok: bool,
}
