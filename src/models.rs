use serde::{Serialize, Deserialize};

/// One pair price: base / quote -> price (quote per 1 base)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairPrice {
    pub base: String,
    pub quote: String,
    pub price: f64,
}

/// The data returned to the frontend for each triangle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriangularResult {
    pub triangle: String,            // human readable path "A/B -> B/C -> C/A"
    pub profit_before_fees: f64,     // percent
    pub trade_fees: f64,             // total percent (3 * per-leg)
    pub profit_after_fees: f64,      // percent
}
