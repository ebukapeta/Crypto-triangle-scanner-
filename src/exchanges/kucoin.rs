use reqwest;
use serde_json::Value;

/// Fetch latest spot prices from KuCoin
/// API: GET https://api.kucoin.com/api/v1/market/allTickers
/// 1
pub async fn fetch_prices() -> Result<Vec<(String, String, f64)>, reqwest::Error> {
    let mut out = Vec::new();

    let resp = reqwest::get("https://api.kucoin.com/api/v1/market/allTickers").await?;
    let v = resp.json::<Value>().await?;

    if let Some(tickers) = v["data"]["ticker"].as_array() {
        for item in tickers {
            if let (Some(sym), Some(pstr)) = (item["symbol"].as_str(), item["last"].as_str()) {
                let s = sym.replace('-', "");
                if let Ok(price) = pstr.parse::<f64>() {
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

    Ok(out)
                         }
