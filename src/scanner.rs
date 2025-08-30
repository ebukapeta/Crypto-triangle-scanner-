use crate::models::{PairPrice, TriangularResult};
use std::collections::{HashMap, HashSet};

/// Scan triangles using spot last prices.
/// - `min_profit` is the minimum % before fees to include (e.g., 0.3 = 0.3%)
/// - `fee_perc` is the per-leg fee percent (e.g., 0.1 = 0.1% per leg)
pub fn scan_triangles(prices: &[PairPrice], min_profit: f64, fee_perc: f64) -> Vec<TriangularResult> {
    // Build directed graph of rates
    // rate[u->v] = quotes of v per 1 unit of u
    let mut rate: HashMap<(String, String), f64> = HashMap::new();
    let mut neighbors: HashMap<String, HashSet<String>> = HashMap::new();

    for p in prices {
        if p.price <= 0.0 { continue; }
        // direct
        rate.insert((p.base.clone(), p.quote.clone()), p.price);
        neighbors.entry(p.base.clone()).or_default().insert(p.quote.clone());
        // inverse
        rate.insert((p.quote.clone(), p.base.clone()), 1.0 / p.price);
        neighbors.entry(p.quote.clone()).or_default().insert(p.base.clone());
    }

    let mut seen: HashSet<(String, String, String)> = HashSet::new();
    let mut out: Vec<TriangularResult> = Vec::new();

    for (a, bs) in &neighbors {
        for b in bs {
            if a == b { continue; }
            // neighbors of b
            if let Some(cs) = neighbors.get(b) {
                for c in cs {
                    if c == a || c == b { continue; }

                    // need edge c->a
                    if !neighbors.get(c).map_or(false, |set| set.contains(a)) { continue; }

                    let r1 = *rate.get(&(a.clone(), b.clone())).unwrap();
                    let r2 = *rate.get(&(b.clone(), c.clone())).unwrap();
                    let r3 = *rate.get(&(c.clone(), a.clone())).unwrap();

                    let gross = r1 * r2 * r3;                 // cycle multiplier before fees
                    let pb = (gross - 1.0) * 100.0;           // %
                    if !(pb.is_finite()) || pb <= 0.0 || pb > 10.0 { // filter nonsense/noisy >10%
                        continue;
                    }

                    // apply 3 legs of fees multiplicatively
                    let f = 1.0 - (fee_perc / 100.0);
                    let net = (r1 * f) * (r2 * f) * (r3 * f);
                    let pa = (net - 1.0) * 100.0;
                    let fee_loss = (pb - pa).max(0.0);

                    // dedupe triangles ignoring rotations (A,B,C) ~ (B,C,A) ~ (C,A,B)
                    let reps = vec![
                        (a.clone(), b.clone(), c.clone()),
                        (b.clone(), c.clone(), a.clone()),
                        (c.clone(), a.clone(), b.clone()),
                    ];
                    let key = reps.iter().min().unwrap().clone();
                    if !seen.insert(key) { continue; }

                    out.push(TriangularResult {
                        triangle: format!("{}/{} -> {}/{} -> {}/{}",
                                          a, b, b, c, c, a),
                        profit_before_fees: round2(pb),
                        trade_fees: round2(fee_loss),
                        profit_after_fees: round2(pa),
                    });
                }
            }
        }
    }

    // Sort by profit_after_fees desc
    out.sort_by(|x, y| y.profit_after_fees.partial_cmp(&x.profit_after_fees).unwrap());
    out
}

#[inline]
fn round2(v: f64) -> f64 { (v * 100.0).round() / 100.0 }
