// src/main.rs
mod exchanges;
mod scanner;
mod utils;
mod models;
mod routes;

use axum::{Router, routing::get_service};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::cors::CorsLayer;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // initialize tracing/logging
    tracing_subscriber::fmt::init();

    // create the API router (routes.rs -> create_router)
    let api = routes::create_router();

    // build application:
    // - API is served under /api
    // - index.html served at "/"
    // - other static assets served under /static/*
    let app = Router::new()
        .nest("/api", api)
        .route_service(
            "/",
            // serve the UI index
            get_service(ServeFile::new("static/index.html")).handle_error(|err| async move {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to open static/index.html: {}", err),
                )
            }),
        )
        .nest_service("/static", ServeDir::new("static"))
        .layer(CorsLayer::permissive());

    // Render provides PORT env variable; default to 8080 locally
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().expect("invalid address");

    tracing::info!("Server running on http://{}", addr);

    // bind and run with axum::serve (Axum 0.7 pattern)
    let listener = TcpListener::bind(addr).await.expect("bind failed");
    axum::serve(listener, app).await.expect("server error");
}
