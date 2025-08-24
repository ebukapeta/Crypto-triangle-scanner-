use reqwest::Error;
use serde_json::Value;
use crate::models::PairPrice;
use crate::utils::split_concat_symbol;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    let url = "https://api.bybit.com/v5/market/tickers?category=spot";
    let v: Value = reqwest::get(url).await?.json().await?;
    let mut out = Vec::new();

    if let Some(list) = v["result"]["list"].as_array() {
        for item in list {
            if let (Some(sym), Some(pstr)) = (item["symbol"].as_str(), item["lastPrice"].as_str()) {
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
