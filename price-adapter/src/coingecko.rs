use super::mapper::types::Mapper;
use crate::error::Error;
use crate::types::PriceInfo;
use price_adapter_raw::CoinGecko as CoinGeckoRaw;

/// An object to query Coingecko public api.
pub struct CoinGecko<M: Mapper> {
    raw: CoinGeckoRaw,
    mapper: M,
}

impl<M: Mapper> CoinGecko<M> {
    pub fn new(mapper: M, api_key: Option<String>) -> Self {
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

        let ids_with_index: Vec<(&str, &str, usize)> = symbols
            .iter()
            .enumerate()
            .filter_map(|(index, &symbol)| {
                mapping
                    .get(symbol)
                    .and_then(|id| id.as_str().and_then(|id| Some((symbol, id, index))))
            })
            .collect();

        let ids: Vec<&str> = ids_with_index.iter().map(|(_, id, _)| *id).collect();
        let prices = self.raw.get_prices(ids.as_slice()).await;

        let mut res: Vec<Result<PriceInfo, Error>> = symbols
            .iter()
            .map(|_| Err(Error::UnsupportedSymbol))
            .collect();

        for (&id, price) in ids_with_index.iter().zip(prices) {
            res[id.2] = price
                .map_err(Error::PriceAdapterRawError)
                .map(|p| PriceInfo {
                    symbol: id.0.to_string(),
                    price: p.price,
                    timestamp: p.timestamp,
                });
        }

        res
    }
}
