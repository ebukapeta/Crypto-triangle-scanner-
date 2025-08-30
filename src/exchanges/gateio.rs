use crate::models::PairPrice;
use crate::utils::split_concat_symbol;
use reqwest::Error;
use serde_json::Value;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    let url = "https://api.gateio.ws/api/v4/spot/tickers";
    let data: Vec<Value> = reqwest::get(url).await?.json().await?;
    let mut out = Vec::new();

    for item in data {
        let sym = item.get("currency_pair").and_then(|v| v.as_str()); // "BTC_USDT"
        let pstr = item.get("last").and_then(|v| v.as_str());
        if let (Some(s), Some(ps)) = (sym, pstr) {
            let pair = s.replace('_', "");
            if let Ok(price) = ps.parse::<f64>() {
                if let Some((base, quote)) = split_concat_symbol(&pair) {
                    out.push(PairPrice { base, quote, price });
                }
            }
        }
    }
    Ok(out)
                                                                 }
