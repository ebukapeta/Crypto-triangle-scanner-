use crate::models::PairPrice;
use crate::utils::split_concat_symbol;
use reqwest::Error;
use serde_json::Value;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    let url = "https://api.binance.com/api/v3/ticker/price";
    let data: Vec<Value> = reqwest::get(url).await?.json().await?;
    let mut out = Vec::new();
    for item in data {
        if let (Some(sym), Some(pstr)) =
            (item.get("symbol").and_then(|v| v.as_str()), item.get("price").and_then(|v| v.as_str()))
        {
            if let Ok(price) = pstr.parse::<f64>() {
                if let Some((base, quote)) = split_concat_symbol(sym) {
                    out.push(PairPrice { base, quote, price });
                }
            }
        }
    }
    Ok(out)
}
