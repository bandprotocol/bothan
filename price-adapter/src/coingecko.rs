use super::mapper::types::Mapper;
use crate::error::Error;
use price_adapter_raw::types::PriceInfo;
use price_adapter_raw::CoinGecko as CoinGeckoRaw;

/// An object to query Coingecko public api.
pub struct CoinGecko {
    raw: CoinGeckoRaw,
    mapper: Box<dyn Mapper>,
}

impl CoinGecko {
    pub fn new(mapper: Box<dyn Mapper>, api_key: Option<String>) -> Self {
        let raw: CoinGeckoRaw;
        if let Some(key) = api_key {
            raw = CoinGeckoRaw::new_with_api_key(key);
        } else {
            raw = CoinGeckoRaw::new();
        }

        Self { raw, mapper }
    }

    /// get pair prices from the given queries (list of a tuple of (base, quote)).
    pub async fn get_prices(&self, symbols: &[&str]) -> Vec<Result<PriceInfo, Error>> {
        let mapping = self.mapper.get_mapping();

        let ids = symbols
            .iter()
            .filter_map(|&symbol| mapping.get(symbol).map(|id| id.as_str().unwrap()))
            .collect::<Vec<_>>();

        let res = self.raw.get_prices(ids.as_slice()).await;

        res.into_iter()
            .map(|result| result.map_err(Error::RawError))
            .collect()
    }
}
