use std::collections::{HashMap, HashSet};

pub struct Opportunity {
    pub path: Vec<String>,
    pub profit: f64,
}

pub fn find_triangles(
    pairs: &[(String, String, f64)],
    min_profit: f64,
) -> Vec<Opportunity> {
    let mut adj: HashMap<&str, Vec<(&str, f64)>> = HashMap::new();
    let mut assets = HashSet::new();

    for (base, quote, price) in pairs {
        assets.insert(base.as_str());
        assets.insert(quote.as_str());
        adj.entry(base).or_default().push((quote, *price));
        if *price != 0.0 {
            adj.entry(quote).or_default().push((base, 1.0 / *price));
        }
    }

    let mut results = Vec::new();
    let universe: Vec<&str> = assets.iter().cloned().collect();

    for &a in &universe {
        if let Some(e1s) = adj.get(a) {
            for &(b, r1) in e1s {
                if let Some(e2s) = adj.get(b) {
                    for &(c, r2) in e2s {
                        if let Some(e3s) = adj.get(c) {
                            for &(a2, r3) in e3s {
                                if a2 == a {
                                    let product = r1 * r2 * r3;
                                    let profit = product - 1.0;
                                    if profit >= min_profit {
                                        results.push(Opportunity {
                                            path: vec![a.to_string(), b.to_string(), c.to_string()],
                                            profit,
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

    results.sort_by(|x, y| y.profit.partial_cmp(&x.profit).unwrap());
    results
}
