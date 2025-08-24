use reqwest::Error;
use serde_json::Value;
use crate::models::PairPrice;
use crate::utils::normalize_asset;

/// Kraken’s `/0/public/Ticker` without pair param returns all tickers.
/// We must map result keys (pair codes) into base/quote.
/// Kraken uses many legacy codes; we normalize common ones.
pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    let url = "https://api.kraken.com/0/public/Ticker";
    let v: Value = reqwest::get(url).await?.json().await?;
    let mut out = Vec::new();

    if let Some(obj) = v["result"].as_object() {
        for (pair_code, val) in obj {
            // Price is "c"[0]
            if let Some(last) = val["c"].as_array().and_then(|a| a.get(0)).and_then(|x| x.as_str()) {
                if let Ok(price) = last.parse::<f64>() {
                    // Try to split the key into base/quote by heuristic:
                    // Many Kraken keys look like "XXBTZUSD", "XETHXXBT" etc.
                    let s = pair_code.to_string();
                    // Try cut last 3/4 for quote
                    let (base_raw, quote_raw) = if s.len() > 4 {
                        (&s[..s.len()-4], &s[s.len()-4..])
                    } else if s.len() > 3 {
                        (&s[..s.len()-3], &s[s.len()-3..])
                    } else {
                        (s.as_str(), "")
                    };
                    let base = normalize_asset(base_raw);
                    let quote = normalize_asset(quote_raw);
                    if !base.is_empty() && !quote.is_empty() {
                        out.push(PairPrice { base, quote, price });
                    }
                }
            }
        }
    }

    Ok(out)
                }
