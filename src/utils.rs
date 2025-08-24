use crate::models::PairPrice;

/// Known quote tokens to help split concatenated symbols
pub fn known_quotes() -> Vec<&'static str> {
    vec![
        "USDT","USD","USDC","BTC","ETH","BUSD","EUR","TRY","TUSD","DAI","UST","BNB","PAX","AUD","GBP","IDR","NGN","RUB","BRL","JPY","KRW"
    ]
}

/// Try to split a concatenated symbol like "BTCUSDT" -> ("BTC","USDT")
pub fn split_concat_symbol(sym: &str) -> Option<(String, String)> {
    for q in known_quotes() {
        if sym.ends_with(q) && sym.len() > q.len() {
            let base = &sym[..sym.len() - q.len()];
            return Some((base.to_string(), q.to_string()));
        }
    }
    None
}

/// Normalize Kraken asset codes (XBT -> BTC etc.)
pub fn normalize_asset(asset: &str) -> String {
    match asset {
        "XBT" => "BTC".into(),
        "XETH" => "ETH".into(),
        "XXBT" => "BTC".into(),
        "XETHZUSD" => "ETHUSD".into(),
        s => s.trim_start_matches('X').trim_start_matches('Z').to_string(),
    }
}

/// Make both directions for a price edge
pub fn expand_with_inverse(mut v: Vec<PairPrice>) -> Vec<PairPrice> {
    let mut inv = Vec::with_capacity(v.len());
    for p in &v {
        if p.price > 0.0 {
            inv.push(PairPrice {
                base: p.quote.clone(),
                quote: p.base.clone(),
                price: 1.0 / p.price,
            });
        }
    }
    v.extend(inv);
    v
}
