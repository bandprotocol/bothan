use super::{base::CoingeckoBase, PUBLIC_ENDPOINT};
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
        Self(CoingeckoBase::new(PUBLIC_ENDPOINT.into()))
    }

    pub async fn get_prices(&self, ids: &[&str]) -> Vec<Result<PriceInfo, Error>> {
        self.0.get_prices(ids).await
    }
}
