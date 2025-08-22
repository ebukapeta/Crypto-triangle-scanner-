use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ScanParams {
    pub exchange: String,
    pub min_profit: f64, // e.g. 0.5 for 0.5%
}