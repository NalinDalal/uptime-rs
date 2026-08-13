//! Uptime API server entry point.
//!
//! Starts an HTTP server using Poem on port `3001` and registers all
//! application routes.

use std::sync::{Arc, Mutex};

use poem::{EndpointExt, Route, Server, get, listener::TcpListener, patch, post};
use routes::{
    user::{create_maintenance, get_maintenances, get_webhook, sign_in, sign_up, update_webhook},
    website::{
        create_website, get_incidents, get_public_maintenance, get_public_status_history, get_websites,
        get_website, get_public_status,
    },
};
use store::store::Store;
pub mod auth_middleware;
pub mod request_inputs;
pub mod request_outputs;
pub mod routes;

/// Starts the Uptime API server.
///
/// Listens on `0.0.0.0:3001` and shares a single `Store` instance across all
/// request handlers via `Arc<Mutex<Store>>`.
#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), std::io::Error> {
    let s = Arc::new(Mutex::new(Store::new().unwrap()));
    let app = Route::new()
        .at("/user/signup", post(sign_up))
        .at("/user/signin", post(sign_in))
        .at("/user/webhook", get(get_webhook))
        .at("/user/webhook", patch(update_webhook))
        .at("/website", post(create_website))
        .at("/websites", get(get_websites))
        .at("/status/:websiteId", get(get_website))
        .at("/incidents", get(get_incidents))
        .at("/maintenance", post(create_maintenance))
        .at("/maintenance", get(get_maintenances))
        .at("/public/status/:userId", get(get_public_status))
        .at("/public/status/:userId/history", get(get_public_status_history))
        .at("/public/maintenance/:userId", get(get_public_maintenance))
        .data(s);
    Server::new(TcpListener::bind("0.0.0.0:3001"))
        .name("uptime-api")
        .run(app)
        .await
}
