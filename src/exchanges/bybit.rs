use reqwest;
use serde_json::Value;

pub async fn fetch_prices() -> Vec<(String, String, f64)> {
    let mut out = Vec::new();
    if let Ok(r) = reqwest::get("https://api.bybit.com/v5/market/tickers?category=spot")
        .await
        .and_then(|r| r.json::<Value>())
        .await
    {
        if let Some(list) = r["result"]["list"].as_array() {
            for item in list {
                if let (Some(sym), Some(price_str)) = (item["symbol"].as_str(), item["lastPrice"].as_str())
                {
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
        }
    }
    out
}
