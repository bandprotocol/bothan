use std::sync::Arc;

use tokio::select;
use tokio::time::{interval, Duration, Interval};
use tracing::{info, warn};

use bothan_core::cache::{Cache, Error as CacheError};
use bothan_core::service::{Error as ServiceError, Service, ServiceResult};
use bothan_core::types::PriceData;

use crate::api::types::Market;
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
            select! {
                _ = update_price_interval.tick() => {
                    update_price_data(&rest_api, &cache).await;
                },
            }
        }
    });
}

async fn update_price_data(rest_api: &Arc<CryptoCompareRestAPI>, cache: &Arc<Cache<PriceData>>) {
    let keys = cache.keys().await;
    let uppercase_keys: Vec<String> = keys.into_iter().map(|key| key.to_uppercase()).collect();

    let ids = uppercase_keys
        .iter()
        .map(|x| x.as_str())
        .collect::<Vec<&str>>();
    if let Ok(markets) = rest_api.get_coins_market(ids.as_slice()).await {
        for (id, market) in ids.iter().zip(markets.iter()) {
            if let Some(m) = market {
                process_market_data(m, cache).await;
            } else {
                warn!("id {} is missing market data", id);
            }
        }
    } else {
        warn!("failed to get market data");
    }
}

async fn process_market_data(market: &Market, cache: &Arc<Cache<PriceData>>) {
    if let Ok(price_data) = parse_market(market) {
        let id = price_data.id.clone();
        if cache.set_data(id.clone(), price_data).await.is_err() {
            warn!("unexpected request to set data for id: {}", id);
        } else {
            info!("set price for id {}", id);
        }
    } else {
        warn!("failed to parse market data");
    }
}

fn parse_market(market: &Market) -> Result<PriceData, Error> {
    Ok(PriceData::new(
        market.id.clone(),
        market.current_price.to_string(),
        market.timestamp,
    ))
}

#[cfg(test)]
mod test {
    use mockito::ServerGuard;

    use crate::api::rest::test::{setup as api_setup, MockCryptoCompare};
    use crate::api::types::Market;
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
        let coin_market = vec![Market {
            id: "BTC".to_string(),
            current_price: 42000.69,
            timestamp: now,
        }];
        server.set_successful_coins_market(&["BTC"], &coin_market);
        cache.set_pending("btc".to_string()).await;
        update_price_data(&rest_api, &cache).await;
        let result = cache.get("btc").await;

        let expected = PriceData::new("BTC".to_string(), "42000.69".to_string(), now);
        assert_eq!(result, Ok(expected));
    }

    #[tokio::test]
    async fn test_process_market_data() {
        // Set the timestamp to a fixed value for testing
        let timestamp_millis = 1694615225000;
        mock_utc::set_timestamp_millis(timestamp_millis);

        let now = mock_utc::now().timestamp() as u64;

        let cache = Arc::new(Cache::<PriceData>::new(None));
        let market = Market {
            id: "BTC".to_string(),
            current_price: 42000.69,
            timestamp: now,
        };

        cache.set_batch_pending(vec!["btc".to_string()]).await;
        process_market_data(&market, &cache).await;
        let result = cache.get("btc").await;

        let expected = PriceData::new("BTC".to_string(), "42000.69".to_string(), now);
        assert_eq!(result.unwrap(), expected);
    }

    #[tokio::test]
    async fn test_process_market_data_without_set_pending() {
        // Set the timestamp to a fixed value for testing
        let timestamp_millis = 1694615225000;
        mock_utc::set_timestamp_millis(timestamp_millis);

        let now = mock_utc::now().timestamp() as u64;

        let cache = Arc::new(Cache::<PriceData>::new(None));
        let market = Market {
            id: "BTC".to_string(),
            current_price: 42000.69,
            timestamp: now,
        };

        process_market_data(&market, &cache).await;
        let result = cache.get("btc").await;
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_market() {
        // Set the timestamp to a fixed value for testing
        let timestamp_millis = 1694615225000;
        mock_utc::set_timestamp_millis(timestamp_millis);

        let now = mock_utc::now().timestamp() as u64;

        let market = Market {
            id: "BTC".to_string(),
            current_price: 42000.69,
            timestamp: now,
        };

        let result = parse_market(&market);
        let expected = PriceData::new("BTC".to_string(), "42000.69".to_string(), now);
        assert_eq!(result.unwrap(), expected);
    }
}
