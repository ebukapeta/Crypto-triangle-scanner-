use serde_json::Value;
use reqwest::Client;
use std::collections::HashMap;
use tracing::info;

pub struct Scanner {
    client: Client,
}

impl Scanner {
    pub fn new() -> Self {
        Scanner {
            client: Client::new(),
        }
    }

    /// Hardcoded trading fees per exchange (per trade)
    fn get_exchange_fee(exchange: &str) -> f64 {
        match exchange.to_lowercase().as_str() {
            "binance" => 0.001, // 0.1%
            "kraken"  => 0.0026, // 0.26%
            "bybit"   => 0.001, // 0.1%
            _         => 0.001,
        }
    }

    /// Fetches prices for an exchange
    async fn fetch_prices(&self, exchange: &str) -> Result<Value, reqwest::Error> {
        let url = match exchange.to_lowercase().as_str() {
            "binance" => "https://api.binance.com/api/v3/ticker/bookTicker",
            "kraken"  => "https://api.kraken.com/0/public/Ticker?pair=BTCUSD,ETHUSD,XRPUSD",
            "bybit"   => "https://api.bybit.com/v5/market/tickers?category=spot",
            _ => return Err(reqwest::Error::new(
                reqwest::StatusCode::BAD_REQUEST,
                "Unsupported exchange",
            )),
        };

        info!("Fetching prices for {exchange} from {url}");
        let resp = self.client.get(url).send().await?;
        let json = resp.json::<Value>().await?;
        Ok(json)
    }

    /// Runs the triangular arbitrage scan for the given exchange
    pub async fn scan(&self, exchange: &str) -> Result<Vec<HashMap<String, String>>, String> {
        let fee = Self::get_exchange_fee(exchange);
        info!("Running scan for {exchange} with fee {fee}");

        match self.fetch_prices(exchange).await {
            Ok(data) => {
                // TODO: Replace with real triangular arbitrage logic
                // For now, simulate one fake profitable triangle
                let mut opportunity = HashMap::new();
                opportunity.insert("exchange".to_string(), exchange.to_string());
                opportunity.insert("triangle".to_string(), "BTC/ETH -> ETH/USDT -> BTC/USDT".to_string());
                opportunity.insert("profit".to_string(), format!("{:.4}%", 1.25 - (fee * 100.0)));
                Ok(vec![opportunity])
            }
            Err(e) => {
                let msg = format!("Failed to fetch data from {exchange}: {e}");
                info!("{msg}");
                Err(msg)
            }
        }
    }
                            }
