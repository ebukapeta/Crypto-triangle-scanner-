use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriangularResult {
    // Legs (actual pairs used, with inversion noted when applicable)
    pub leg1: String,
    pub leg2: String,
    pub leg3: String,

    // Profit in percent (not fractions)
    pub profit_before_fees: f64,
    pub fee_perc_per_leg: f64,
    pub total_fee_percent: f64,
    pub profit_after_fees: f64,
}
