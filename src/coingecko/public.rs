use super::base::CoingeckoBase;
use crate::{error::Error, types::PriceInfo};

pub struct CoingeckoPublic(CoingeckoBase);

impl CoingeckoPublic {
    pub fn new() -> Self {
        Self(CoingeckoBase::new(
            "https://api.coingecko.com/api/v3/".into(),
        ))
    }

    pub async fn get_prices(&self, symbol_ids: &[(&str, &str)]) -> Vec<Result<PriceInfo, Error>> {
        self.0.get_prices(symbol_ids).await
    }
}
