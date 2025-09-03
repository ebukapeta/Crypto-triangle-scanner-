use crate::models::PairPrice;
use crate::utils::split_concat_symbol;
use reqwest::Error;
use serde_json::Value;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    // Step 1: fetch spot currency pairs
    let meta_url = "https://api.gateio.ws/api/v4/spot/currency_pairs";
    let meta_resp: Value = reqwest::get(meta_url).await?.json().await?;
    let mut valid = std::collections::HashSet::new();

    if let Some(arr) = meta_resp.as_array() {
        for sym in arr {
            if sym["trade_status"] == "tradable" {
                if let Some(id) = sym["id"].as_str() {
                    valid.insert(id.to_uppercase());
                }
            }
        }
    }

    // Step 2: fetch ticker prices
    let tick_url = "https://api.gateio.ws/api/v4/spot/tickers";
    let tick_resp: Value = reqwest::get(tick_url).await?.json().await?;
    let mut out = Vec::new();

    if let Some(arr) = tick_resp.as_array() {
        for item in arr {
            if let (Some(sym), Some(pstr)) =
                (item["currency_pair"].as_str(), item["last"].as_str())
            {
                let s = sym.replace("_", "").to_uppercase();
                if !valid.contains(&sym.to_string()) { continue; }
                if let Ok(price) = pstr.parse::<f64>() {
                    if let Some((base, quote)) = split_concat_symbol(&s) {
                        out.push(PairPrice { base, quote, price });
                    }
                }
            }
        }
    }

    Ok(out)
                                            }
