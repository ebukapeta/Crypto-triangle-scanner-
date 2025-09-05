use crate::models::PairPrice;
use crate::utils::normalize_kraken_asset;
use reqwest::Error;
use serde_json::Value;
use std::collections::HashSet;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    // Step 1: get all tradable spot pairs
    let pairs_url = "https://api.kraken.com/0/public/AssetPairs";
    let pairs_resp: Value = reqwest::get(pairs_url).await?.json().await?;
    let mut spot_pairs: HashSet<String> = HashSet::new();

    if let Some(result) = pairs_resp.get("result").and_then(|r| r.as_object()) {
        for (pair_name, info) in result {
            // "spot" is indicated by "wsname" and "aclass_base"/"aclass_quote" == "currency"
            if let (Some(base), Some(quote)) =
                (info.get("base").and_then(|v| v.as_str()), info.get("quote").and_then(|v| v.as_str()))
            {
                // only spot if asset classes are currencies
                let aclass_base = info.get("aclass_base").and_then(|v| v.as_str());
                let aclass_quote = info.get("aclass_quote").and_then(|v| v.as_str());

                if aclass_base == Some("currency") && aclass_quote == Some("currency") {
                    spot_pairs.insert(pair_name.to_string());
                }
            }
        }
    }

    // Step 2: get ticker prices for ALL pairs
    let url = "https://api.kraken.com/0/public/Ticker?pair=ALL";
    let resp: Value = reqwest::get(url).await?.json().await?;

    let mut out = Vec::new();

    if let Some(result) = resp.get("result").and_then(|r| r.as_object()) {
        for (pair_name, data) in result {
            // only keep if Kraken says it's a spot pair
            if !spot_pairs.contains(pair_name) {
                continue;
            }

            if let Some(price_str) = data
                .get("c")
                .and_then(|c| c.get(0))
                .and_then(|v| v.as_str())
            {
                if let Ok(price) = price_str.parse::<f64>() {
                    // normalize both base and quote
                    let base_raw = result[pair_name]["base"].as_str().unwrap_or("");
                    let quote_raw = result[pair_name]["quote"].as_str().unwrap_or("");
                    let base = normalize_kraken_asset(base_raw);
                    let quote = normalize_kraken_asset(quote_raw);

                    if !base.is_empty() && !quote.is_empty() && base != quote && price.is_finite() && price > 0.0 {
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

    Ok(out)
                        }
