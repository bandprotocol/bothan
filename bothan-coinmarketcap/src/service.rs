use std::sync::Arc;

use tokio::task::JoinSet;
use tokio::time::{interval, Duration, Interval};
use tracing::{info, warn};

use bothan_core::cache::{Cache, Error as CacheError};
use bothan_core::service::{Error as ServiceError, Service, ServiceResult};
use bothan_core::types::PriceData;

use crate::api::types::Quote;
use crate::api::CoinMarketCapRestAPI;
use crate::service::parser::{parse_quote, QuoteParserError};

pub mod builder;
mod parser;

pub struct CoinMarketCapService {
    cache: Arc<Cache<PriceData>>,
}

impl CoinMarketCapService {
    pub async fn new(rest_api: CoinMarketCapRestAPI, update_interval: Duration) -> Self {
        let cache = Arc::new(Cache::new(None));
        let update_price_interval = interval(update_interval);

        start_service(Arc::new(rest_api), cache.clone(), update_price_interval).await;

        Self { cache }
    }
}

#[async_trait::async_trait]
impl Service for CoinMarketCapService {
    async fn get_price_data(&mut self, ids: &[&str]) -> Vec<ServiceResult<PriceData>> {
        let mut to_set_pending = Vec::<String>::new();

        let result = self
            .cache
            .get_batch(ids)
            .await
            .into_iter()
            .enumerate()
            .map(|(idx, pd)| match pd {
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
    rest_api: Arc<CoinMarketCapRestAPI>,
    cache: Arc<Cache<PriceData>>,
    mut update_price_interval: Interval,
) {
    tokio::spawn(async move {
        loop {
            update_price_interval.tick().await;
            update_price_data(rest_api.clone(), cache.clone()).await;
        }
    });
}

async fn update_price_data(rest_api: Arc<CoinMarketCapRestAPI>, cache: Arc<Cache<PriceData>>) {
    let keys = cache.keys().await;
    let parsed_ids = keys
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<usize>>();

    if let Ok(quote) = rest_api.get_latest_quotes(parsed_ids.as_slice()).await {
        let mut set = JoinSet::new();
        for (id, quote) in parsed_ids.iter().zip(quote.into_iter()) {
            if let Some(q) = quote {
                let cloned_cache = cache.clone();
                set.spawn(async move {
                    process_price_quote(&q, &cloned_cache).await;
                });
            } else {
                warn!("id {} is missing market data", id);
            }
        }
        while set.join_next().await.is_some() {}
    } else {
        warn!("failed to get market data");
    }
}

async fn process_price_quote(quote: &Quote, cache: &Cache<PriceData>) {
    match parse_quote(quote) {
        Ok(price_data) => {
            let id = price_data.id.clone();
            if cache.set_data(id.clone(), price_data).await.is_err() {
                warn!("unexpected request to set data for id: {}", id);
            } else {
                info!("set price for id {}", id);
            }
        }
        Err(QuoteParserError::InvalidTimestamp) => warn!("failed to parse date time"),
        Err(QuoteParserError::InvalidPrice) => warn!("invalid price given"),
    };
}

#[cfg(test)]
mod test {
    use mockito::ServerGuard;

    use crate::api::rest::test::{mock_quote, setup as api_setup, MockCoinMarketCap};

    use super::*;

    async fn setup() -> (
        Arc<CoinMarketCapRestAPI>,
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
        let mock_quote = mock_quote();
        let coin_market = vec![mock_quote.clone()];
        server.set_successful_quotes(&["1"], &coin_market);
        cache.set_batch_pending(vec!["1".to_string()]).await;

        update_price_data(rest_api, cache.clone()).await;

        let result = cache.get("1").await;
        let expected = parse_quote(&mock_quote).unwrap();
        assert_eq!(result.unwrap(), expected);
    }

    #[tokio::test]
    async fn test_process_market_data() {
        let cache = Arc::new(Cache::<PriceData>::new(None));
        let quote = mock_quote();

        cache.set_batch_pending(vec!["1".to_string()]).await;
        process_price_quote(&quote, &cache).await;
        let result = cache.get("1").await;
        let expected = parse_quote(&quote).unwrap();
        assert_eq!(result.unwrap(), expected);
    }

    #[tokio::test]
    async fn test_process_market_data_without_set_pending() {
        let cache = Arc::new(Cache::<PriceData>::new(None));
        let quote = mock_quote();

        process_price_quote(&quote, &cache).await;
        let result = cache.get("1").await;
        assert!(result.is_err());
    }
}
