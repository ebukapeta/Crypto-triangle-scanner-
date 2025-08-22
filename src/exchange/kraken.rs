use reqwest;
use serde_json::Value;

pub async fn fetch_prices() -> Vec<(String, String, f64)> {
    let candidates = ["XBTUSDT", "XBTUSD", "ETHUSDT", "ETHUSD", "ETHXBT"];
    let mut out = Vec::new();

    let pairs = candidates.join(",");
    let url = format!("https://api.kraken.com/0/public/Ticker?pair={}", pairs);

    if let Ok(r) = reqwest::get(&url).await.and_then(|r| r.json::<Value>()).await {
        if let Some(obj) = r["result"].as_object() {
            for (k, val) in obj {
                if let Some(arr) = val["c"].as_array() {
                    if let Some(price_str) = arr.get(0).and_then(|v| v.as_str()) {
                        if let Ok(price) = price_str.parse::<f64>() {
                            let sym = k.replace("XBT", "BTC");
                            let (base, quote) = if sym.len() > 4 {
                                (&sym[..sym.len() - 4], &sym[sym.len() - 4..])
                            } else {
                                (&sym, "")
                            };
                            out.push((base.to_string(), quote.to_string(), price));
                        }
                    }
                }
            }
        }
    }
    out
}