use crate::models::PairPrice;
use reqwest::Error;
use serde_json::Value;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    let url = "https://api.gateio.ws/api/v4/spot/tickers";
    let raw: Vec<Value> = reqwest::get(url).await?.json().await?;
    let mut out = vec![];

    for item in raw {
        if let (Some(pair), Some(pstr)) = (
            item.get("currency_pair").and_then(|v| v.as_str()),
            item.get("last").and_then(|v| v.as_str()),
        ) {
            if let Ok(price) = pstr.parse::<f64>() {
                let pq = pair.split('_').collect::<Vec<&str>>();
                if pq.len() == 2 {
                    out.push(PairPrice {
                        base: pq[0].to_string(),
                        quote: pq[1].to_string(),
                        price,
                        is_spot: true,
                    });
                }
            }
        }
    }

    Ok(out)
}
