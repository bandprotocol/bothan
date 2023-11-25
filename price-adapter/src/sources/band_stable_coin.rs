use crate::error::Error;
use crate::types::{PriceInfo, Source};
use std::time::{SystemTime, UNIX_EPOCH};

/// A struct representing a stable coin with a constant price.
pub struct BandStableCoin;

impl BandStableCoin {
    /// Constructor to create a new BandStableCoin.
    pub fn new() -> Self {
        Self
    }
}

impl Default for BandStableCoin {
    fn default() -> Self {
        Self::new()
    }
}

// Implementing the StableCoin trait for BandStableCoin.
#[async_trait::async_trait]
impl Source for BandStableCoin {
    async fn get_prices(&self, symbols: &[&str]) -> Vec<Result<PriceInfo, Error>> {
        let cur_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(Error::SystemTimeError);

        match cur_time {
            Ok(time) => symbols
                .iter()
                .map(|symbol| {
                    Ok(PriceInfo {
                        symbol: symbol.to_string(),
                        price: 1_f64,
                        timestamp: time.as_secs(),
                    })
                })
                .collect(),
            Err(_) => {
                return symbols.iter().map(|_| Err(Error::Unknown)).collect();
            }
        }
    }

    /// Retrieve the price of the stable coin.
    ///
    /// This method returns a constant value of 1.0 for any symbol.
    async fn get_price(&self, symbol: &str) -> Result<PriceInfo, Error> {
        Ok(PriceInfo {
            symbol: symbol.to_string(),
            price: 1_f64,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        })
    }
}
