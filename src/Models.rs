use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct TriangularResult {
    pub triangle: String,       // Path: BTC/ETH → ETH/USDT → BTC/USDT
    pub profit_before_fees: f64, // Profit margin before fees (%)
    pub trade_fees: f64,         // Total fees in %
    pub profit_after_fees: f64,  // Final profit margin after fees (%)
}
