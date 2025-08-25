// utils.rs

/// List of known quote currencies for identifying trading pairs
pub fn known_quotes() -> Vec<&'static str> {
    vec![
        "USDT","USD","USDC","BTC","ETH","BUSD","EUR","TRY","TUSD","DAI","BNB","AUD","GBP",
        "IDR","NGN","RUB","BRL","JPY","KRW","UST","PAX"
    ]
}

/// Split a trading pair symbol into (base, quote)
pub fn split_concat_symbol(sym: &str) -> Option<(String, String)> {
    for q in known_quotes() {
        if sym.ends_with(q) && sym.len() > q.len() {
            let base = &sym[..sym.len() - q.len()];
            return Some((base.to_string(), q.to_string()));
        }
    }
    None
}

/// Normalize Kraken-specific asset codes (like XBT to BTC)
pub fn normalize_kraken_asset(asset: &str) -> String {
    let mut s = asset.trim_start_matches(|c| c == 'X' || c == 'Z').to_string();
    if s == "XBT" {
        s = "BTC".into();
    }
    s
}

/// Normalize asset symbols to uppercase (e.g., eth -> ETH)
pub fn normalize_asset(asset: &str) -> String {
    asset.trim().to_uppercase()
}

/// Round a floating-point number to the given decimal places
pub fn round_to(value: f64, decimals: u32) -> f64 {
    let factor = 10_f64.powi(decimals as i32);
    (value * factor).round() / factor
}

/// Log messages with a consistent format
pub fn log_message(message: &str) {
    println!("[LOG] {}", message);
        }
