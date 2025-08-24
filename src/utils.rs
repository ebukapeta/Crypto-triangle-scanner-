pub fn known_quotes() -> Vec<&'static str> {
    vec![
        "USDT","USD","USDC","BTC","ETH","BUSD","EUR","TRY","TUSD","DAI","BNB","AUD","GBP",
        "IDR","NGN","RUB","BRL","JPY","KRW","UST","PAX"
    ]
}

pub fn split_concat_symbol(sym: &str) -> Option<(String, String)> {
    for q in known_quotes() {
        if sym.ends_with(q) && sym.len() > q.len() {
            let base = &sym[..sym.len()-q.len()];
            return Some((base.to_string(), q.to_string()));
        }
    }
    None
}

pub fn normalize_kraken_asset(asset: &str) -> String {
    let mut s = asset.trim_start_matches(|c| c == 'X' || c == 'Z').to_string();
    if s == "XBT" { s = "BTC".into(); }
    s
                }
