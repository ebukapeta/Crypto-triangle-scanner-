use axum::Router;
use tower_http::{services::ServeDir, cors::CorsLayer};

mod models;
mod utils;
mod scanner;
mod exchanges;
mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(routes::binance::binance_routes())
        .merge(routes::bybit::bybit_routes())
        .merge(routes::kucoin::kucoin_routes())
        .merge(routes::gateio::gateio_routes())
        .merge(routes::kraken::kraken_routes())
        .nest_service("/", ServeDir::new("static"))
        .layer(CorsLayer::permissive());

    let addr = "0.0.0.0:3000".parse().unwrap();
    println!("Server running at http://{addr}");
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
