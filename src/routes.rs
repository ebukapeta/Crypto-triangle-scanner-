use axum::{routing::get, Router};
use crate::handlers::{
    index_handler,
    binance_handler,
    kucoin_handler,
    bybit_handler,
    okx_handler,
};

pub fn create_router() -> Router {
    // API routes for different exchanges
    let api_routes = Router::new()
        .route("/binance/triangular", get(binance_handler))
        .route("/kucoin/triangular", get(kucoin_handler))
        .route("/bybit/triangular", get(bybit_handler))
        .route("/okx/triangular", get(okx_handler));

    // Main router with UI at root and API under /api
    Router::new()
        .route("/", get(index_handler))
        .nest("/api", api_routes)
}
