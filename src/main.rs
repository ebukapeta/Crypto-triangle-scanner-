mod exchanges;
mod scanner;
mod utils;
mod models;
mod routes;

use axum::{Router, extract::Extension};
use std::{env, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tower_http::cors::CorsLayer;
use tower_http::cors::Any;
use tokio::sync::Mutex;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // shared app state if later needed
    let shared_state = Arc::new(Mutex::new(()));

    // build router
    let api = routes::create_router();
    let app = Router::new()
        .merge(api)
        .nest_service("/", ServeDir::new("static"))
        .layer(CorsLayer::permissive())
        .layer(Extension(shared_state));

    // bind to Render PORT or default 8080
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().expect("invalid addr");

    let listener = TcpListener::bind(addr).await.expect("bind failed");
    println!("Listening on {}", port);

    axum::serve(listener, app).await.unwrap();
                }
