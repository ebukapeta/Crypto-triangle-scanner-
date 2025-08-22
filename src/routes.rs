use axum::{
    extract::Query,
    response::Html,
    routing::get,
    Json, Router,
};
use serde::Serialize;

use crate::config::ScanParams;
use crate::scanner::find_triangles;
use crate::exchanges::*;
use crate::utils::percent_string;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/opportunities", get(opps_handler))
        .route("/style.css", get(|| async { axum::response::Css("".to_string()) }))
        .route("/script.js", get(|| async { axum::response::Html("".to_string()) }))
        .fallback(get(index))
}

async fn index() -> Html<&'static str> {
    include_str!("../static/index.html").into()
}

#[derive(Serialize)]
struct OpportunityResponse {
    path: Vec<String>,
    profit_pct: f64,
}

async fn opps_handler(Query(params): Query<ScanParams>) -> Json<Vec<OpportunityResponse>> {
    let pairs = match params.exchange.as_str() {
        "binance" => binance::fetch_prices().await.unwrap_or_default(),
        "bybit" => bybit::fetch_prices().await.unwrap_or_default(),
        "kucoin" => kucoin::fetch_prices().await.unwrap_or_default(),
        "gateio" => gateio::fetch_prices().await.unwrap_or_default(),
        "kraken" => kraken::fetch_prices().await.unwrap_or_default(),
        _ => Vec::new(),
    };

    let opps = find_triangles(&pairs, params.min_profit / 100.0);

    Json(
        opps
            .into_iter()
            .map(|o| OpportunityResponse {
                path: o.path,
                profit_pct: o.profit * 100.0,
            })
            .collect(),
    )
}