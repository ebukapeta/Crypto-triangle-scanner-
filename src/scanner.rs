use crate::models::{PairPrice, TriangularResult};
use std::collections::HashMap;

pub fn scan_triangles(prices: &Vec<PairPrice>, min_profit: f64, fee: f64) -> Vec<TriangularResult> {
    let mut map: HashMap<(String, String), f64> = HashMap::new();
    for p in prices {
        map.insert((p.base.clone(), p.quote.clone()), p.price);
    }

    let mut results = Vec::new();

    for a in prices {
        for b in prices {
            if a.quote == b.base {
                for c in prices {
                    if b.quote == c.base && c.quote == a.base {
                        let mut value = 1.0;
                        value *= 1.0 / a.price;
                        value *= 1.0 / b.price;
                        value *= c.price;
                        let profit = (value - 1.0 - 3.0 * (fee / 100.0)) * 100.0;

                        if profit >= min_profit {
                            results.push(TriangularResult {
                                pair1: format!("{}/{}", a.base, a.quote),
                                pair2: format!("{}/{}", b.base, b.quote),
                                pair3: format!("{}/{}", c.base, c.quote),
                                profit_percent: profit,
                            });
                        }
                    }
                }
            }
        }
    }

    results
                        }
