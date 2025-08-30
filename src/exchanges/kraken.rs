use crate::models::PairPrice;
use crate::utils::{split_concat_symbol, normalize_kraken_asset};
use reqwest::Error;
use serde_json::Value;

/// Simplified: query a set of popular alt names to avoid the heavy AssetPairs+chunking dance.
pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    // Add more as needed
    let pairs = [
        "XBTUSD","ETHUSD","ADAUSD","DOTUSD","SOLUSD","XRPUSD","DOGEUSD",
        "XBTUSDT","ETHUSDT","ADAUSDT","SOLUSDT","XRPUSDT","DOGEUSDT","DOTUSDT",
        "ETHXBT","ADAXBT","SOLXBT","XRPXBT","DOGEXBT",
    ].join(",");

    let url = format!("https://api.kraken.com/0/public/Ticker?pair={}", pairs);
    let resp: Value = reqwest::get(&url).await?.json().await?;
    let mut out = Vec::new();

    if let Some(obj) = resp.get("result").and_then(|r| r.as_object()) {
        for (k, v) in obj {
            // Kraken returns last trade price at c[0]
            if let Some(pstr) = v.get("c").and_then(|c| c.get(0)).and_then(|x| x.as_str()) {
                if let Ok(price) = pstr.parse::<f64>() {
                    // Try altname split approach: normalize assets then split
                    // First, try direct split on key (often like "XBTUSD" or "XXBTZUSD")
                    let cleaned = k.replace(".", "");
                    if let Some((base, quote)) = split_concat_symbol(&cleaned) {
                        let b = normalize_kraken_asset(&base);
                        let q = normalize_kraken_asset(&quote);
                        out.push(PairPrice { base: b, quote: q, price });
                        continue;
                    }
                }
            }
        }
    }
    Ok(out)
                    }
