use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct TriangularResult {
    pub triangle: String,
    pub profit_before_fees: f64,
    pub trade_fees: f64,
    pub profit_after_fees: f64,
}

#[derive(Clone, Debug)]
pub struct PairPrice {
    pub base: String,
    pub quote: String,
    pub price: f64, // quote per base
}
