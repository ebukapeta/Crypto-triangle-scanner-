use reqwest;
use serde_json::Value;

/// Fetch latest spot tickers for Bybit (real-time data)
/// API: GET https://api.bybit.com/v5/market/tickers?category=spot
/// 0
pub async fn fetch_prices() -> Result<Vec<(String, String, f64)>, reqwest::Error> {
    let mut out = Vec::new();

    let resp = reqwest::get("https://api.bybit.com/v5/market/tickers?category=spot").await?;
    let v = resp.json::<Value>().await?;

    if let Some(list) = v["result"]["list"].as_array() {
        for item in list {
            if let (Some(sym), Some(pstr)) = (item["symbol"].as_str(), item["lastPrice"].as_str()) {
                if let Ok(price) = pstr.parse::<f64>() {
                    let (base, quote) = if sym.len() > 4 {
                        (&sym[..sym.len() - 4], &sym[sym.len() - 4..])
                    } else {
                        (sym, "")
                    };
                    out.push((base.to_string(), quote.to_string(), price));
                }
            }
        }
    }

    Ok(out)
    }
