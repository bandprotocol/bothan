use std::sync::Arc;

use tokio::time::{interval, Duration, Interval};
use tracing::{info, warn};

use bothan_core::cache::{Cache, Error as CacheError};
use bothan_core::service::{Error as ServiceError, Service, ServiceResult};
use bothan_core::types::PriceData;

use crate::api::types::SymbolPrice;
use crate::api::CryptoCompareRestAPI;
use crate::error::Error;

pub struct CryptoCompareService {
    cache: Arc<Cache<PriceData>>,
}

impl CryptoCompareService {
    pub async fn new(rest_api: CryptoCompareRestAPI, update_interval: Duration) -> Self {
        let cache = Arc::new(Cache::new(None));
        let update_price_interval = interval(update_interval);

        start_service(Arc::new(rest_api), cache.clone(), update_price_interval).await;

        Self { cache }
    }
}

#[async_trait::async_trait]
impl Service for CryptoCompareService {
    async fn get_price_data(&mut self, ids: &[&str]) -> Vec<ServiceResult<PriceData>> {
        let mut to_set_pending = Vec::<String>::new();
        let result = self
            .cache
            .get_batch(ids)
            .await
            .into_iter()
            .enumerate()
            .map(|(idx, x)| match x {
                Ok(v) => Ok(v),
                Err(CacheError::DoesNotExist) => {
                    to_set_pending.push(ids[idx].to_string());
                    Err(ServiceError::Pending)
                }
                Err(CacheError::Invalid) => Err(ServiceError::InvalidSymbol),
                Err(e) => panic!("unexpected error: {}", e), // This should never happen
            })
            .collect();

        if !to_set_pending.is_empty() {
            self.cache.set_batch_pending(to_set_pending).await
        }

        result
    }
}

pub async fn start_service(
    rest_api: Arc<CryptoCompareRestAPI>,
    cache: Arc<Cache<PriceData>>,
    mut update_price_interval: Interval,
) {
    tokio::spawn(async move {
        loop {
            update_price_interval.tick().await;
            update_price_data(&rest_api, &cache).await;
        }
    });
}

async fn update_price_data(rest_api: &CryptoCompareRestAPI, cache: &Cache<PriceData>) {
    let keys = cache.keys().await;
    let uppercase_keys: Vec<String> = keys.into_iter().map(|key| key.to_uppercase()).collect();

    let ids = uppercase_keys
        .iter()
        .map(|x| x.as_str())
        .collect::<Vec<&str>>();
    if let Ok(symbol_prices) = rest_api.get_multi_symbol_price(ids.as_slice()).await {
        for (id, symbol_price) in ids.iter().zip(symbol_prices.iter()) {
            if let Some(m) = symbol_price {
                process_symbol_price(m, cache).await;
            } else {
                warn!("id {} is missing symbol price data", id);
            }
        }
    } else {
        warn!("failed to get symbol price");
    }
}

async fn process_symbol_price(symbol_price: &SymbolPrice, cache: &Cache<PriceData>) {
    if let Ok(price_data) = parse_symbol_price(symbol_price) {
        let id = price_data.id.clone();
        if cache.set_data(id.clone(), price_data).await.is_err() {
            warn!("unexpected request to set data for id: {}", id);
        } else {
            info!("set price for id {}", id);
        }
    } else {
        warn!("failed to parse symbol price");
    }
}

fn parse_symbol_price(symbol_price: &SymbolPrice) -> Result<PriceData, Error> {
    Ok(PriceData::new(
        symbol_price.id.clone(),
        symbol_price.current_price.to_string(),
        symbol_price.timestamp,
    ))
}

#[cfg(test)]
mod test {
    use mockito::ServerGuard;

    use crate::api::rest::test::{setup as api_setup, MockCryptoCompare};
    use crate::api::types::SymbolPrice;
    use crate::mock::mock_utc;

    use super::*;

    async fn setup() -> (
        Arc<CryptoCompareRestAPI>,
        Arc<Cache<PriceData>>,
        ServerGuard,
    ) {
        let cache = Arc::new(Cache::<PriceData>::new(None));
        let (server, rest_api) = api_setup().await;
        (Arc::new(rest_api), cache, server)
    }

    #[tokio::test]
    async fn test_update_price_data() {
        // Set the timestamp to a fixed value for testing
        let timestamp_millis = 1694615225000;
        mock_utc::set_timestamp_millis(timestamp_millis);

        let now = mock_utc::now().timestamp() as u64;

        let (rest_api, cache, mut server) = setup().await;
        let symbol_prices = vec![SymbolPrice {
            id: "BTC".to_string(),
            current_price: 42000.69,
            timestamp: now,
        }];
        server.set_successful_multi_symbol_price(&["BTC"], &symbol_prices);
        cache.set_pending("btc".to_string()).await;
        update_price_data(&rest_api, &cache).await;
        let result = cache.get("btc").await;

        let expected = PriceData::new("BTC".to_string(), "42000.69".to_string(), now);
        assert_eq!(result, Ok(expected));
    }

    #[tokio::test]
    async fn test_process_symbol_price() {
        // Set the timestamp to a fixed value for testing
        let timestamp_millis = 1694615225000;
        mock_utc::set_timestamp_millis(timestamp_millis);

        let now = mock_utc::now().timestamp() as u64;

        let cache = Arc::new(Cache::<PriceData>::new(None));
        let symbol_price = SymbolPrice {
            id: "BTC".to_string(),
            current_price: 42000.69,
            timestamp: now,
        };

        cache.set_batch_pending(vec!["btc".to_string()]).await;
        process_symbol_price(&symbol_price, &cache).await;
        let result = cache.get("btc").await;

        let expected = PriceData::new("BTC".to_string(), "42000.69".to_string(), now);
        assert_eq!(result.unwrap(), expected);
    }

    #[tokio::test]
    async fn test_process_symbol_price_without_set_pending() {
        // Set the timestamp to a fixed value for testing
        let timestamp_millis = 1694615225000;
        mock_utc::set_timestamp_millis(timestamp_millis);

        let now = mock_utc::now().timestamp() as u64;

        let cache = Arc::new(Cache::<PriceData>::new(None));
        let symbol_price = SymbolPrice {
            id: "BTC".to_string(),
            current_price: 42000.69,
            timestamp: now,
        };

        process_symbol_price(&symbol_price, &cache).await;
        let result = cache.get("btc").await;
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_symbol_price() {
        // Set the timestamp to a fixed value for testing
        let timestamp_millis = 1694615225000;
        mock_utc::set_timestamp_millis(timestamp_millis);

        let now = mock_utc::now().timestamp() as u64;

        let symbol_price = SymbolPrice {
            id: "BTC".to_string(),
            current_price: 42000.69,
            timestamp: now,
        };

        let result = parse_symbol_price(&symbol_price);
        let expected = PriceData::new("BTC".to_string(), "42000.69".to_string(), now);
        assert_eq!(result.unwrap(), expected);
    }
}
