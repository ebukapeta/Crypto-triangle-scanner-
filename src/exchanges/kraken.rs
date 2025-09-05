use crate::models::PairPrice;
use reqwest::Error;
use serde_json::Value;
use std::collections::HashMap;

/// Normalize Kraken asset codes to standard symbols
fn normalize_symbol(s: &str) -> String {
    match s {
        "XBT" => "BTC".to_string(),
        "XETH" | "ETH" => "ETH".to_string(),
        "XXRP" | "XRP" => "XRP".to_string(),
        "XLTC" | "LTC" => "LTC".to_string(),
        "XBNB" | "BNB" => "BNB".to_string(),
        "USDT" => "USDT".to_string(),
        "USDC" => "USDC".to_string(),
        _ => s.trim_start_matches('X').trim_start_matches('Z').to_string(),
    }
}

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    let url = "https://api.kraken.com/0/public/Ticker?pair=all";
    let resp: Value = reqwest::get(url).await?.json().await?;
    let mut out = Vec::new();

    if let Some(result) = resp.get("result").and_then(|r| r.as_object()) {
        for (symbol, data) in result {
            if let Some(c) = data.get("c").and_then(|c| c.get(0)).and_then(|v| v.as_str()) {
                if let Ok(price) = c.parse::<f64>() {
                    // Kraken pairs like "XXBTZUSD" → base = BTC, quote = USD
                    let (base_raw, quote_raw) = symbol.split_at(symbol.len() / 2);
                    let base = normalize_symbol(base_raw);
                    let quote = normalize_symbol(quote_raw);

                    out.push(PairPrice {
                        base,
                        quote,
                        price,
                        is_spot: true,
                    });
                }
            }
        }
    }

    Ok(out)
                }
