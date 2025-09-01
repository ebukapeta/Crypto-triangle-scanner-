use crate::models::PairPrice;
use reqwest::Error;
use serde_json::Value;

pub async fn fetch_prices() -> Result<Vec<PairPrice>, Error> {
    let url = "https://api.gateio.ws/api/v4/spot/tickers";
    let arr: Vec<Value> = reqwest::get(url).await?.json().await?;
    let mut out = Vec::new();
    for item in arr {
        if let (Some(cp), Some(pstr)) = (item.get("currency_pair").and_then(|s| s.as_str()), item.get("last").and_then(|s| s.as_str())) {
            if let Ok(price) = pstr.parse::<f64>() {
                let pair = cp.replace('_', "");
                if let Some((base, quote)) = crate::utils::split_concat_symbol(&pair) {
                    out.push(PairPrice { base, quote, price });
                }
            }
        }
    }
    Ok(out)
}
