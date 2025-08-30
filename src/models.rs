use serde::Serialize;

#[derive(Clone, Debug)]
pub struct PairPrice {
    pub base: String,
    pub quote: String,
    pub price: f64, // quote per base
}

#[derive(Serialize, Clone, Debug)]
pub struct TriangularResult {
    pub triangle: String,              // e.g. "BTC/USDT -> ETH/USDT -> BTC/ETH"
    pub profit_before_fees: f64,       // %
    pub trade_fees: f64,               // % (total across 3 legs)
    pub profit_after_fees: f64,        // %
}
