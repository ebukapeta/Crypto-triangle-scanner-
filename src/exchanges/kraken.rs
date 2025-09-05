use crate::models::PairPrice;
use reqwest::Error;
use serde_json::Value;
use std::collections::HashMap;

/// Normalize Kraken asset codes to standard symbols
fn normalize_symbol(s: &str) -> String {
    match s {
        "XBT" => "BTC".to_string(),
        "ZUSD" | "USD" => "USD".to_string(),
        "ZEUR" | "EUR" => "EUR".to_string(),
        "ZGBP" | "GBP" => "GBP".to_string(),
        "ZJPY" | "JPY" => "JPY".to_string(),
        "XETH" | "ETH" => "ETH".to_string(),
        "XLTC" | "LTC" => "LTC".to_string(),
        "XXRP" | "XRP" => "XRP".to_string(),
        "USDT" => "USDT".to_string(),
        "USDC" => "USDC".to_string(),
        other => other.trim_start_matches('X').trim_start_matches('Z').to_string(),
    }
}

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    // Step 1: Fetch asset pair metadata
    let url_pairs = "https://api.kraken.com/0/public/AssetPairs";
    let resp_pairs: Value = reqwest::get(url_pairs).await?.json().await?;
    let mut pair_map = HashMap::new();

    if let Some(result) = resp_pairs.get("result").and_then(|r| r.as_object()) {
        for (pair_code, details) in result {
            // skip dark pool markets
            if pair_code.ends_with(".d") {
                continue;
            }

            // skip margin-only or derivative pairs
            let has_leverage = details.get("leverage_buy")
                .and_then(|v| v.as_array())
                .map(|arr| !arr.is_empty())
                .unwrap_or(false);

            if has_leverage {
                continue;
            }

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

    let pair_list: Vec<String> = pair_map.keys().cloned().collect();
    let mut out = Vec::new();

    // Step 2: Request tickers in batches of 50
    for chunk in pair_list.chunks(50) {
        let url_ticker = format!(
            "https://api.kraken.com/0/public/Ticker?pair={}",
            chunk.join(",")
        );
        let resp_ticker: Value = reqwest::get(&url_ticker).await?.json().await?;

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
    }

    println!("Kraken: loaded {} spot pairs", out.len());
    Ok(out)
            }
