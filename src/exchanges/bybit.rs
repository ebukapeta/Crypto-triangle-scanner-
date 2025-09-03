use crate::models::PairPrice;
use reqwest::Error;
use serde_json::Value;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    let url = "https://api.bybit.com/spot/v3/public/quote/ticker/price";
    let data: Value = reqwest::get(url).await?.json().await?;
    let mut out = Vec::new();

    if let Some(prices) = data.get("result").and_then(|v| v.as_array()) {
        for item in prices {
            if let (Some(base), Some(quote), Some(pstr)) = (
                item.get("baseCoin").and_then(|v| v.as_str()),
                item.get("quoteCoin").and_then(|v| v.as_str()),
                item.get("price").and_then(|v| v.as_str()),
            ) {
                if let Ok(price) = pstr.parse::<f64>() {
                    out.push(PairPrice {
                        base: base.to_uppercase(),
                        quote: quote.to_uppercase(),
                        price,
                        is_spot: true,
                    });
                }
            }
        }
    }

    Ok(out)
                }
