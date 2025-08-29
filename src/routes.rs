use axum::{Router, routing::get, extract::Query, response::Json};
use serde::Deserialize;
use crate::models::TriangularResult;
use crate::scanner;

#[derive(Deserialize)]
pub struct ScanParams {
    pub min_profit: Option<f64>,
    pub fee_perc: Option<f64>,
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
    scan_exchange(crate::exchanges::binance::fetch_prices().await, params)
}

async fn bybit_handler(Query(params): Query<ScanParams>) -> Json<Vec<TriangularResult>> {
    scan_exchange(crate::exchanges::bybit::fetch_prices().await, params)
}

async fn kucoin_handler(Query(params): Query<ScanParams>) -> Json<Vec<TriangularResult>> {
    scan_exchange(crate::exchanges::kucoin::fetch_prices().await, params)
}

async fn gateio_handler(Query(params): Query<ScanParams>) -> Json<Vec<TriangularResult>> {
    scan_exchange(crate::exchanges::gateio::fetch_prices().await, params)
}

async fn kraken_handler(Query(params): Query<ScanParams>) -> Json<Vec<TriangularResult>> {
    scan_exchange(crate::exchanges::kraken::fetch_prices().await, params)
}

fn scan_exchange(result: Result<Vec<crate::models::PairPrice>, reqwest::Error>, params: ScanParams) -> Json<Vec<TriangularResult>> {
    let min_profit = params.min_profit.unwrap_or(0.3);
    let fee = params.fee_perc.unwrap_or(0.1);
    match result {
        Ok(prices) => Json(scanner::scan_triangles(&prices, min_profit, fee)),
        Err(_) => Json(vec![]),
    }
    }
