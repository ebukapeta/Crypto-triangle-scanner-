use crate::models::PairPrice;
use reqwest::Error;
use serde_json::Value;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    let url = "https://api.kucoin.com/api/v1/market/allTickers";
    let resp: Value = reqwest::get(url).await?.json().await?;
    let mut out = Vec::new();

    if let Some(tickers) = resp.get("data").and_then(|d| d.get("ticker")).and_then(|t| t.as_array()) {
        for t in tickers {
            if let (Some(symbol), Some(last_str)) = (t.get("symbol").and_then(|v| v.as_str()), t.get("last").and_then(|v| v.as_str())) {
                if let Ok(price) = last_str.parse::<f64>() {
                    // KuCoin symbols are like "BTC-USDT"
                    let parts: Vec<&str> = symbol.split('-').collect();
                    if parts.len() == 2 {
                        let base = parts[0].to_string();
                        let quote = parts[1].to_string();
                        out.push(PairPrice {
                            base,
                            quote,
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
