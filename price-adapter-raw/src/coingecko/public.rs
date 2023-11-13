use super::base::CoingeckoBase;
use crate::{error::Error, types::PriceInfo};

/// An object to query Coingecko public api.
pub struct CoingeckoPublic(CoingeckoBase);

impl Default for CoingeckoPublic {
    fn default() -> Self {
        Self::new()
    }
}

impl CoingeckoPublic {
    pub fn new() -> Self {
        Self(CoingeckoBase::new(
            "https://api.coingecko.com/api/v3/".into(),
        ))
    }

    /// get pair prices from the given queries (list of a tuple of (base, quote)).
    pub async fn get_prices(&self, symbol_ids: &[(&str, &str)]) -> Vec<Result<PriceInfo, Error>> {
        self.0.get_prices(symbol_ids).await
    }

    /// get a pair price from the given query.
    pub async fn get_price(&self, base: &str, quote: &str) -> Result<PriceInfo, Error> {
        self.0.get_price(base, quote).await
    }
}
