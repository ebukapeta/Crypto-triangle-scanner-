mod routes;
mod scanner;
mod models;
mod utils;
mod exchanges;

use axum::{Router, routing::get_service};
use std::net::SocketAddr;
use tower_http::{services::{ServeDir, ServeFile}, cors::CorsLayer};

#[tokio::main]
async fn main() {
    // API mounted under /api
    let api = routes::create_router();

    // Serve the static UI from / (root) and ensure `/` returns index.html
    let static_dir = ServeDir::new("static");
    let app = Router::new()
        .nest("/api", api)
        .route_service("/", get_service(ServeFile::new("static/index.html")))
        .nest_service("/", static_dir)
        .layer(CorsLayer::permissive());

    // Bind to Render's PORT or default 3000
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();

    println!("Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    }
