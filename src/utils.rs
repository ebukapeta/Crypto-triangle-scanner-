// Utilities for parsing symbols, rounding, normalization

/// Known quote assets used to split concatenated symbols like BTCUSDT
pub fn known_quotes() -> &'static [&'static str] {
    &[
        "USDT","USD","USDC","BTC","ETH","BUSD","EUR","TRY","TUSD","DAI","BNB","AUD","GBP",
        "IDR","NGN","RUB","BRL","JPY","KRW","UST","PAX"
    ]
}

/// Try to split concatenated symbol like BTCUSDT -> (BTC, USDT)
pub fn split_concat_symbol(sym: &str) -> Option<(String, String)> {
    let s = sym.trim().to_uppercase();
    for q in known_quotes() {
        if s.ends_with(q) && s.len() > q.len() {
            let base = s[..s.len() - q.len()].to_string();
            return Some((base, (*q).to_string()));
        }
    }
    None
}

/// Normalize Kraken asset codes into standard tickers
pub fn normalize_kraken_asset(asset: &str) -> String {
    match asset {
        "XXBT" | "XBT" => "BTC".to_string(),
        "XETH" | "ETH" => "ETH".to_string(),
        "XLTC" | "LTC" => "LTC".to_string(),
        "XDG"  | "DOGE" => "DOGE".to_string(),
        "USDT" => "USDT".to_string(),
        "USDC" => "USDC".to_string(),
        "DAI"  => "DAI".to_string(),
        "ZEUR" | "EUR" => "EUR".to_string(),
        "ZUSD" | "USD" => "USD".to_string(),
        "ZGBP" | "GBP" => "GBP".to_string(),
        "ZJPY" | "JPY" => "JPY".to_string(),
        "ZCAD" | "CAD" => "CAD".to_string(),
        "ZCHF" | "CHF" => "CHF".to_string(),
        // fallback: strip leading X/Z if present
        _ => asset.trim_start_matches('X')
                  .trim_start_matches('Z')
                  .to_string(),
    }
}

#[inline]
pub fn round2(v: f64) -> f64 { (v * 100.0).round() / 100.0 }
