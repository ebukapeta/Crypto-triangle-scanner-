use crate::models::PairPrice;
use crate::utils::normalize_kraken_asset;
use reqwest::Error;
use serde_json::Value;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    // Step 1: Get all tradable spot pairs
    let pairs_url = "https://api.kraken.com/0/public/AssetPairs";
    let pairs_resp: Value = reqwest::get(pairs_url).await?.json().await?;
    let mut symbols = Vec::new();

    if let Some(result) = pairs_resp.get("result").and_then(|r| r.as_object()) {
        for (pair_name, info) in result {
            if let (Some(base), Some(quote)) = (
                info.get("base").and_then(|v| v.as_str()),
                info.get("quote").and_then(|v| v.as_str()),
            ) {
                let aclass_base = info.get("aclass_base").and_then(|v| v.as_str());
                let aclass_quote = info.get("aclass_quote").and_then(|v| v.as_str());

                // Only currencies = spot
                if aclass_base == Some("currency") && aclass_quote == Some("currency") {
                    symbols.push((pair_name.clone(), base.to_string(), quote.to_string()));
                }
            }
        }
    }

    if symbols.is_empty() {
        return Ok(Vec::new());
    }

    // Step 2: Fetch ticker only for these pairs
    let joined = symbols.iter().map(|(p, _, _)| p.as_str()).collect::<Vec<_>>().join(",");
    let ticker_url = format!("https://api.kraken.com/0/public/Ticker?pair={}", joined);
    let resp: Value = reqwest::get(&ticker_url).await?.json().await?;

    let mut out = Vec::new();

    if let Some(result) = resp.get("result").and_then(|r| r.as_object()) {
        for (pair_name, data) in result {
            if let Some((_, base_raw, quote_raw)) = symbols.iter().find(|(p, _, _)| p == pair_name) {
                if let Some(price_str) = data.get("c").and_then(|c| c.get(0)).and_then(|v| v.as_str()) {
                    if let Ok(price) = price_str.parse::<f64>() {
                        let base = normalize_kraken_asset(base_raw);
                        let quote = normalize_kraken_asset(quote_raw);

                        if !base.is_empty() && !quote.is_empty() && base != quote && price > 0.0 {
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
