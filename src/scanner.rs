use crate::models::TriangularResult;
use std::collections::HashMap;

/// Find valid triangular arbitrage opportunities
///
/// - `prices`: HashMap of "BASE/QUOTE" -> price
/// - `min_profit`: minimum profit threshold (in percent)
/// - `fee`: trading fee per leg (in percent)
pub fn find_triangular_arbitrage(
    prices: HashMap<String, f64>,
    min_profit: f64,
    fee: f64,
) -> Vec<TriangularResult> {
    let mut results: Vec<TriangularResult> = Vec::new();

    // Convert pairs into structured form for easier processing
    let parsed_pairs: Vec<(String, String, f64)> = prices
        .iter()
        .filter_map(|(pair, price)| {
            let parts: Vec<&str> = pair.split('/').collect();
            if parts.len() == 2 {
                Some((parts[0].to_string(), parts[1].to_string(), *price))
            } else {
                None
            }
        })
        .collect();

    // Iterate through all possible combinations of three pairs
    for (i, (base1, quote1, price1)) in parsed_pairs.iter().enumerate() {
        for (j, (base2, quote2, price2)) in parsed_pairs.iter().enumerate() {
            if i == j {
                continue;
            }
            for (k, (base3, quote3, price3)) in parsed_pairs.iter().enumerate() {
                if i == k || j == k {
                    continue;
                }

                // Check if these three pairs form a valid triangular loop
                // Scenario: A/B -> B/C -> C/A
                let tokens = vec![base1, quote1, base2, quote2, base3, quote3];
                let unique_tokens: std::collections::HashSet<_> =
                    tokens.iter().collect();
                if unique_tokens.len() != 3 {
                    continue; // not a proper triangle with 3 unique tokens
                }

                // Simulate a trade with 1 unit of starting currency
                let start_amount = 1.0;
                let after_first = start_amount / price1;  // buy second token
                let after_second = after_first / price2;  // buy third token
                let after_third = after_second * price3;  // convert back to first token

                // Apply fees for each leg
                let fee_multiplier = (1.0 - (fee / 100.0)).powf(3.0);
                let final_amount = after_third * fee_multiplier;

                let profit_percent = (final_amount - start_amount) * 100.0;

                if profit_percent >= min_profit {
                    results.push(TriangularResult {
                        leg1: format!("{}/{}", base1, quote1),
                        leg2: format!("{}/{}", base2, quote2),
                        leg3: format!("{}/{}", base3, quote3),
                        profit_percent,
                    });
                }
            }
        }
    }

    // Sort results by profit descending
    results.sort_by(|a, b| b.profit_percent.partial_cmp(&a.profit_percent).unwrap());

    results
        }
