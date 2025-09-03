use crate::models::PairPrice;
use reqwest::Error;
use serde_json::Value;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    let url = "https://api.bybit.com/v5/market/tickers?category=spot";
    let raw: Value = reqwest::get(url).await?.json().await?;
    let mut out = vec![];

    if let Some(list) = raw.get("result").and_then(|r| r.get("list")).and_then(|l| l.as_array()) {
        for item in list {
            if let (Some(sym), Some(pstr)) = (
                item.get("symbol").and_then(|v| v.as_str()),
                item.get("close").and_then(|v| v.as_str()),
            ) {
                if let Ok(price) = pstr.parse::<f64>() {
                    // Split the symbol, e.g., "BTCUSDT" -> ("BTC","USDT")
                    let base_quote = crate::utils::split_concat_symbol(sym);
                    if let Some((base, quote)) = base_quote {
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
