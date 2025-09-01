use crate::models::PairPrice;
use crate::utils::{normalize_kraken_asset, split_concat_symbol};
use reqwest::Error;
use serde_json::Value;

/// Kraken: we query a set of commonly used pairs to keep it simple and avoid heavy chunking.
/// You can extend this list for additional pairs if needed.
pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    let pairs = [
        "XBTUSD","ETHUSD","ADAUSD","DOTUSD","SOLUSD","XRPUSD","DOGEUSD",
        "XBTUSDT","ETHUSDT","ADAUSDT","SOLUSDT","XRPUSDT","DOGEUSDT","DOTUSDT",
        "ETHXBT","ADAXBT","SOLXBT","XRPXBT","DOGEXBT",
    ].join(",");

    let url = format!("https://api.kraken.com/0/public/Ticker?pair={}", pairs);
    let v: Value = reqwest::get(&url).await?.json().await?;
    let mut out = Vec::new();

    if let Some(obj) = v.get("result").and_then(|r| r.as_object()) {
        for (k, info) in obj {
            // Kraken returns last trade price at c[0]
            if let Some(last) = info.get("c").and_then(|c| c.get(0)).and_then(|x| x.as_str()) {
                if let Ok(price) = last.parse::<f64>() {
                    // try to split the returned key (cleaned)
                    let key = k.replace(".", "");
                    // try to use the split logic (some keys map well)
                    if let Some((base_raw, quote_raw)) = split_concat_symbol(&key) {
                        let base = normalize_kraken_asset(&base_raw);
                        let quote = normalize_kraken_asset(&quote_raw);
                        out.push(PairPrice { base, quote, price });
                        continue;
                    }
                }
            }
        }
    }
    Ok(out)
}
