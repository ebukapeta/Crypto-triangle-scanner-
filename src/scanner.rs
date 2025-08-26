use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ArbitrageOpportunity {
    pub triangle_pair: String,
    pub profit_before_fees: f64,
    pub trade_fee_percent: f64,
    pub profit_after_fees: f64,
}

#[derive(Debug, Clone)]
pub struct Market {
    pub symbol: String,
    pub base: String,
    pub quote: String,
    pub spot: bool,
    pub active: bool,
    pub bid_price: f64,
    pub ask_price: f64,
}

/// Helper to get the best available price for a market
impl Market {
    pub fn best_price(&self) -> f64 {
        self.bid_price.max(self.ask_price)
    }
}

/// Main function to find triangular arbitrage opportunities
pub fn find_triangular_arbitrage(markets: &[Market]) -> Vec<ArbitrageOpportunity> {
    let mut opportunities = Vec::new();
    let triangles = generate_triangles(markets);

    for (pair_a, pair_b, pair_c) in triangles {
        // --- 1. Filter only active spot pairs ---
        if !pair_a.spot || !pair_a.active || !pair_b.spot || !pair_b.active || !pair_c.spot || !pair_c.active {
            continue;
        }

        // --- 2. Get prices and validate ---
        let price_a = pair_a.best_price();
        let price_b = pair_b.best_price();
        let price_c = pair_c.best_price();

        if price_a <= 0.0 || price_b <= 0.0 || price_c <= 0.0 {
            continue;
        }

        // --- 3. Calculate profit percentage ---
        let profit_percentage = calculate_profit(price_a, price_b, price_c);

        // --- 4. Sanity check for realistic range (0.1% - 10%) ---
        if profit_percentage <= 0.1 || profit_percentage > 10.0 {
            continue;
        }

        // --- 5. Add clean result to opportunities ---
        opportunities.push(ArbitrageOpportunity {
            triangle_pair: format!("{} → {} → {}", pair_a.symbol, pair_b.symbol, pair_c.symbol),
            profit_before_fees: profit_percentage,
            trade_fee_percent: 0.30,
            profit_after_fees: profit_percentage - 0.30,
        });
    }

    opportunities
}

/// Generate all possible triangles from available markets
fn generate_triangles(markets: &[Market]) -> Vec<(Market, Market, Market)> {
    let mut triangles = Vec::new();

    for a in markets {
        for b in markets {
            if a.symbol == b.symbol {
                continue;
            }
            for c in markets {
                if a.symbol == c.symbol || b.symbol == c.symbol {
                    continue;
                }

                // Check triangular relationship between pairs
                if a.base == b.quote && b.base == c.quote && c.base == a.quote {
                    triangles.push((a.clone(), b.clone(), c.clone()));
                }
            }
        }
    }

    triangles
}

/// Simplified profit calculation logic
fn calculate_profit(price_a: f64, price_b: f64, price_c: f64) -> f64 {
    // Multiply path prices to get total return
    let total_return = (1.0 / price_a) * price_b * price_c;
    let profit_percent = (total_return - 1.0) * 100.0;
    profit_percent
        }
