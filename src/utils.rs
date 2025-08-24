use crate::models::TriangularResult;

const FEE_PER_LEG: f64 = 0.1;
const TOTAL_LEGS: f64 = 3.0;

pub fn calculate_with_fees(triangle: String, raw_profit: f64) -> TriangularResult {
    let total_fee = FEE_PER_LEG * TOTAL_LEGS;
    let final_profit = raw_profit - total_fee;

    TriangularResult {
        triangle,
        profit_before_fees: raw_profit,
        trade_fees: total_fee,
        profit_after_fees: final_profit,
    }
}
