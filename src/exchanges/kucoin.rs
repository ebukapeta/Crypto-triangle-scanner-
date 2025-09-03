use crate::models::PairPrice;
use reqwest::Error;
use serde_json::Value;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    let url = "https://api.kucoin.com/api/v1/market/allTickers";
    let data: Value = reqwest::get(url).await?.json().await?;
    let mut out = Vec::new();

    if let Some(tickers) = data.get("data").and_then(|d| d.get("ticker")).and_then(|t| t.as_array()) {
        for item in tickers {
            if let (Some(symbol), Some(pstr)) = (
                item.get("symbol").and_then(|v| v.as_str()),
                item.get("last").and_then(|v| v.as_str()),
            ) {
                let parts: Vec<&str> = symbol.split('-').collect();
                if parts.len() == 2 {
                    if let Ok(price) = pstr.parse::<f64>() {
                        out.push(PairPrice {
                            base: parts[0].to_uppercase(),
                            quote: parts[1].to_uppercase(),
                            price,
                            is_spot: true, // KuCoin only returns spot here
                        });
                    }
                }
            }
        }
    }

    Ok(out)
}
