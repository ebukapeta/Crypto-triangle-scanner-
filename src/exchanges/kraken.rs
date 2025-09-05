use crate::models::PairPrice;
use crate::utils::normalize_kraken_asset;
use reqwest::Error;
use serde_json::Value;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    let url = "https://api.kraken.com/0/public/AssetPairs";
    let resp: Value = reqwest::get(url).await?.json().await?;

    let mut out = Vec::new();

    if let Some(data) = resp.get("result").and_then(|r| r.as_object()) {
        for (_sym, v) in data {
            // ✅ Filter only spot markets that are online
            if v.get("status").and_then(|s| s.as_str()) != Some("online") {
                continue;
            }
            if v.get("wsname").is_none() {
                continue;
            }

            if let (Some(base), Some(quote)) = (v.get("base"), v.get("quote")) {
                let base = normalize_kraken_asset(base.as_str().unwrap_or(""));
                let quote = normalize_kraken_asset(quote.as_str().unwrap_or(""));

                // Kraken gives tick_size / min_price etc. but we need actual ticker data
                let ticker_url = format!("https://api.kraken.com/0/public/Ticker?pair={}", v["altname"].as_str().unwrap_or(""));
                let ticker_resp: Value = reqwest::get(&ticker_url).await?.json().await?;

                if let Some(result) = ticker_resp.get("result") {
                    if let Some(first) = result.as_object().and_then(|map| map.values().next()) {
                        if let Some(price_str) = first.get("c").and_then(|c| c.get(0)).and_then(|s| s.as_str()) {
                            if let Ok(price) = price_str.parse::<f64>() {
                                out.push(PairPrice { base, quote, price, is_spot: true });
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(out)
                                                                         }
