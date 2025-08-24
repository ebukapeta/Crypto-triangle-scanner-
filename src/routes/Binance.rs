use axum::{routing::get, extract::Query, response::Json, Router};
use serde::Deserialize;
use crate::{exchanges::binance, scanner};
use crate::models::TriangularResult;

#[derive(Deserialize)]
pub struct ScanParams {
    pub min_profit: Option<f64>, // in %
    pub fee_perc:   Option<f64>, // per leg, in %
}

pub fn binance_routes() -> Router {
    Router::new().route("/binance/triangular", get(handler))
}

async fn handler(Query(params): Query<ScanParams>) -> Json<Vec<TriangularResult>> {
    let min_profit = params.min_profit.unwrap_or(0.3);
    let fee_per_leg = params.fee_perc.unwrap_or(0.1);

    let prices = match binance::fetch_prices().await {
        Ok(v) => v,
        Err(_) => Vec::new(),
    };
    let results = scanner::scan_triangles(prices, min_profit, fee_per_leg);
    Json(results)
}
