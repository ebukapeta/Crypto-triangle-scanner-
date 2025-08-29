// Utility to split a concatenated symbol like "BTCUSDT" into ("BTC", "USDT")
pub fn split_concat_symbol(symbol: &str) -> Option<(String, String)> {
    let quotes = ["USDT", "BTC", "ETH", "BUSD", "USDC"];
    for q in quotes {
        if symbol.ends_with(q) {
            let base = symbol.trim_end_matches(q).to_string();
            return Some((base, q.to_string()));
        }
    }
    None
}
