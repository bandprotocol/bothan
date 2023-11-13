use super::base::CoingeckoBase;
use crate::{error::Error, types::PriceInfo};

/// An object to query Coingecko pro api.
pub struct CoingeckoPro(CoingeckoBase);

impl CoingeckoPro {
    /// initiate new api object.
    pub fn new() -> Self {
        Self(CoingeckoBase::new(
            "https://pro-api.coingecko.com/api/v3/".into(),
        ))
    }

    /// set up an api key.
    pub fn api_key(mut self, api_key: String) -> Self {
        self.0 = self.0.api_key(api_key);
        self
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
