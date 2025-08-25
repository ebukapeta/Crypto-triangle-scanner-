mod exchanges;
mod scanner;
mod utils;
mod models;
mod routes;

use axum::{Router, extract::Extension};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use tower_http::services::ServeDir;
use tower_http::cors::CorsLayer;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Shared app state if needed
    let shared_state = Arc::new(Mutex::new(()));

    // API router
    let api = routes::create_router();

    // Combine API routes and static file serving
    let app = Router::new()
        .nest("/api", api)
        .nest_service("/", ServeDir::new("static"))
        .layer(CorsLayer::permissive())
        .layer(Extension(shared_state));

    // Bind to Render PORT or default to 8080
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port)
        .parse()
        .expect("Invalid address");

    println!("Server running on {}", addr);

    // Start server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
