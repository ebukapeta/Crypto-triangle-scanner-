use axum::{routing::get, Json, Router};
use serde_json::json;
use crate::scanner::Scanner;
use tracing::info;

pub fn create_router() -> Router {
    Router::new()
        .route("/scan/:exchange", get(scan_exchange))
}

/// Handles GET /scan/:exchange
async fn scan_exchange(axum::extract::Path(exchange): axum::extract::Path<String>) -> Json<serde_json::Value> {
    info!("Received scan request for {exchange}");
    let scanner = Scanner::new();

    match scanner.scan(&exchange).await {
        Ok(results) => Json(json!({
            "status": "success",
            "exchange": exchange,
            "results": results
        })),
        Err(err) => Json(json!({
            "status": "error",
            "exchange": exchange,
            "message": err
        })),
    }
}
