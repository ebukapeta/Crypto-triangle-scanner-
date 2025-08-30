// Known quote assets to split concatenated symbols like BTCUSDT
pub fn known_quotes() -> &'static [&'static str] {
    &[
        "USDT","USD","USDC","BTC","ETH","BUSD","EUR","TRY","TUSD","DAI","BNB",
        "AUD","GBP","IDR","NGN","BRL","JPY","KRW","RUB","PAX"
    ]
}

// Split "BTCUSDT" -> Some(("BTC","USDT"))
pub fn split_concat_symbol(sym: &str) -> Option<(String, String)> {
    for q in known_quotes() {
        if sym.ends_with(q) && sym.len() > q.len() {
            let base = sym[..sym.len()-q.len()].to_string();
            return Some((base, (*q).to_string()));
        }
    }
    None
}

// Kraken uses XBT for BTC and X/Z prefixes; normalize to common names.
pub fn normalize_kraken_asset(sym: &str) -> String {
    let mut s = sym.trim_start_matches(|c| c == 'X' || c == 'Z').to_string();
    if s == "XBT" { s = "BTC".into(); }
    s
}
