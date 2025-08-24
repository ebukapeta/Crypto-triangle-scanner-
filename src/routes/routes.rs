use axum::{response::Html, routing::get, Router};
use crate::exchanges::{binance, bybit, kucoin, gateio, kraken};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/binance", get(binance_prices))
        .route("/bybit", get(bybit_prices))
        .route("/kucoin", get(kucoin_prices))
        .route("/gateio", get(gateio_prices))
        .route("/kraken", get(kraken_prices))
}

async fn index() -> Html<&'static str> {
    Html("<h1>Triangular Arbitrage Scanner is running!</h1><p>Try /binance, /bybit, /kucoin, /gateio, or /kraken</p>")
}

async fn binance_prices() -> Html<String> {
    fetch_and_format(binance::fetch_prices().await, "Binance").await
}

async fn bybit_prices() -> Html<String> {
    fetch_and_format(bybit::fetch_prices().await, "Bybit").await
}

async fn kucoin_prices() -> Html<String> {
    fetch_and_format(kucoin::fetch_prices().await, "KuCoin").await
}

async fn gateio_prices() -> Html<String> {
    fetch_and_format(gateio::fetch_prices().await, "Gate.io").await
}

async fn kraken_prices() -> Html<String> {
    fetch_and_format(kraken::fetch_prices().await, "Kraken").await
}

async fn fetch_and_format(
    result: Result<Vec<(String, String, f64)>, reqwest::Error>,
    exchange_name: &str,
) -> Html<String> {
    match result {
        Ok(prices) if !prices.is_empty() => {
            let formatted: Vec<String> = prices
                .into_iter()
                .take(10)
                .map(|(base, quote, price)| format!("{} / {} : {}", base, quote, price))
                .collect();
            Html(format!(
                "<h2>{} Prices (sample):</h2><p>{}</p>",
                exchange_name,
                formatted.join("<br>")
            ))
        }
        Ok(_) => Html(format!("<p>No data yet for {}.</p>", exchange_name)),
        Err(e) => Html(format!("<p>Error fetching {} prices: {}</p>", exchange_name, e)),
    }
                     }
