use crate::error::Error;
use crate::mappers::BandStaticMapper;
use crate::types::Mapper;
use crate::types::{PriceInfo, Source};
use price_adapter_raw::HuobiPro as HuobiProRaw;

pub type DefaultHuobiPro = HuobiPro<BandStaticMapper>;

// Generic struct `HuobiPro` parameterized over a `Mapper` type.
pub struct HuobiPro<M: Mapper> {
    raw: HuobiProRaw,
    mapper: M,
}

impl<M: Mapper> HuobiPro<M> {
    // Constructor for the `HuobiPro` struct.
    pub fn new(mapper: M) -> Self {
        let raw = HuobiProRaw::new();
        Self { raw, mapper }
    }
}

impl DefaultHuobiPro {
    // Constructor for a default `HuobiPro` instance with `BandStaticMapper`.
    pub fn new_with_default() -> Result<Self, Error> {
        let mapper = BandStaticMapper::from_source("huobipro")?;
        Ok(Self::new(mapper))
    }
}

#[async_trait::async_trait]
impl<M: Mapper> Source for HuobiPro<M> {
    // Asynchronous function to get prices for symbols.
    async fn get_prices(&self, symbols: &[&str]) -> Vec<Result<PriceInfo, Error>> {
        // Retrieve the symbol-to-id mapping from the provided mapper.
        let mapping = self.mapper.get_mapping().await;

        // Match on the result of obtaining the mapping.
        if let Ok(mapping) = &mapping {
            // Collect symbols with associated ids and indices.
            let ids_with_index: Vec<(&str, &str, usize)> = symbols
                .iter()
                .enumerate()
                .filter_map(|(index, &symbol)| {
                    mapping
                        .get(symbol)
                        .and_then(|id| id.as_str().map(|id| (symbol, id, index)))
                })
                .collect();

            // Extract only the ids from the collected tuples.
            let ids: Vec<&str> = ids_with_index.iter().map(|(_, id, _)| *id).collect();

            // Retrieve prices for the collected ids asynchronously.
            let prices = self.raw.get_prices(ids.as_slice()).await;

            // Initialize a vector to store the results.
            let mut res: Vec<Result<PriceInfo, Error>> = symbols
                .iter()
                .map(|_| Err(Error::UnsupportedSymbol))
                .collect();

            // Iterate over collected ids and prices to populate the results vector.
            for (&id, price) in ids_with_index.iter().zip(prices) {
                // Assign the result based on the price, mapping errors.
                res[id.2] = price
                    .map_err(Error::PriceAdapterRawError)
                    .map(|p| PriceInfo {
                        symbol: id.0.to_string(),
                        price: p.price,
                        timestamp: p.timestamp,
                    });
            }

            // Return the results vector.
            res
        } else {
            // Return errors for symbols if there's an issue with the mapping.
            symbols.iter().map(|_| Err(Error::MappingError)).collect()
        }
    }

    // Asynchronous function to get price for a symbol.
    async fn get_price(&self, symbol: &str) -> Result<PriceInfo, Error> {
        self.get_prices(&[symbol])
            .await
            .pop()
            .ok_or(Error::Unknown)?
    }
}
