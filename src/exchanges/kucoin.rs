use crate::models::PairPrice;
use crate::utils::split_concat_symbol;
use reqwest::Error;
use serde_json::Value;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    let url = "https://api.kucoin.com/api/v1/market/allTickers";
    let resp: Value = reqwest::get(url).await?.json().await?;
    let mut out = Vec::new();

    if let Some(tickers) = resp["data"]["ticker"].as_array() {
        for item in tickers {
            if let (Some(sym), Some(pstr)) =
                (item.get("symbol").and_then(|v| v.as_str()), item.get("last").and_then(|v| v.as_str()))
            {
                if let Ok(price) = pstr.parse::<f64>() {
                    let pair = sym.replace("-", "");
                    if let Some((base, quote)) = split_concat_symbol(&pair) {
                        out.push(PairPrice { base, quote, price });
                    }
                }
            }
        }
    }

    Ok(out)
                        }
