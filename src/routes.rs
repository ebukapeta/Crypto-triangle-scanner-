// src/routes.rs
use axum::{routing::get, Router, extract::Query, Json};
use serde::Deserialize;
use crate::models::{PairPrice, TriangularResult};
use crate::scanner::scan_triangles;

#[derive(Deserialize)]
pub struct ScanParams {
    pub min_profit: Option<f64>, // percent before fees
    pub fee_perc: Option<f64>,   // per-leg percent
}

pub fn create_router() -> Router {
    Router::new()
        .route("/binance/triangular", get(binance_handler))
        .route("/bybit/triangular", get(bybit_handler))
        .route("/kucoin/triangular", get(kucoin_handler))
        .route("/gateio/triangular", get(gateio_handler))
        .route("/kraken/triangular", get(kraken_handler))
}

// Shared handler to reduce duplication
async fn handle_scan<F, Fut>(params: ScanParams, fetch_fn: F) -> Json<Vec<TriangularResult>>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<Vec<PairPrice>, reqwest::Error>>,
{
    let min_profit = params.min_profit.unwrap_or(0.3);
    let fee = params.fee_perc.unwrap_or(0.10);

    match fetch_fn().await {
        Ok(prices) => {
            // Use only valid spot pairs
            let spot_pairs: Vec<PairPrice> = prices
                .into_iter()
                .filter(|p| p.is_spot) // assumes PairPrice has `is_spot: bool`
                .collect();

            Json(scan_triangles(&spot_pairs, min_profit, fee))
        }
        Err(_) => Json(Vec::new()),
    }
}

async fn binance_handler(Query(params): Query<ScanParams>) -> Json<Vec<TriangularResult>> {
    handle_scan(params, crate::exchanges::binance::fetch_prices).await
}

async fn bybit_handler(Query(params): Query<ScanParams>) -> Json<Vec<TriangularResult>> {
    handle_scan(params, crate::exchanges::bybit::fetch_prices).await
}

async fn kucoin_handler(Query(params): Query<ScanParams>) -> Json<Vec<TriangularResult>> {
    handle_scan(params, crate::exchanges::kucoin::fetch_prices).await
}

async fn gateio_handler(Query(params): Query<ScanParams>) -> Json<Vec<TriangularResult>> {
    handle_scan(params, crate::exchanges::gateio::fetch_prices).await
}

async fn kraken_handler(Query(params): Query<ScanParams>) -> Json<Vec<TriangularResult>> {
    handle_scan(params, crate::exchanges::kraken::fetch_prices).await
                }
