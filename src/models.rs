use serde::Serialize;

#[derive(Clone, Debug)]
pub struct PairPrice {
    pub base: String,
    pub quote: String,
    pub price: f64,
}

#[derive(Serialize, Clone, Debug)]
pub struct TriangularResult {
    pub pair1: String,
    pub pair2: String,
    pub pair3: String,
    pub profit_percent: f64,
}
