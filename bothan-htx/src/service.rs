use std::sync::Arc;

use tokio::task::JoinSet;
use tokio::time::{interval, Duration, Interval};
use tracing::warn;

use bothan_core::cache::{Cache, Error as CacheError, Error};
use bothan_core::service::{Error as ServiceError, Service, ServiceResult};
use bothan_core::types::PriceData;

use crate::api::types::{Status, Ticker};
use crate::api::HtxRestAPI;
use crate::service::parser::parse_ticker;

pub mod builder;
mod parser;

/// A service for interacting with the HTX REST API and caching price data.
pub struct HtxService {
    cache: Arc<Cache<PriceData>>,
}

impl HtxService {
    /// Creates a new `HtxService` instance.
    ///
    /// # Arguments
    ///
    /// * `rest_api` - An instance of `HtxRestAPI`.
    /// * `update_interval` - The interval for updating the price data.
    ///
    /// # Returns
    ///
    /// A new `HtxService` instance.
    pub async fn new(rest_api: HtxRestAPI, update_interval: Duration) -> Self {
        let cache = Arc::new(Cache::new(None));
        let update_price_interval = interval(update_interval);

        start_service(Arc::new(rest_api), cache.clone(), update_price_interval).await;

        Self { cache }
    }
}

#[async_trait::async_trait]
impl Service for HtxService {
    /// Retrieves price data for the given IDs.
    ///
    /// # Arguments
    ///
    /// * `ids` - A slice of string slices representing the IDs.
    ///
    /// # Returns
    ///
    /// A vector of `ServiceResult` containing `PriceData`.
    async fn get_price_data(&mut self, ids: &[&str]) -> Vec<ServiceResult<PriceData>> {
        self.cache
            .get_batch(ids)
            .await
            .into_iter()
            .map(|result| match result {
                Ok(price_data) => Ok(price_data),
                Err(CacheError::DoesNotExist) => Err(ServiceError::InvalidSymbol),
                Err(CacheError::Invalid) => Err(ServiceError::InvalidSymbol),
                Err(e) => panic!("unexpected error: {}", e), // This should never happen
            })
            .collect()
    }
}

/// Starts the service for updating price data.
///
/// # Arguments
///
/// * `rest_api` - An instance of `HtxRestAPI`.
/// * `cache` - An instance of `Cache<PriceData>`.
/// * `update_price_interval` - The interval for updating the price data.
pub async fn start_service(
    rest_api: Arc<HtxRestAPI>,
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
///
/// # Arguments
///
/// * `rest_api` - An instance of `HtxRestAPI`.
/// * `cache` - An instance of `Cache<PriceData>`.
async fn update_price_data(rest_api: Arc<HtxRestAPI>, cache: Arc<Cache<PriceData>>) {
    if let Ok(quote) = rest_api.get_latest_tickers().await {
        match quote.status {
            Status::Ok => {
                let mut set = JoinSet::new();
                let timestamp = quote.timestamp;
                for ticker in quote.data {
                    let cloned_cache = cache.clone();
                    set.spawn(async move {
                        process_ticker(&ticker, timestamp, &cloned_cache).await;
                    });
                }
                while set.join_next().await.is_some() {}
            }
            Status::Error => {
                warn!("received error status from api")
            }
        }
    } else {
        warn!("failed to get market data");
    }
}

/// Processes the ticker data and updates the cache.
///
/// # Arguments
///
/// * `ticker` - A reference to the `Ticker`.
/// * `timestamp` - The timestamp associated with the `Ticker`.
/// * `cache` - An instance of `Cache<PriceData>`.
async fn process_ticker(ticker: &Ticker, timestamp: usize, cache: &Cache<PriceData>) {
    let price_data = parse_ticker(ticker, timestamp);
    let id = price_data.id.clone();

    if cache.get(id.as_str()).await == Err(Error::DoesNotExist) {
        cache.set_pending(id.clone()).await;
    }

    let _ = cache.set_data(id, price_data).await;
}

#[cfg(test)]
mod test {
    use mockito::ServerGuard;

    use crate::api::rest::test::{mock_ticker, setup as api_setup, MockHtx};

    use super::*;

    async fn setup() -> (Arc<HtxRestAPI>, Arc<Cache<PriceData>>, ServerGuard) {
        let cache = Arc::new(Cache::<PriceData>::new(None));
        let (server, rest_api) = api_setup().await;
        (Arc::new(rest_api), cache, server)
    }

    #[tokio::test]
    async fn test_update_price_data() {
        let (rest_api, cache, mut server) = setup().await;
        let mock_ticker = mock_ticker();
        let tickers = vec![mock_ticker.clone()];
        let timestamp = 100000;
        let mock = server.set_successful_tickers(&tickers, timestamp);

        update_price_data(rest_api, cache.clone()).await;
        mock.assert();

        let result = cache.get("btcusdt").await;
        let expected = parse_ticker(&mock_ticker, timestamp);
        assert_eq!(result.unwrap(), expected);
    }

    #[tokio::test]
    async fn test_update_price_data_with_failure() {
        let (rest_api, cache, mut server) = setup().await;
        let mock = server.set_failed_tickers();
        update_price_data(rest_api, cache.clone()).await;

        mock.assert();
        let result = cache.get("btcusdt").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_process_ticker() {
        let cache = Arc::new(Cache::<PriceData>::new(None));
        let ticker = mock_ticker();
        let timestamp = 100000;

        process_ticker(&ticker, timestamp, &cache).await;
        let result = cache.get(&ticker.symbol).await;
        assert_eq!(result.unwrap(), parse_ticker(&ticker, timestamp));
    }
}
