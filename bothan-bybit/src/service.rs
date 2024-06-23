use std::sync::Arc;

use tokio::task::JoinSet;
use tokio::time::{interval, Duration, Interval};
use tracing::{info, warn};

use bothan_core::cache::{Cache, Error as CacheError, Error};
use bothan_core::service::{Error as ServiceError, Service, ServiceResult};
use bothan_core::types::PriceData;

use crate::api::types::ticker::{Category, Tickers};
use crate::api::BybitRestAPI;
use crate::service::parser::parse_tickers;

pub mod builder;
mod parser;

/// A service for interacting with the Bybit REST API and caching price data.
pub struct BybitService {
    cache: Arc<Cache<PriceData>>,
}

impl BybitService {
    /// Creates a new `BybitService` instance.
    pub async fn new(rest_api: BybitRestAPI, update_interval: Duration) -> Self {
        let cache = Arc::new(Cache::new(None));
        let update_price_interval = interval(update_interval);

        start_service(Arc::new(rest_api), cache.clone(), update_price_interval).await;

        Self { cache }
    }
}

#[async_trait::async_trait]
impl Service for BybitService {
    /// Retrieves price data for the given IDs.
    async fn get_price_data(&mut self, ids: &[&str]) -> Vec<ServiceResult<PriceData>> {
        self.cache
            .get_batch(ids)
            .await
            .into_iter()
            .map(|result| match result {
                Ok(price_data) => Ok(price_data),
                Err(CacheError::DoesNotExist) | Err(CacheError::Invalid) => {
                    Err(ServiceError::InvalidSymbol)
                }
                Err(e) => panic!("unexpected error: {}", e), // This should never happen
            })
            .collect()
    }
}

/// Starts the price data update service.
pub async fn start_service(
    rest_api: Arc<BybitRestAPI>,
    cache: Arc<Cache<PriceData>>,
    mut update_price_interval: Interval,
) {
    update_price_data(rest_api.clone(), cache.clone()).await;
    tokio::spawn(async move {
        loop {
            update_price_interval.tick().await;
            update_price_data(rest_api.clone(), cache.clone()).await;
        }
    });
}

/// Updates the price data in the cache.
async fn update_price_data(rest_api: Arc<BybitRestAPI>, cache: Arc<Cache<PriceData>>) {
    match rest_api.get_tickers(Category::Spot, None).await {
        Ok(response) => {
            if let Some(tickers) = response.result.list {
                process_tickers(&tickers, response.time, cache).await;
            } else {
                warn!("failed to get market data: {}", response.ret_msg);
            }
        }
        Err(e) => {
            warn!("failed to get market data: {}", e);
        }
    }
}

/// Processes the tickers and updates the cache.
async fn process_tickers(tickers: &Tickers, timestamp: usize, cache: Arc<Cache<PriceData>>) {
    let mut set = JoinSet::new();
    for price_data in parse_tickers(tickers, timestamp) {
        let cloned_cache = cache.clone();
        set.spawn(async move {
            let id = price_data.id.clone();

            if cloned_cache.get(&id).await == Err(Error::DoesNotExist) {
                cloned_cache.set_pending(id.clone()).await;
            }

            let result = cloned_cache.set_data(id.clone(), price_data).await;
            if result.is_err() {
                warn!("unexpected request to set data for id: {}", id);
            } else {
                info!("set price for id {}", id);
            }
        });
    }

    while set.join_next().await.is_some() {}
}

#[cfg(test)]
mod test {
    use mockito::ServerGuard;

    use crate::api::rest::test::{mock_tickers_response, setup as api_setup, MockBybit};

    use super::*;

    async fn setup() -> (Arc<BybitRestAPI>, Arc<Cache<PriceData>>, ServerGuard) {
        let cache = Arc::new(Cache::<PriceData>::new(None));
        let (server, rest_api) = api_setup().await;
        (Arc::new(rest_api), cache, server)
    }

    #[tokio::test]
    async fn test_update_price_data() {
        let (rest_api, cache, mut server) = setup().await;
        let mock_tickers = mock_tickers_response();
        let timestamp = 100000;
        let mock = server.set_successful_tickers(&mock_tickers, timestamp);

        update_price_data(rest_api, cache.clone()).await;
        mock.assert();

        let expected = parse_tickers(&mock_tickers.list.unwrap(), timestamp);
        let result = cache.get(&expected[0].id).await;
        assert_eq!(result.unwrap(), expected[0]);
    }

    #[tokio::test]
    async fn test_update_price_data_with_failure() {
        let (rest_api, cache, mut server) = setup().await;
        let mock = server.set_failed_tickers(Category::Spot);
        update_price_data(rest_api, cache.clone()).await;

        mock.assert();
        let result = cache.get("BTCUSDT").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_process_ticker() {
        let cache = Arc::new(Cache::<PriceData>::new(None));
        let tickers = mock_tickers_response();
        let timestamp = 100000;

        process_tickers(&tickers.list.clone().unwrap(), timestamp, cache.clone()).await;

        let result = cache.get("BTCUSDT").await;
        assert_eq!(
            result.unwrap(),
            parse_tickers(&tickers.list.unwrap(), timestamp)[0]
        );
    }
}
