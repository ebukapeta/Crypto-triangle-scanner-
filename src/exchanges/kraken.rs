use crate::models::PairPrice;
use crate::utils::normalize_kraken_asset;
use reqwest::Error;
use serde_json::Value;
use std::collections::HashMap;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    // Step 1: Fetch all asset pairs
    let url = "https://api.kraken.com/0/public/AssetPairs";
    let resp: Value = reqwest::get(url).await?.json().await?;

    let mut pair_map: HashMap<String, (String, String)> = HashMap::new();
    let mut valid_pairs: Vec<String> = Vec::new();

    if let Some(map) = resp["result"].as_object() {
        for (pair_name, info) in map {
            let status = info.get("status").and_then(|v| v.as_str()).unwrap_or("");
            let base = info.get("base").and_then(|v| v.as_str()).unwrap_or("");
            let quote = info.get("quote").and_then(|v| v.as_str()).unwrap_or("");

            // Only include spot, online pairs
            if status == "online" {
                let base_norm = normalize_kraken_asset(base);
                let quote_norm = normalize_kraken_asset(quote);
                pair_map.insert(pair_name.clone(), (base_norm, quote_norm));
                valid_pairs.push(pair_name.clone());
            }
        }
    }

    if valid_pairs.is_empty() {
        return Ok(vec![]);
    }

    // Step 2: Fetch all tickers in one request
    let ticker_url = format!(
        "https://api.kraken.com/0/public/Ticker?pair={}",
        valid_pairs.join(",")
    );
    let ticker_resp: Value = reqwest::get(&ticker_url).await?.json().await?;

    let mut out = Vec::new();
    if let Some(ticker_obj) = ticker_resp["result"].as_object() {
        for (pair_name, ticker_data) in ticker_obj {
            if let Some((base, quote)) = pair_map.get(pair_name) {
                if let Some(price_str) = ticker_data["c"][0].as_str() {
                    if let Ok(price) = price_str.parse::<f64>() {
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
