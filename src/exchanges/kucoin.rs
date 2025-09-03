use crate::models::PairPrice;
use crate::utils::split_concat_symbol;
use reqwest::Error;
use serde_json::Value;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    // Step 1: fetch tradable spot symbols
    let meta_url = "https://api.kucoin.com/api/v2/symbols";
    let meta_resp: Value = reqwest::get(meta_url).await?.json().await?;
    let mut valid = std::collections::HashSet::new();

    if let Some(arr) = meta_resp["data"].as_array() {
        for sym in arr {
            if sym["enableTrading"].as_bool() == Some(true) {
                if let Some(symbol) = sym["symbol"].as_str() {
                    valid.insert(symbol.to_string());
                }
            }
        }
    }

    // Step 2: fetch ticker prices
    let tick_url = "https://api.kucoin.com/api/v1/market/allTickers";
    let tick_resp: Value = reqwest::get(tick_url).await?.json().await?;
    let mut out = Vec::new();

    if let Some(arr) = tick_resp["data"]["ticker"].as_array() {
        for item in arr {
            if let (Some(sym), Some(pstr)) =
                (item["symbol"].as_str(), item["last"].as_str())
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
