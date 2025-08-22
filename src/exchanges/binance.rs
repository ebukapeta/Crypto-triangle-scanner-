use reqwest;
use serde_json::Value;

pub async fn fetch_prices() -> Vec<(String, String, f64)> {
    let mut out = Vec::new();
    if let Ok(r) = reqwest::get("https://api.binance.com/api/v3/ticker/price")
        .await
        .and_then(|r| r.json::<Vec<Value>>())
        .await
    {
        for item in r {
            if let (Some(sym), Some(price_str)) = (item["symbol"].as_str(), item["price"].as_str())
            {
                if let Ok(price) = price_str.parse::<f64>() {
                    // Basic split: last 4 chars as quote
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
    out
}
