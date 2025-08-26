use std::collections::HashMap;
use anyhow::Result;

pub async fn scan_exchange(exchange: &str, fee: f64) -> Result<Vec<HashMap<String, String>>> {
    // Temporary fake arbitrage data for testing API/UI
    let mut opportunity = HashMap::new();
    opportunity.insert("exchange".to_string(), exchange.to_string());
    opportunity.insert(
        "triangle".to_string(),
        "BTC/ETH -> ETH/USDT -> BTC/USDT".to_string(),
    );
    opportunity.insert(
        "profit".to_string(),
        format!("{:.4}%", 1.25 - (fee * 100.0)),
    );

    Ok(vec![opportunity])
        }
