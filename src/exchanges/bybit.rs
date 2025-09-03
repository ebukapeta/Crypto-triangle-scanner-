use crate::models::PairPrice;
use crate::utils::split_concat_symbol;
use reqwest::Error;
use serde_json::Value;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    // Step 1: fetch tradable spot symbols
    let meta_url = "https://api.bybit.com/v5/market/instruments-info?category=spot";
    let meta_resp: Value = reqwest::get(meta_url).await?.json().await?;
    let mut valid = std::collections::HashSet::new();

    if let Some(arr) = meta_resp["result"]["list"].as_array() {
        for sym in arr {
            if sym["status"] == "Trading" {
                if let Some(symbol) = sym["symbol"].as_str() {
                    valid.insert(symbol.to_string());
                }
            }
        }
    }

    // Step 2: fetch live ticker prices
    let tick_url = "https://api.bybit.com/v5/market/tickers?category=spot";
    let tick_resp: Value = reqwest::get(tick_url).await?.json().await?;
    let mut out = Vec::new();

    if let Some(arr) = tick_resp["result"]["list"].as_array() {
        for item in arr {
            if let (Some(sym), Some(pstr)) =
                (item["symbol"].as_str(), item["lastPrice"].as_str())
            {
                if !valid.contains(sym) { continue; }
                if let Ok(price) = pstr.parse::<f64>() {
                    if let Some((base, quote)) = split_concat_symbol(sym) {
                        out.push(PairPrice { base, quote, price });
                    }
                }
            }
        }
    }

    Ok(out)
        }
