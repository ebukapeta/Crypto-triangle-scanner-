use reqwest;
use serde_json::Value;

pub async fn fetch_prices() -> Vec<(String, String, f64)> {
    let mut out = Vec::new();
    if let Ok(r) = reqwest::get("https://api.kucoin.com/api/v1/market/allTickers")
        .await
        .and_then(|r| r.json::<Value>())
        .await
    {
        if let Some(arr) = r["data"]["ticker"].as_array() {
            for item in arr {
                if let (Some(sym), Some(price_str)) = (item["symbol"].as_str(), item["last"].as_str())
                {
                    if let Ok(price) = price_str.parse::<f64>() {
                        let s = sym.replace('-', "");
                        let (base, quote) = if s.len() > 4 {
                            (&s[..s.len() - 4], &s[s.len() - 4..])
                        } else {
                            (&s, "")
                        };
                        out.push((base.to_string(), quote.to_string(), price));
                    }
                }
            }
        }
    }
    out
}