use crate::models::PairPrice;
use reqwest::Error;
use serde_json::Value;
use std::collections::HashMap;

/// Normalize Kraken asset codes to common symbols (BTC, ETH, USD, etc.)
fn normalize_symbol(s: &str) -> String {
    match s {
        "XBT" => "BTC".to_string(),
        other => other.trim_start_matches('X').trim_start_matches('Z').to_string(),
    }
}

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    // 1) Fetch AssetPairs metadata to know valid spot markets and map pair code -> (base, quote)
    let url_pairs = "https://api.kraken.com/0/public/AssetPairs";
    let resp_pairs: Value = reqwest::get(url_pairs).await?.json().await?;

    let mut code_to_base_quote: HashMap<String, (String, String)> = HashMap::new();

    if let Some(result) = resp_pairs.get("result").and_then(|r| r.as_object()) {
        for (pair_code, details) in result {
            // Skip dark-pool pairs like "XXBTZUSD.d"
            if pair_code.ends_with(".d") {
                continue;
            }

            // Kraken spot pairs have base/quote and a wsname like "ETH/USD" or "BTC/USDT".
            // Some pairs also allow margin (non-empty leverage arrays) — that is STILL spot,
            // so we DO NOT filter them out.

            let base_raw = match details.get("base").and_then(|v| v.as_str()) {
                Some(v) => v,
                None => continue,
            };
            let quote_raw = match details.get("quote").and_then(|v| v.as_str()) {
                Some(v) => v,
                None => continue,
            };

            // Normalize e.g. XETH->ETH, ZUSD->USD, XBT->BTC, etc.
            let base = normalize_symbol(base_raw);
            let quote = normalize_symbol(quote_raw);

            // Defensive: require a human-readable wsname with slash (helps filter non-spot like indices)
            if details
                .get("wsname")
                .and_then(|v| v.as_str())
                .map(|s| s.contains('/'))
                .unwrap_or(false)
            {
                code_to_base_quote.insert(pair_code.clone(), (base, quote));
            }
        }
    }

    // If nothing mapped, return early
    if code_to_base_quote.is_empty() {
        return Ok(Vec::new());
    }

    // 2) Build a list of pair codes and fetch Ticker in batches (Kraken doesn't like huge URLs)
    let pair_codes: Vec<String> = code_to_base_quote.keys().cloned().collect();
    let mut out: Vec<PairPrice> = Vec::new();

    // Batch size ~75 is a safe middle ground
    for chunk in pair_codes.chunks(75) {
        let url_ticker = format!(
            "https://api.kraken.com/0/public/Ticker?pair={}",
            chunk.join(",")
        );
        let resp_ticker: Value = reqwest::get(&url_ticker).await?.json().await?;

        // Kraken returns "error": [] and "result": { <pair_code>: {...}, ... }
        if let Some(result) = resp_ticker.get("result").and_then(|r| r.as_object()) {
            for (pair_code, data) in result {
                // Only accept pair codes we whitelisted from AssetPairs
                if let Some((base, quote)) = code_to_base_quote.get(pair_code) {
                    // Ticker field "c" => last trade [<price>, <lot size>]
                    if let Some(price_str) = data.get("c")
                        .and_then(|v| v.get(0))
                        .and_then(|v| v.as_str())
                    {
                        if let Ok(price) = price_str.parse::<f64>() {
                            if price.is_finite() && price > 0.0 {
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
        }
    }

    // Optional: log how many spot pairs we loaded
    println!("Kraken: loaded {} spot pairs", out.len());

    Ok(out)
    }
