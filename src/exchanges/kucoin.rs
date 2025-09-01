use crate::models::PairPrice;
use reqwest::Error;
use serde_json::Value;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    let url = "https://api.kucoin.com/api/v1/market/allTickers";
    let v: Value = reqwest::get(url).await?.json().await?;
    let mut out = Vec::new();
    if let Some(tickers) = v.get("data").and_then(|d| d.get("ticker")).and_then(|t| t.as_array()) {
        for item in tickers {
            if let (Some(sym), Some(pstr)) = (item.get("symbol").and_then(|s| s.as_str()), item.get("last").and_then(|s| s.as_str())) {
                if let Ok(price) = pstr.parse::<f64>() {
                    // KuCoin symbol formats like "BTC-USDT"
                    let pair = sym.replace('-', "");
                    // Try to split using common quotes
                    if let Some((base, quote)) = crate::utils::split_concat_symbol(&pair) {
                        out.push(PairPrice { base, quote, price });
                    }
                }
            }
        }
    }
    Ok(out)
}
