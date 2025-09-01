use axum::{routing::get, Router, extract::Query, Json};
use serde::Deserialize;
use crate::models::TriangularResult;

#[derive(Deserialize)]
pub struct ScanParams {
    pub min_profit: Option<f64>, // percent before fees
    pub fee_perc: Option<f64>,   // per-leg percent
}

pub fn create_router() -> Router {
    let api = Router::new()
        .route("/binance/triangular", get(binance_handler))
        .route("/bybit/triangular", get(bybit_handler))
        .route("/kucoin/triangular", get(kucoin_handler))
        .route("/gateio/triangular", get(gateio_handler))
        .route("/kraken/triangular", get(kraken_handler));

    api
}

async fn binance_handler(Query(params): Query<ScanParams>) -> Json<Vec<TriangularResult>> {
    let min_profit = params.min_profit.unwrap_or(0.3);
    let fee = params.fee_perc.unwrap_or(0.10);
    let result = crate::exchanges::binance::fetch_prices().await;
    match result {
        Ok(prices) => Json(crate::scanner::scan_triangles(&prices, min_profit, fee)),
        Err(_) => Json(Vec::new()),
    }
}

async fn bybit_handler(Query(params): Query<ScanParams>) -> Json<Vec<TriangularResult>> {
    let min_profit = params.min_profit.unwrap_or(0.3);
    let fee = params.fee_perc.unwrap_or(0.10);
    let result = crate::exchanges::bybit::fetch_prices().await;
    match result {
        Ok(prices) => Json(crate::scanner::scan_triangles(&prices, min_profit, fee)),
        Err(_) => Json(Vec::new()),
    }
}

async fn kucoin_handler(Query(params): Query<ScanParams>) -> Json<Vec<TriangularResult>> {
    let min_profit = params.min_profit.unwrap_or(0.3);
    let fee = params.fee_perc.unwrap_or(0.10);
    let result = crate::exchanges::kucoin::fetch_prices().await;
    match result {
        Ok(prices) => Json(crate::scanner::scan_triangles(&prices, min_profit, fee)),
        Err(_) => Json(Vec::new()),
    }
}

async fn gateio_handler(Query(params): Query<ScanParams>) -> Json<Vec<TriangularResult>> {
    let min_profit = params.min_profit.unwrap_or(0.3);
    let fee = params.fee_perc.unwrap_or(0.10);
    let result = crate::exchanges::gateio::fetch_prices().await;
    match result {
        Ok(prices) => Json(crate::scanner::scan_triangles(&prices, min_profit, fee)),
        Err(_) => Json(Vec::new()),
    }
}

async fn kraken_handler(Query(params): Query<ScanParams>) -> Json<Vec<TriangularResult>> {
    let min_profit = params.min_profit.unwrap_or(0.3);
    let fee = params.fee_perc.unwrap_or(0.10);
    let result = crate::exchanges::kraken::fetch_prices().await;
    match result {
        Ok(prices) => Json(crate::scanner::scan_triangles(&prices, min_profit, fee)),
        Err(_) => Json(Vec::new()),
    }
        }
