use crate::models::PairPrice;
use crate::utils::normalize_kraken_asset;
use reqwest::Error;
use serde_json::Value;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    let url = "https://api.kraken.com/0/public/Ticker?pair=ALL";
    let data: Value = reqwest::get(url).await?.json().await?;
    let mut out = Vec::new();

    if let Some(result) = data.get("result").and_then(|v| v.as_object()) {
        for (pair, val) in result {
            if let Some(c) = val.get("c").and_then(|v| v.as_array()).and_then(|arr| arr.get(0)).and_then(|v| v.as_str()) {
                if let Ok(price) = c.parse::<f64>() {
                    let parts: Vec<&str> = pair.split('/').collect();
                    if parts.len() == 2 {
                        out.push(PairPrice {
                            base: normalize_kraken_asset(parts[0]),
                            quote: normalize_kraken_asset(parts[1]),
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
