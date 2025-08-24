use axum::{Router};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

mod models;
mod utils;
mod routes;
mod exchanges;

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .merge(routes::binance::binance_routes(shared_state.clone()))
        .merge(routes::bybit::bybit_routes(shared_state.clone()))
        .merge(routes::kucoin::kucoin_routes(shared_state.clone()))
        .merge(routes::gateio::gateio_routes(shared_state.clone()))
        .merge(routes::kraken::kraken_routes(shared_state.clone()))
        .nest_service("/", ServeDir::new("static"))
        .layer(CorsLayer::permissive());

    println!("Server running at http://127.0.0.1:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
            }
