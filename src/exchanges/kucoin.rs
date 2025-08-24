use reqwest::Error;
use serde_json::Value;
use crate::models::PairPrice;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    let url = "https://api.kucoin.com/api/v1/market/allTickers";
    let v: Value = reqwest::get(url).await?.json().await?;
    let mut out = Vec::new();

    if let Some(tickers) = v["data"]["ticker"].as_array() {
        for item in tickers {
            if let (Some(sym), Some(pstr)) = (item["symbol"].as_str(), item["last"].as_str()) {
                if let Ok(price) = pstr.parse::<f64>() {
                    // KuCoin uses "BASE-QUOTE"
                    let parts: Vec<&str> = sym.split('-').collect();
                    if parts.len() == 2 {
                        out.push(PairPrice {
                            base: parts[0].to_string(),
                            quote: parts[1].to_string(),
                            price,
                        });
                    }
                }
            }
        }
    }

    Ok(out)
}
