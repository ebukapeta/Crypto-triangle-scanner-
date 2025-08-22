mod config;
mod scanner;
mod routes;
mod utils;
mod exchanges;

use routes::create_router;
use tokio::net::TcpListener;
use axum::serve;
use std::env;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = create_router();

    // Use Render-provided port or default to 8080 locally
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let listener = TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Server running at http://{}", addr);

    serve(listener, app).await.unwrap();
}
