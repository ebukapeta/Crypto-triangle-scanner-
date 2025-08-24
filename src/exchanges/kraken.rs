use reqwest;
use serde_json::Value;

/// Fetch spot ticker info from Kraken
/// API: GET https://api.kraken.com/0/public/Ticker
/// 3
pub async fn fetch_prices() -> Result<Vec<(String, String, f64)>, reqwest::Error> {
    let mut out = Vec::new();
    let url = "https://api.kraken.com/0/public/Ticker";
    let resp = reqwest::get(url).await?;
    let obj = resp.json::<Value>().await?;

    if let Some(result) = obj["result"].as_object() {
        for (pair, val) in result {
            if let Some(arr) = val["c"].as_array() {
                if let Some(pstr) = arr.get(0).and_then(|v| v.as_str()) {
                    if let Ok(price) = pstr.parse::<f64>() {
                        // Replace Kraken pair codes, e.g. XBT->BTC
                        let sym = pair.replace("XBT", "BTC");
                        // Split: naive assumption last 3-4 chars is quote
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

    Ok(out)
                    }
