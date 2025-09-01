mod exchanges;
mod scanner;
mod utils;
mod models;
mod routes;

use axum::{Router, routing::get};
use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::cors::CorsLayer;
use axum::routing::get_service;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // API routes (they are nested under /api)
    let api = routes::create_router();

    // Serve index.html at "/" and serve other static files from "static"
    let app = Router::new()
        .nest("/api", api)
        // ensure "/" returns the UI
        .route_service("/", get_service(ServeFile::new("static/index.html")).handle_error(
            |_err| async move { (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Could not open index.html".to_string()) }
        ))
        // serve the rest of static assets (css, js) under root
        .nest_service("/static", ServeDir::new("static"))
        .layer(CorsLayer::permissive());

    // Render sets PORT env var; default to 8080 for local
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().expect("invalid address");

    println!("Server running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
