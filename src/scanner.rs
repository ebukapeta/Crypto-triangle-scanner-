use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::models::{PairPrice, TriangularResult};

/// Build adjacency: price[base][quote] = price (quote per base)
fn build_graph(pairs: &[PairPrice]) -> HashMap<String, HashMap<String, f64>> {
    let mut g: HashMap<String, HashMap<String, f64>> = HashMap::new();
    for p in pairs {
        if p.price <= 0.0 { continue; }
        g.entry(p.base.clone())
            .or_default()
            .insert(p.quote.clone(), p.price);
    }
    g
}

/// Find all unique triangles A->B->C->A and compute profit in percent:
/// profit_before = (A→B price * B→C price * C→A price - 1) * 100
pub fn scan_triangles(
    pairs: Vec<PairPrice>,
    min_profit_pct: f64,  // threshold before fees (e.g., 0.3)
    fee_per_leg_pct: f64, // e.g., 0.1
) -> Vec<TriangularResult> {
    let expanded = crate::utils::expand_with_inverse(pairs);
    let g = build_graph(&expanded);

    let mut results = Vec::new();
    let bases: HashSet<String> = g.keys().cloned().collect();

    for a in &bases {
        if let Some(nei_b) = g.get(a) {
            for b in nei_b.keys() {
                if a == b { continue; }
                if let Some(nei_c) = g.get(b) {
                    for c in nei_c.keys() {
                        if c == a || c == b { continue; }
                        // Check closing edge C->A
                        if let Some(p1) = g.get(a).and_then(|m| m.get(b)) {
                            if let Some(p2) = g.get(b).and_then(|m| m.get(c)) {
                                if let Some(p3) = g.get(c).and_then(|m| m.get(a)) {
                                    let product = p1 * p2 * p3;
                                    let profit_before = (product - 1.0) * 100.0;
                                    if profit_before >= min_profit_pct {
                                        let total_fee = fee_per_leg_pct * 3.0;
                                        let after = profit_before - total_fee;
                                        let triangle = format!("{}/{} → {}/{} → {}/{}",
                                            a, b, b, c, c, a
                                        );
                                        results.push(TriangularResult {
                                            triangle,
                                            profit_before_fees: round2(profit_before),
                                            trade_fees: round2(total_fee),
                                            profit_after_fees: round2(after),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Deduplicate by triangle string (cycles can repeat in order)
    results.sort_by(|x,y| x.triangle.cmp(&y.triangle));
    results.dedup_by(|x,y| x.triangle == y.triangle);
    // Sort by best profit_after_fees desc
    results.sort_by(|a,b| b.profit_after_fees.partial_cmp(&a.profit_after_fees).unwrap());
    results
}

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
                                }
