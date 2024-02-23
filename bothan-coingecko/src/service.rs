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
        page_size: usize,
        page_query_delay: Option<Duration>,
    ) -> Self {
        let cache = Arc::new(Cache::new(None));
        let coin_list = Arc::new(RwLock::new(HashSet::<String>::new()));
        let update_price_interval = interval(update_interval);
        let update_supported_assets_interval = interval(update_supported_assets_interval);

        start_service(
            Arc::new(rest_api),
            cache.clone(),
            update_price_interval,
            update_supported_assets_interval,
            coin_list.clone(),
            page_size,
            page_query_delay,
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
                Err(CacheError::Invalid) => Err(ServiceError::InvalidSymbol),
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
    rest_api: Arc<CoinGeckoRestAPI>,
    cache: Arc<Cache<PriceData>>,
    mut update_price_interval: Interval,
    mut update_supported_assets_interval: Interval,
    coin_list: Arc<RwLock<HashSet<String>>>,
    page_size: usize,
    page_query_delay: Option<Duration>,
) {
    update_coin_list(&rest_api, &coin_list).await;

    tokio::spawn(async move {
        loop {
            select! {
                _ = update_price_interval.tick() => {
                    update_price_data(&rest_api, &cache, page_size, page_query_delay).await;
                },
                _ = update_supported_assets_interval.tick() => {
                    update_coin_list(&rest_api, &coin_list).await;
                },
            }
        }
    });
}

async fn update_price_data(
    rest_api: &Arc<CoinGeckoRestAPI>,
    cache: &Arc<Cache<PriceData>>,
    page_size: usize,
    page_query_delay: Option<Duration>,
) {
    let mut delay = page_query_delay.map(interval);

    let pages = {
        let keys = cache.keys().await;
        keys.len().div_ceil(page_size)
    };

    for page in 1..=pages {
        if let Some(interval) = delay.as_mut() {
            interval.tick().await;
        }

        let cloned_api = rest_api.clone();
        let cloned_cache = cache.clone();
        tokio::spawn(async move {
            update_price_data_from_api(&cloned_api, &cloned_cache, page, page_size).await;
        });
    }
}

async fn update_price_data_from_api(
    rest_api: &Arc<CoinGeckoRestAPI>,
    cache: &Arc<Cache<PriceData>>,
    page: usize,
    page_size: usize,
) {
    let keys = cache.keys().await;
    let ids = keys.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
    if let Ok(markets) = rest_api
        .get_coins_market(ids.as_slice(), page_size, page)
        .await
    {
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

async fn update_coin_list(
    rest_api: &Arc<CoinGeckoRestAPI>,
    coin_list: &Arc<RwLock<HashSet<String>>>,
) {
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