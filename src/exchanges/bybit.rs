use crate::models::PairPrice;
use crate::utils::split_concat_symbol;
use reqwest::Error;
use serde_json::Value;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    let url = "https://api.bybit.com/v5/market/tickers?category=spot";
    let resp: Value = reqwest::get(url).await?.json().await?;
    let mut out = Vec::new();

    if let Some(list) = resp["result"]["list"].as_array() {
        for item in list {
            let sym = item.get("symbol").and_then(|v| v.as_str());
            let pstr = item.get("lastPrice").and_then(|v| v.as_str());
            if let (Some(s), Some(ps)) = (sym, pstr) {
                if let Ok(price) = ps.parse::<f64>() {
                    if let Some((base, quote)) = split_concat_symbol(s) {
                        out.push(PairPrice { base, quote, price });
                    }
                }
            }
        }
    }
    Ok(out)
}
