use std::sync::{Arc, Mutex};

use poem::{EndpointExt, Route, Server, get, listener::TcpListener, post};
use routes::{
    user::{sign_in, sign_up},
    website::{create_website, get_website},
};
use store::store::Store;
pub mod auth_middleware;
pub mod request_inputs;
pub mod request_outputs;
pub mod routes;
#[tokio::main(flavor = "multi_thread")]
//1. Creating the routes
async fn main() -> Result<(), std::io::Error> {
    let s = Arc::new(Mutex::new(Store::new().unwrap()));
    let app = Route::new()
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_website))
        .at("/user/signup", post(sign_up))
        .at("/user/signin", post(sign_in))
        .data(s);
    // creates and runs the http server
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}
