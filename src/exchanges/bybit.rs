use crate::models::PairPrice;
use crate::utils::split_concat_symbol;
use reqwest::Error;
use serde_json::Value;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    let url = "https://api.bybit.com/v5/market/tickers?category=spot";
    let v: Value = reqwest::get(url).await?.json().await?;
    let mut out = Vec::new();
    if let Some(list) = v.get("result").and_then(|r| r.get("list")).and_then(|l| l.as_array()) {
        for item in list {
            if let (Some(sym), Some(pstr)) = (item.get("symbol").and_then(|s| s.as_str()), item.get("lastPrice").and_then(|s| s.as_str())) {
                if let Ok(price) = pstr.parse::<f64>() {
                    if let Some((base, quote)) = split_concat_symbol(sym) {
                        out.push(PairPrice { base, quote, price });
                    }
                }
            }
        }
    }
    Ok(out)
}
