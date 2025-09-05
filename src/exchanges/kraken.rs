use crate::models::PairPrice;
use reqwest::Error;
use serde_json::Value;
use std::collections::HashMap;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    // Step 1: Get all asset pairs
    let url = "https://api.kraken.com/0/public/AssetPairs";
    let resp: Value = reqwest::get(url).await?.json().await?;

    // Safely extract result as a map
    let pairs_map = resp["result"].as_object();
    let mut out = Vec::new();

    if let Some(map) = pairs_map {
        for (pair_name, info) in map {
            // Only include spot, online, tradeable pairs
            let status = info.get("status").and_then(|v| v.as_str()).unwrap_or("");
            let spot = info.get("spot").and_then(|v| v.as_str()).unwrap_or("true"); // default true
            let base = info.get("base").and_then(|v| v.as_str()).unwrap_or("");
            let quote = info.get("quote").and_then(|v| v.as_str()).unwrap_or("");

            if status != "online" || spot != "true" {
                continue; // skip non-spot or offline
            }

            // Step 2: Fetch ticker for this pair
            let ticker_url = format!("https://api.kraken.com/0/public/Ticker?pair={}", pair_name);
            let ticker_resp: Value = reqwest::get(&ticker_url).await?.json().await?;

            if let Some(ticker_obj) = ticker_resp["result"].as_object() {
                if let Some(first_entry) = ticker_obj.values().next() {
                    if let Some(price_str) = first_entry["c"][0].as_str() {
                        if let Ok(price) = price_str.parse::<f64>() {
                            out.push(PairPrice {
                                base: base.to_string(),
                                quote: quote.to_string(),
                                price,
                                is_spot: true,
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(out)
                }
