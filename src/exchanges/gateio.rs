use reqwest;
use serde_json::Value;

/// Fetch latest spot tickers from Gate.io
/// API: GET https://api.gateio.ws/api/v4/spot/tickers
/// 2
pub async fn fetch_prices() -> Result<Vec<(String, String, f64)>, reqwest::Error> {
    let mut out = Vec::new();

    let resp = reqwest::get("https://api.gateio.ws/api/v4/spot/tickers").await?;
    let arr = resp.json::<Value>().await?.as_array().cloned().unwrap_or_default();

    for item in arr {
        if let (Some(sym), Some(pstr)) = (item["currency_pair"].as_str(), item["last"].as_str()) {
            let s = sym.replace('_', "");
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

    Ok(out)
}
