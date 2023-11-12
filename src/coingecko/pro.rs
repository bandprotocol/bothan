use super::base::CoingeckoBase;
use crate::{error::Error, types::PriceInfo};

pub struct CoingeckoPro(CoingeckoBase);

impl CoingeckoPro {
    pub fn new() -> Self {
        Self(CoingeckoBase::new(
            "https://pro-api.coingecko.com/api/v3/".into(),
        ))
    }

    pub fn api_key(mut self, api_key: String) -> Self {
        self.0 = self.0.api_key(api_key);
        self
    }

    pub async fn get_prices(&self, symbol_ids: &[(&str, &str)]) -> Vec<Result<PriceInfo, Error>> {
        self.0.get_prices(symbol_ids).await
    }
}
