use super::{base::CoingeckoBase, PRO_ENDPOINT};
use crate::{error::Error, types::PriceInfo};

/// An object to query Coingecko pro api.
pub struct CoingeckoPro(CoingeckoBase);

impl Default for CoingeckoPro {
    fn default() -> Self {
        Self::new()
    }
}

impl CoingeckoPro {
    /// initiate new api object.
    pub fn new() -> Self {
        Self(CoingeckoBase::new(PRO_ENDPOINT.into()))
    }

    /// set up an api key.
    pub fn set_api_key(mut self, api_key: String) -> Self {
        self.0 = self.0.set_api_key(api_key);
        self
    }

    pub async fn get_prices(&self, ids: &[&str]) -> Vec<Result<PriceInfo, Error>> {
        self.0.get_prices(ids).await
    }
}
