use serde::{Serialize, Deserialize};

/// One pair price: base / quote -> price (quote per 1 base)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairPrice {
    pub base: String,   // e.g., "BTC"
    pub quote: String,  // e.g., "USDT"
    pub price: f64,     // price of 1 base in terms of quote
    pub is_spot: bool,  // true if pair belongs to spot market
}

/// The data returned to the frontend for each triangle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriangularResult {
    pub triangle: String,            // human-readable path "A/B -> B/C -> C/A"
    pub profit_before_fees: f64,     // percent before fees
    pub trade_fees: f64,             // total percent (3 legs combined)
    pub profit_after_fees: f64,      // percent after deducting fees
}

impl PairPrice {
    /// Helper to easily create a spot pair entry
    pub fn new_spot(base: &str, quote: &str, price: f64) -> Self {
        Self {
            base: base.to_uppercase(),
            quote: quote.to_uppercase(),
            price,
            is_spot: true,
        }
    }

    /// Helper to create a non-spot pair entry (e.g., futures)
    pub fn new_non_spot(base: &str, quote: &str, price: f64) -> Self {
        Self {
            base: base.to_uppercase(),
            quote: quote.to_uppercase(),
            price,
            is_spot: false,
        }
    }
}
