mod config;
mod scanner;
mod routes;
mod utils;
mod exchanges;

use routes::create_router;
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = create_router();

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
let addr = SocketAddr::from(([0, 0, 0, 0], port.parse::<u16>().unwrap()));
    tracing::info!("Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
