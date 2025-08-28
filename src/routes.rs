use axum::{Router, routing::get, extract::Query, response::Json};
use serde::Deserialize;
use crate::models::TriangularResult;
use crate::scanner;

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

async fn binance_handler(Query(params): Query<ScanParams>) -> Json<Vec<TriangularResult>> {
    let min_profit = params.min_profit.unwrap_or(0.3);
    let fee = params.fee_perc.unwrap_or(0.1);
    let prices = match crate::exchanges::binance::fetch_prices().await { Ok(v)=>v, Err(_)=>Vec::new() };
    Json(scanner::scan_triangles(prices, min_profit, fee))
}

async fn bybit_handler(Query(params): Query<ScanParams>) -> Json<Vec<TriangularResult>> {
    let min_profit = params.min_profit.unwrap_or(0.3);
    let fee = params.fee_perc.unwrap_or(0.1);
    let prices = match crate::exchanges::bybit::fetch_prices().await { Ok(v)=>v, Err(_)=>Vec::new() };
    Json(scanner::scan_triangles(prices, min_profit, fee))
}

async fn kucoin_handler(Query(params): Query<ScanParams>) -> Json<Vec<TriangularResult>> {
    let min_profit = params.min_profit.unwrap_or(0.3);
    let fee = params.fee_perc.unwrap_or(0.1);
    let prices = match crate::exchanges::kucoin::fetch_prices().await { Ok(v)=>v, Err(_)=>Vec::new() };
    Json(scanner::scan_triangles(prices, min_profit, fee))
}

async fn gateio_handler(Query(params): Query<ScanParams>) -> Json<Vec<TriangularResult>> {
    let min_profit = params.min_profit.unwrap_or(0.3);
    let fee = params.fee_perc.unwrap_or(0.1);
    let prices = match crate::exchanges::gateio::fetch_prices().await { Ok(v)=>v, Err(_)=>Vec::new() };
    Json(scanner::scan_triangles(prices, min_profit, fee))
}

async fn kraken_handler(Query(params): Query<ScanParams>) -> Json<Vec<TriangularResult>> {
    let min_profit = params.min_profit.unwrap_or(0.3);
    let fee = params.fee_perc.unwrap_or(0.1);
    let prices = match crate::exchanges::kraken::fetch_prices().await { Ok(v)=>v, Err(_)=>Vec::new() };
    Json(scanner::scan_triangles(prices, min_profit, fee))
}
