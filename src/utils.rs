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

/// Kraken cleanup: remove leading X/Z and map weird codes to standard ones
pub fn normalize_kraken_asset(asset: &str) -> String {
    let mut s = asset.trim().to_uppercase();

    // strip common prefixes Kraken uses
    if s.starts_with('X') || s.starts_with('Z') {
        s = s.trim_start_matches(|c| c == 'X' || c == 'Z').to_string();
    }

    // special mappings
    match s.as_str() {
        "XBT" => "BTC".to_string(),
        "XDG" => "DOGE".to_string(),
        "XETH" => "ETH".to_string(),
        "XXMR" => "XMR".to_string(),
        "XXRP" => "XRP".to_string(),
        "XLTC" => "LTC".to_string(),
        "XBNB" => "BNB".to_string(),
        "XADA" => "ADA".to_string(),
        "XDOT" => "DOT".to_string(),
        _ => s, // if no mapping needed, keep as-is
    }
                              }

#[inline]
pub fn round2(v: f64) -> f64 { (v * 100.0).round() / 100.0 }
