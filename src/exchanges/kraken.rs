use crate::models::PairPrice;
use reqwest::Error;
use serde_json::Value;
use std::collections::HashMap;

/// Normalize Kraken asset codes to standard symbols
fn normalize_symbol(s: &str) -> String {
    match s {
        "XBT" => "BTC".to_string(),
        "ETH" | "XETH" => "ETH".to_string(),
        "XXRP" | "XRP" => "XRP".to_string(),
        "XLTC" | "LTC" => "LTC".to_string(),
        "XBNB" | "BNB" => "BNB".to_string(),
        "USDT" => "USDT".to_string(),
        "USDC" => "USDC".to_string(),
        "EUR" => "EUR".to_string(),
        "USD" | "ZUSD" => "USD".to_string(),
        _ => s.trim_start_matches('X').trim_start_matches('Z').to_string(),
    }
}

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    // Step 1: Get asset pair mappings
    let url_pairs = "https://api.kraken.com/0/public/AssetPairs";
    let resp_pairs: Value = reqwest::get(url_pairs).await?.json().await?;
    let mut pair_map = HashMap::new();

    if let Some(result) = resp_pairs.get("result").and_then(|r| r.as_object()) {
        for (pair_code, details) in result {
            if let (Some(base), Some(quote)) = (
                details.get("base").and_then(|v| v.as_str()),
                details.get("quote").and_then(|v| v.as_str()),
            ) {
                pair_map.insert(
                    pair_code.clone(),
                    (normalize_symbol(base), normalize_symbol(quote)),
                );
            }
        }
    }

    // Step 2: Get ticker prices
    let url_ticker = "https://api.kraken.com/0/public/Ticker?pair=all";
    let resp_ticker: Value = reqwest::get(url_ticker).await?.json().await?;
    let mut out = Vec::new();

    if let Some(result) = resp_ticker.get("result").and_then(|r| r.as_object()) {
        for (pair_code, data) in result {
            if let Some((base, quote)) = pair_map.get(pair_code) {
                if let Some(c) = data.get("c").and_then(|c| c.get(0)).and_then(|v| v.as_str()) {
                    if let Ok(price) = c.parse::<f64>() {
                        out.push(PairPrice {
                            base: base.clone(),
                            quote: quote.clone(),
                            price,
                            is_spot: true,
                        });
                    }
                }
            }
        }
    }

    Ok(out)
        }
