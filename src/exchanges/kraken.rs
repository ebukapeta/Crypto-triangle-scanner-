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
            // last trade price
            if let Some(price_str) = data.get("c")
                .and_then(|v| v.get(0))
                .and_then(|v| v.as_str()) 
            {
                if let Ok(price) = price_str.parse::<f64>() {
                    if let Some(wsname) = data.get("wsname").and_then(|v| v.as_str()) {
                        let parts: Vec<&str> = wsname.split('/').collect();
                        if parts.len() == 2 {
                            let base = normalize_kraken_asset(parts[0]);
                            let quote = normalize_kraken_asset(parts[1]);

                            // ✅ filter: only crypto/major stablecoin spot, skip pure fiat pairs
                            let skip_fiats = ["USD","EUR","GBP","JPY"];
                            if skip_fiats.contains(&base.as_str()) && skip_fiats.contains(&quote.as_str()) {
                                continue; // skip forex-only markets
                            }

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
