use crate::models::PairPrice;
use crate::utils::normalize_kraken_asset;
use reqwest::Error;
use serde_json::Value;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    let url = "https://api.kraken.com/0/public/AssetPairs";
    let raw: Value = reqwest::get(url).await?.json().await?;
    let mut out = vec![];

    if let Some(result) = raw.get("result").and_then(|r| r.as_object()) {
        for (k, v) in result {
            if let (Some(ws), Some(c0)) = (
                v.get("wsname").and_then(|x| x.as_str()),
                v.get("base").and_then(|b| b.as_str()),
            ) {
                if let Some(last) = v.get("c")
                    .and_then(|c| c.get(0))
                    .and_then(|x| x.as_str())
                {
                    if let Ok(price) = last.parse::<f64>() {
                        let parts = ws.split('/').collect::<Vec<&str>>();
                        if parts.len() == 2 {
                            out.push(PairPrice {
                                base: normalize_kraken_asset(parts[0]),
                                quote: normalize_kraken_asset(parts[1]),
                                price,
                                is_spot: true,
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(out)
            }
