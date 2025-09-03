use crate::models::PairPrice;
use crate::utils::normalize_kraken_asset;
use reqwest::Error;
use serde_json::Value;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    // Step 1: fetch tradable asset pairs
    let meta_url = "https://api.kraken.com/0/public/AssetPairs";
    let meta_resp: Value = reqwest::get(meta_url).await?.json().await?;
    let mut valid = std::collections::HashMap::new();

    if let Some(obj) = meta_resp["result"].as_object() {
        for (key, val) in obj {
            if val["status"] == "online" && val["quote"].is_string() && val["base"].is_string() {
                let base = normalize_kraken_asset(val["base"].as_str().unwrap_or(""));
                let quote = normalize_kraken_asset(val["quote"].as_str().unwrap_or(""));
                valid.insert(key.to_string(), (base, quote));
            }
        }
    }

    // Step 2: fetch ticker prices
    let tick_url = "https://api.kraken.com/0/public/Ticker?pair=";
    let tick_resp: Value = reqwest::get(tick_url).await?.json().await?;
    let mut out = Vec::new();

    if let Some(obj) = tick_resp["result"].as_object() {
        for (sym, val) in obj {
            if let Some((base, quote)) = valid.get(sym) {
                if let Some(price_str) = val["c"][0].as_str() {
                    if let Ok(price) = price_str.parse::<f64>() {
                        out.push(PairPrice {
                            base: base.clone(),
                            quote: quote.clone(),
                            price,
                        });
                    }
                }
            }
        }
    }

    Ok(out)
}
