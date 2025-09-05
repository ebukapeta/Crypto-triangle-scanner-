use crate::models::PairPrice;
use crate::utils::normalize_kraken_asset;
use reqwest::Error;
use serde_json::Value;
use std::collections::HashMap;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    // 1. Get asset pairs (metadata)
    let pairs_resp: Value = reqwest::get("https://api.kraken.com/0/public/AssetPairs").await?.json().await?;

    let mut altname_to_pair: HashMap<String, (String, String)> = HashMap::new();
    if let Some(data) = pairs_resp.get("result").and_then(|r| r.as_object()) {
        for (_sym, v) in data {
            // Only spot + online
            if v.get("status").and_then(|s| s.as_str()) != Some("online") {
                continue;
            }

            if let (Some(base), Some(quote), Some(alt)) =
                (v.get("base"), v.get("quote"), v.get("altname"))
            {
                let base = normalize_kraken_asset(base.as_str().unwrap_or(""));
                let quote = normalize_kraken_asset(quote.as_str().unwrap_or(""));
                let alt = alt.as_str().unwrap_or("").to_string();
                altname_to_pair.insert(alt, (base, quote));
            }
        }
    }

    // 2. Build ticker URL with all alt names
    let joined: String = altname_to_pair.keys().cloned().collect::<Vec<_>>().join(",");
    let ticker_url = format!("https://api.kraken.com/0/public/Ticker?pair={}", joined);

    // 3. Fetch all ticker prices in one request
    let ticker_resp: Value = reqwest::get(&ticker_url).await?.json().await?;
    let mut out = Vec::new();

    if let Some(result) = ticker_resp.get("result").and_then(|r| r.as_object()) {
        for (alt, ticker) in result {
            if let Some((base, quote)) = altname_to_pair.get(alt) {
                if let Some(price_str) = ticker
                    .get("c")
                    .and_then(|c| c.get(0))
                    .and_then(|s| s.as_str())
                {
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
