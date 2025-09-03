use crate::models::PairPrice;
use crate::utils::split_concat_symbol;
use reqwest::Error;
use serde_json::Value;
use std::collections::HashSet;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    let price_url = "https://api.binance.com/api/v3/ticker/price";
    let info_url = "https://api.binance.com/api/v3/exchangeInfo";

    let info_data: Value = reqwest::get(info_url).await?.json().await?;
    let mut spot_symbols = HashSet::new();
    if let Some(symbols) = info_data.get("symbols").and_then(|s| s.as_array()) {
        for s in symbols {
            if let (Some(symbol), Some(status), Some(spot)) = (
                s.get("symbol").and_then(|v| v.as_str()),
                s.get("status").and_then(|v| v.as_str()),
                s.get("isSpotTradingAllowed").and_then(|v| v.as_bool()),
            ) {
                if status == "TRADING" && spot {
                    spot_symbols.insert(symbol.to_uppercase());
                }
            }
        }
    }

    let data: Vec<Value> = reqwest::get(price_url).await?.json().await?;
    let mut out = vec![];

    for item in data {
        if let (Some(sym), Some(pstr)) = (
            item.get("symbol").and_then(|v| v.as_str()),
            item.get("price").and_then(|v| v.as_str()),
        ) {
            if spot_symbols.contains(&sym.to_uppercase()) {
                if let Ok(price) = pstr.parse::<f64>() {
                    if let Some((base, quote)) = split_concat_symbol(sym) {
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
