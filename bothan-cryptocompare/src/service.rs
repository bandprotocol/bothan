use std::sync::Arc;

use chrono::Utc;
use tokio::time::{interval, Duration, Interval};
use tracing::{info, warn};

use bothan_core::cache::{Cache, Error as CacheError};
use bothan_core::service::{Error as ServiceError, Service, ServiceResult};
use bothan_core::types::PriceData;

use crate::api::CryptoCompareRestAPI;

pub mod builder;

pub struct CryptoCompareService {
    cache: Arc<Cache<PriceData>>,
}

impl CryptoCompareService {
    pub async fn new(rest_api: CryptoCompareRestAPI, update_interval: Duration) -> Self {
        let cache = Arc::new(Cache::new(None));
        let update_price_interval = interval(update_interval);

        start_service(Arc::new(rest_api), cache.clone(), update_price_interval);

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
            .map(|(idx, result)| match result {
                Ok(price_data) => Ok(price_data),
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

pub fn start_service(
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

    if !keys.is_empty() {
        let uppercase_keys: Vec<String> = keys.into_iter().map(|key| key.to_uppercase()).collect();

        let now = Utc::now().timestamp() as u64;

        let ids = uppercase_keys
            .iter()
            .map(|x| x.as_str())
            .collect::<Vec<&str>>();
        if let Ok(symbol_prices) = rest_api.get_multi_symbol_price(ids.as_slice()).await {
            for (&id, symbol_price) in ids.iter().zip(symbol_prices.iter()) {
                if let Some(m) = symbol_price {
                    process_symbol_price(id, m, &now, cache).await;
                } else {
                    warn!("id {} is missing symbol price data", id);
                }
            }
        } else {
            warn!("failed to get symbol price");
        }
    }
}

async fn process_symbol_price(
    id: &str,
    symbol_price: &f64,
    timestamp: &u64,
    cache: &Cache<PriceData>,
) {
    let price_data = parse_symbol_price(id, symbol_price, timestamp);

    let id = price_data.id.clone();
    if cache.set_data(id.clone(), price_data).await.is_err() {
        warn!("unexpected request to set data for id: {}", id);
    } else {
        info!("set price for id {}", id);
    }
}

fn parse_symbol_price(id: &str, symbol_price: &f64, timestamp: &u64) -> PriceData {
    PriceData::new(
        id.to_owned(),
        symbol_price.to_string(),
        timestamp.to_owned(),
    )
}

#[cfg(test)]
mod test {
    use chrono::Utc;
    use mockito::ServerGuard;

    use crate::api::rest::test::{setup as api_setup, MockCryptoCompare};

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
        let (rest_api, cache, mut server) = setup().await;
        let now = Utc::now().timestamp() as u64;
        let symbol_prices = vec![42000.69];

        server.set_successful_multi_symbol_price(&["BTC"], &symbol_prices);
        cache.set_pending("btc".to_string()).await;
        update_price_data(&rest_api, &cache).await;
        let result = cache.get("btc").await;

        let expected = PriceData::new("BTC".to_string(), "42000.69".to_string(), now);
        assert_eq!(result, Ok(expected));
    }

    #[tokio::test]
    async fn test_process_symbol_price() {
        let cache = Arc::new(Cache::<PriceData>::new(None));
        let now = Utc::now().timestamp() as u64;
        let id = "BTC";
        let symbol_price = 42000.69;

        cache.set_batch_pending(vec!["btc".to_string()]).await;
        process_symbol_price(id, &symbol_price, &now, &cache).await;
        let result = cache.get("btc").await;

        let expected = PriceData::new("BTC".to_string(), "42000.69".to_string(), now);
        assert_eq!(result, Ok(expected));
    }

    #[tokio::test]
    async fn test_process_symbol_price_without_set_pending() {
        let cache = Arc::new(Cache::<PriceData>::new(None));
        let now = Utc::now().timestamp() as u64;
        let id = "BTC";
        let symbol_price = 42000.69;

        process_symbol_price(id, &symbol_price, &now, &cache).await;
        let result = cache.get("btc").await;
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_symbol_price() {
        let now = Utc::now().timestamp() as u64;
        let id = "BTC";
        let symbol_price = 42000.69;

        let result = parse_symbol_price(id, &symbol_price, &now);
        let expected = PriceData::new("BTC".to_string(), "42000.69".to_string(), now);
        assert_eq!(result, expected);
    }
}
