mod exchanges;
mod scanner;
mod utils;
mod models;
mod routes;

use axum::{Router, extract::Extension};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tower_http::services::ServeDir;
use tower_http::cors::CorsLayer;
use tracing_subscriber;
use axum::serve;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Shared app state
    let shared_state = Arc::new(Mutex::new(()));

    // API router
    let api = routes::create_router();

    // Combine API and static routes
    let app = Router::new()
        .nest("/api", api)
        .nest_service("/", ServeDir::new("static"))
        .layer(CorsLayer::permissive())
        .layer(Extension(shared_state));

    // Get port for Render or default 8080
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port)
        .parse()
        .expect("Invalid address");

    println!("Server running on {}", addr);

    // Bind listener and serve app
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");
    serve(listener, app).await.unwrap();
}
