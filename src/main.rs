mod exchanges;
mod scanner;
mod utils;
mod models;
mod routes;

use axum::Router;
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tower_http::cors::CorsLayer;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // API routes (JSON)
    let api = routes::create_router();

    // App = API + Static UI
    let app = Router::new()
        .merge(api) // /binance/triangular etc
        .nest_service("/", ServeDir::new("static")) // serve / -> static/index.html
        .layer(CorsLayer::permissive());

    // Render binds via PORT env var
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().expect("invalid addr");

    let listener = TcpListener::bind(addr).await.expect("bind failed");
    println!("Listening on {}", port);

    axum::serve(listener, app).await.unwrap();
}
