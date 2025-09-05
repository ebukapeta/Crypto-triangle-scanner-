use crate::models::PairPrice;
use crate::utils::normalize_kraken_asset;
use reqwest::Error;
use serde_json::Value;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    let url = "https://api.kraken.com/0/public/Ticker?pair=all";
    let resp: Value = reqwest::get(url).await?.json().await?;

    let mut out = Vec::new();

    if let Some(result) = resp.get("result").and_then(|r| r.as_object()) {
        for (pair, data) in result {
            // Skip synthetic/futures pairs
            if pair.contains(".d") || pair.contains(".m") || pair.contains("FUTURE") {
                continue;
            }

            // Kraken Ticker response -> "c" = last trade [<price>, <lot size>]
            if let Some(price_str) = data.get("c")
                .and_then(|v| v.get(0))
                .and_then(|v| v.as_str()) 
            {
                if let Ok(price) = price_str.parse::<f64>() {
                    // Kraken gives wsname like "ETH/USD" for spot markets
                    if let Some(wsname) = data.get("wsname").and_then(|v| v.as_str()) {
                        let parts: Vec<&str> = wsname.split('/').collect();
                        if parts.len() == 2 {
                            let base = normalize_kraken_asset(parts[0]);
                            let quote = normalize_kraken_asset(parts[1]);
                            // Push only real spot pairs
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
        }
    }

    Ok(out)
                        }
