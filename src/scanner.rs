use crate::models::TriangularResult;
use std::collections::{HashMap, HashSet};

/// Find triangular arbitrage opportunities from a (base, quote, price) list.
/// - `prices`: Vec of (BASE, QUOTE, price) where price = QUOTE per 1 BASE (last price).
/// - `min_profit`: minimum required profit **before fees**, in percent.
/// - `fee_per_leg`: fee **percent** applied on each leg (e.g., 0.10 means 0.10%).
pub fn find_triangular_arbitrage(
    prices: Vec<(String, String, f64)>,
    min_profit: f64,
    fee_per_leg: f64,
) -> Vec<TriangularResult> {
    // --- Build rate map (A,B) -> price (B per 1 A) ---
    let mut rate: HashMap<(String, String), f64> = HashMap::new();
    let mut assets: HashSet<String> = HashSet::new();

    for (base, quote, p) in prices.into_iter() {
        if p.is_finite() && p > 0.0 {
            let b = base.trim().to_uppercase();
            let q = quote.trim().to_uppercase();
            assets.insert(b.clone());
            assets.insert(q.clone());
            rate.insert((b, q), p);
        }
    }

    let assets: Vec<String> = assets.into_iter().collect();
    let n = assets.len();

    // Helper to get A->B conversion rate using direct or inverse pair
    let mut get_rate = |a: &str, b: &str| -> Option<(f64, String)> {
        if let Some(p) = rate.get(&(a.to_string(), b.to_string())) {
            // direct: a/b at price p; 1 a -> p b
            return Some((*p, format!("{}/{}", a, b)));
        }
        if let Some(p) = rate.get(&(b.to_string(), a.to_string())) {
            // inverse: b/a at price p; 1 a -> 1/p b
            return Some((1.0 / *p, format!("{}/{} (inv)", b, a)));
        }
        None
    };

    let fee_mult_one_leg = 1.0 - (fee_per_leg / 100.0);
    let total_fee_percent = 3.0 * fee_per_leg;

    let mut out: Vec<TriangularResult> = Vec::new();

    // --- Try all ordered triples (A,B,C), A,B,C distinct ---
    for i in 0..n {
        for j in 0..n {
            if j == i { continue; }
            for k in 0..n {
                if k == i || k == j { continue; }

                let a = &assets[i];
                let b = &assets[j];
                let c = &assets[k];

                // Need A->B, B->C, C->A rates
                let (r_ab, leg1) = match get_rate(a, b) { Some(x) => x, None => continue };
                let (r_bc, leg2) = match get_rate(b, c) { Some(x) => x, None => continue };
                let (r_ca, leg3) = match get_rate(c, a) { Some(x) => x, None => continue };

                // Gross (before fees)
                // Start with 1.0 A:
                // 1 A -> r_ab B -> r_ab*r_bc C -> r_ab*r_bc*r_ca A
                let gross_return = r_ab * r_bc * r_ca;
                let profit_before = (gross_return - 1.0) * 100.0;

                // Quick sanity: discard negatives and absurd values
                if !profit_before.is_finite() || profit_before <= 0.0 || profit_before > 10.0 {
                    continue;
                }

                // Apply fees multiplicatively (three legs)
                let net_return = gross_return * (fee_mult_one_leg * fee_mult_one_leg * fee_mult_one_leg);
                let profit_after = (net_return - 1.0) * 100.0;

                // Respect threshold on BEFORE-fee profit (your earlier API comment)
                if profit_before >= min_profit {
                    out.push(TriangularResult {
                        leg1,
                        leg2,
                        leg3,
                        profit_before_fees: round4(profit_before),
                        fee_perc_per_leg: fee_per_leg,
                        total_fee_percent: round4(total_fee_percent),
                        profit_after_fees: round4(profit_after),
                    });
                }
            }
        }
    }

    // Sort by profit AFTER fees (what matters to users)
    out.sort_by(|a, b| b.profit_after_fees.partial_cmp(&a.profit_after_fees).unwrap_or(std::cmp::Ordering::Equal));
    out
}

fn round4(x: f64) -> f64 {
    (x * 10_000.0).round() / 10_000.0
            }
