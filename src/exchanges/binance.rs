use reqwest;
use serde_json::Value;

pub async fn fetch_prices() -> Result<Vec<(String, String, f64)>, reqwest::Error> {
    let mut out = Vec::new();

    let resp = reqwest::get("https://api.binance.com/api/v3/ticker/price").await?;
    let data = resp.json::<Vec<Value>>().await?;

    for item in data {
        if let (Some(sym), Some(price_str)) = (item["symbol"].as_str(), item["price"].as_str()) {
            if let Ok(price) = price_str.parse::<f64>() {
                let (base, quote) = if sym.len() > 4 {
                    (&sym[..sym.len() - 4], &sym[sym.len() - 4..])
                } else {
                    (sym, "")
                };
                out.push((base.to_string(), quote.to_string(), price));
            }
        }
    }

    Ok(out)
                     }
