use reqwest::Error;
use serde_json::Value;
use crate::models::PairPrice;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    let url = "https://api.gateio.ws/api/v4/spot/tickers";
    let arr: Value = reqwest::get(url).await?.json().await?;
    let mut out = Vec::new();

    if let Some(list) = arr.as_array() {
        for item in list {
            if let (Some(sym), Some(pstr)) = (item["currency_pair"].as_str(), item["last"].as_str()) {
                if let Ok(price) = pstr.parse::<f64>() {
                    // Gate uses "BASE_QUOTE"
                    let parts: Vec<&str> = sym.split('_').collect();
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
