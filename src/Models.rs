use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct TriangularResult {
    pub triangle: String,           // e.g., "BTC/ETH → ETH/USDT → BTC/USDT"
    pub profit_before_fees: f64,    // %
    pub trade_fees: f64,            // total % (fee_per_leg * 3)
    pub profit_after_fees: f64,     // %
}

#[derive(Clone, Debug)]
pub struct PairPrice {
    pub base: String,
    pub quote: String,
    pub price: f64, // price in quote per base (quote/base)
}
