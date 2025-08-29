use crate::models::PairPrice;
use crate::utils::split_concat_symbol;
use reqwest::Error;
use serde_json::Value;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    let url = "https://api.kraken.com/0/public/Ticker?pair=BTCUSD,ETHUSD,ADAUSD,DOGEUSD,XRPUSD,SOLUSD";
    let resp: Value = reqwest::get(url).await?.json().await?;
    let mut out = Vec::new();

    if let Some(result) = resp["result"].as_object() {
        for (sym, item) in result {
            if let Some(pstr) = item["c"][0].as_str() {
                if let Ok(price) = pstr.parse::<f64>() {
                    if let Some((base, quote)) = split_concat_symbol(sym) {
                        out.push(PairPrice { base, quote, price });
                    }
                }
            }
        }
    }

    Ok(out)
}
