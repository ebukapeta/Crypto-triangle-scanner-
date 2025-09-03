use crate::models::PairPrice;
use reqwest::Error;
use serde_json::Value;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    let url = "https://api.gateio.ws/api/v4/spot/tickers";
    let data: Vec<Value> = reqwest::get(url).await?.json().await?;
    let mut out = Vec::new();

    for item in data {
        if let (Some(pair), Some(pstr)) = (
            item.get("currency_pair").and_then(|v| v.as_str()),
            item.get("last").and_then(|v| v.as_str()),
        ) {
            let parts: Vec<&str> = pair.split('_').collect();
            if parts.len() == 2 {
                if let Ok(price) = pstr.parse::<f64>() {
                    out.push(PairPrice {
                        base: parts[0].to_uppercase(),
                        quote: parts[1].to_uppercase(),
                        price,
                        is_spot: true,
                    });
                }
            }
        }
    }

    Ok(out)
}
