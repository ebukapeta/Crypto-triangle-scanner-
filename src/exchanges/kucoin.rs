use crate::models::PairPrice;
use reqwest::Error;
use serde_json::Value;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    let url = "https://api.kucoin.com/api/v1/market/allTickers";
    let raw: Value = reqwest::get(url).await?.json().await?;
    let mut out = vec![];

    if let Some(tickers) = raw.get("data").and_then(|d| d.get("ticker")).and_then(|t| t.as_array()) {
        for item in tickers {
            if let (Some(sym), Some(pstr)) = (
                item.get("symbol").and_then(|v| v.as_str()),
                item.get("last").and_then(|v| v.as_str()),
            ) {
                if let Ok(price) = pstr.parse::<f64>() {
                    let base_quote = sym.split('-').collect::<Vec<&str>>();
                    if base_quote.len() == 2 {
                        out.push(PairPrice {
                            base: base_quote[0].to_string(),
                            quote: base_quote[1].to_string(),
                            price,
                            is_spot: true,
                        });
                    }
                }
            }
        }
    }

    Ok(out)
                            }
