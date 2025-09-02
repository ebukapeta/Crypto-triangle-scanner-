// src/main.rs
mod exchanges;
mod scanner;
mod utils;
mod models;
mod routes;

use axum::Router;
use axum::routing::get_service;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::cors::CorsLayer;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // init logging
    tracing_subscriber::fmt::init();

    // build API router (defined in src/routes.rs)
    let api = routes::create_router();

    // application: API under /api, UI served at /, other static assets under /static
    let app = Router::new()
        .nest("/api", api)
        // ensure "/" returns index.html (give helpful error on failure)
        .route_service(
            "/",
            get_service(ServeFile::new("static/index.html")).handle_error(
                |_err| async move {
                    (
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        "Could not open static/index.html".to_string(),
                    )
                },
            ),
        )
        // serve all other static files (css/js) at /static/*
        .nest_service("/static", ServeDir::new("static"))
        .layer(CorsLayer::permissive());

    // bind to Render PORT or default 8080
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port)
        .parse()
        .expect("invalid address");

    tracing::info!("Server starting on http://{}", addr);

    // create listener and run the app (axum v0.7 pattern)
    let listener = TcpListener::bind(addr).await.expect("bind failed");
    axum::serve(listener, app).await.unwrap();
        }
