use std::collections::HashSet;
use std::sync::Arc;

use chrono::NaiveDateTime;
use tokio::select;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration, Interval};
use tracing::warn;

use bothan_core::cache::{Cache, Error as CacheError};
use bothan_core::service::{Error as ServiceError, Service, ServiceResult};
use bothan_core::types::PriceData;

use crate::api::types::Market;
use crate::api::CoinGeckoRestAPI;
use crate::error::Error;

pub struct CoinGeckoService {
    cache: Arc<Cache<PriceData>>,
    coin_list: Arc<RwLock<HashSet<String>>>,
}

impl CoinGeckoService {
    pub async fn new(
        rest_api: CoinGeckoRestAPI,
        update_interval: Duration,
        update_supported_assets_interval: Duration,
    ) -> Self {
        let cache = Arc::new(Cache::new(None));
        let coin_list = Arc::new(RwLock::new(HashSet::<String>::new()));
        let update_price_interval = interval(update_interval);
        let update_supported_assets_interval = interval(update_supported_assets_interval);

        start_service(
            rest_api,
            cache.clone(),
            update_price_interval,
            update_supported_assets_interval,
            coin_list.clone(),
        )
        .await;

        Self { cache, coin_list }
    }
}

#[async_trait::async_trait]
impl Service for CoinGeckoService {
    async fn get_price_data(&mut self, ids: &[&str]) -> Vec<ServiceResult<PriceData>> {
        let reader = self.coin_list.read().await;

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
                    if reader.contains(ids[idx]) {
                        to_set_pending.push(ids[idx].to_string());
                        Err(ServiceError::Pending)
                    } else {
                        Err(ServiceError::InvalidSymbol)
                    }
                }
                Err(CacheError::Invalid) => Err(ServiceError::Pending),
                Err(_) => Err(ServiceError::Pending),
            })
            .collect();

        if !to_set_pending.is_empty() {
            self.cache.set_batch_pending(to_set_pending).await
        }

        result
    }
}

pub async fn start_service(
    rest_api: CoinGeckoRestAPI,
    cache: Arc<Cache<PriceData>>,
    mut update_price_interval: Interval,
    mut update_supported_assets_interval: Interval,
    coin_list: Arc<RwLock<HashSet<String>>>,
) {
    update_coin_list(&rest_api, &coin_list).await;

    tokio::spawn(async move {
        loop {
            select! {
                _ = update_price_interval.tick() => {
                    update_price_data(&rest_api, &cache).await;
                },
                _ = update_supported_assets_interval.tick() => {
                    update_coin_list(&rest_api, &coin_list).await;
                },
            }
        }
    });
}

async fn update_price_data(rest_api: &CoinGeckoRestAPI, cache: &Arc<Cache<PriceData>>) {
    let keys = cache.keys().await;
    let ids = keys.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
    let market_results = rest_api.get_coins_market(&ids).await;
    for market_result in market_results {
        if let Ok(market) = market_result {
            process_market_data(&market, cache).await;
        } else {
            warn!("failed to get market data");
        }
    }
}

async fn update_coin_list(rest_api: &CoinGeckoRestAPI, coin_list: &Arc<RwLock<HashSet<String>>>) {
    if let Ok(new_coin_list) = rest_api.get_coins_list().await {
        let new_coin_set = HashSet::<String>::from_iter(new_coin_list.into_iter().map(|x| x.id));
        let mut locked = coin_list.write().await;
        *locked = new_coin_set;
    } else {
        warn!("Failed to get coin list");
    }
}

async fn process_market_data(market: &Market, cache: &Arc<Cache<PriceData>>) {
    if let Ok(price_data) = parse_market(market) {
        let id = price_data.id.clone();
        if cache.set_data(id.clone(), price_data).await.is_err() {
            warn!("Unexpected request to set data for id: {}", id);
        }
    } else {
        warn!("Failed to parse date time");
    }
}

fn parse_market(market: &Market) -> Result<PriceData, Error> {
    let last_updated = market.last_updated.as_str();
    let naive_date_time = NaiveDateTime::parse_from_str(last_updated, "%Y-%m-%dT%H:%M:%S.%fZ")
        .map_err(|_| Error::InvalidTimestamp)?;
    let timestamp =
        u64::try_from(naive_date_time.timestamp()).map_err(|_| Error::InvalidTimestamp)?;

    Ok(PriceData::new(
        market.id.clone(),
        market.current_price.to_string(),
        timestamp,
    ))
}

#[cfg(test)]
mod test {
    use mockito::ServerGuard;

    use crate::api::rest::test::{setup as api_setup, MockGecko};
    use crate::api::types::Coin;

    use super::*;

    fn setup() -> (CoinGeckoRestAPI, Arc<Cache<PriceData>>, ServerGuard) {
        let cache = Arc::new(Cache::<PriceData>::new(None));
        let (server, rest_api) = api_setup();
        (rest_api, cache, server)
    }

    #[tokio::test]
    async fn test_update_price_data() {
        let (rest_api, cache, mut server) = setup();
        let coin_market = vec![Market {
            id: "bitcoin".to_string(),
            symbol: "BTC".to_string(),
            name: "Bitcoin".to_string(),
            current_price: 8426.69,
            last_updated: "2021-01-01T00:00:00.000Z".to_string(),
        }];
        server.set_successful_coins_market(&["bitcoin"], &coin_market);
        cache.set_batch_pending(vec!["bitcoin".to_string()]).await;

        update_price_data(&rest_api, &cache).await;
        let result = cache.get("bitcoin").await;
        let expected = PriceData::new("bitcoin".to_string(), "8426.69".to_string(), 1609459200);
        assert_eq!(result.unwrap(), expected);
    }

    #[tokio::test]
    async fn test_update_coin_list() {
        let (rest_api, _, mut server) = setup();
        let coin_list_store = Arc::new(RwLock::new(HashSet::<String>::new()));
        let coin_list = vec![Coin {
            id: "bitcoin".to_string(),
            symbol: "BTC".to_string(),
            name: "Bitcoin".to_string(),
        }];
        server.set_successful_coin_list(&coin_list);

        update_coin_list(&rest_api, &coin_list_store).await;
        assert!(coin_list_store.read().await.contains("bitcoin"));
    }

    #[tokio::test]
    async fn test_process_market_data() {
        let cache = Arc::new(Cache::<PriceData>::new(None));
        let market = Market {
            id: "bitcoin".to_string(),
            symbol: "BTC".to_string(),
            name: "Bitcoin".to_string(),
            current_price: 8426.69,
            last_updated: "2021-01-01T00:00:00.000Z".to_string(),
        };

        cache.set_batch_pending(vec!["bitcoin".to_string()]).await;
        process_market_data(&market, &cache).await;
        let result = cache.get("bitcoin").await;
        let expected = PriceData::new("bitcoin".to_string(), "8426.69".to_string(), 1609459200);
        assert_eq!(result.unwrap(), expected);
    }

    #[tokio::test]
    async fn test_process_market_data_without_set_pending() {
        let cache = Arc::new(Cache::<PriceData>::new(None));
        let market = Market {
            id: "bitcoin".to_string(),
            symbol: "BTC".to_string(),
            name: "Bitcoin".to_string(),
            current_price: 8426.69,
            last_updated: "2021-01-01T00:00:00.000Z".to_string(),
        };

        process_market_data(&market, &cache).await;
        let result = cache.get("bitcoin").await;
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_market() {
        let market = Market {
            id: "bitcoin".to_string(),
            symbol: "BTC".to_string(),
            name: "Bitcoin".to_string(),
            current_price: 8426.69,
            last_updated: "2021-01-01T00:00:00.000Z".to_string(),
        };
        let result = parse_market(&market);
        let expected = PriceData::new("bitcoin".to_string(), "8426.69".to_string(), 1609459200);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_parse_market_with_failure() {
        let market = Market {
            id: "bitcoin".to_string(),
            symbol: "BTC".to_string(),
            name: "Bitcoin".to_string(),
            current_price: 8426.69,
            last_updated: "johnny appleseed".to_string(),
        };
        assert!(parse_market(&market).is_err());
    }
}
