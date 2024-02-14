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
use crate::api::CoinGeckoRest;
use crate::error::Error;

pub struct CoinGecko {
    cache: Arc<Cache<PriceData>>,
    coin_list: Arc<RwLock<HashSet<String>>>,
}

impl CoinGecko {
    pub async fn new() -> Result<Self, Error> {
        let cache = Arc::new(Cache::new(None));
        let cg = CoinGeckoRest::default()?;
        let coin_list = Arc::new(RwLock::new(HashSet::<String>::new()));
        let update_price_interval = interval(Duration::from_secs(60));
        let update_list_interval = interval(Duration::from_secs(86400));

        start_service(
            update_price_interval,
            update_list_interval,
            cache.clone(),
            cg,
            coin_list.clone(),
        )
        .await;

        Ok(Self { cache, coin_list })
    }
}

#[async_trait::async_trait]
impl Service for CoinGecko {
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
    mut update_price_interval: Interval,
    mut update_list_interval: Interval,
    cache: Arc<Cache<PriceData>>,
    cg: CoinGeckoRest,
    coin_list: Arc<RwLock<HashSet<String>>>,
) {
    tokio::spawn(async move {
        loop {
            select! {
                _ = update_price_interval.tick() => {
                    update_price_data(&cache, &cg).await;
                },
                _ = update_list_interval.tick() => {
                    update_coin_list(&cg, &coin_list).await;
                },
            }
        }
    });
}

async fn update_price_data(cache: &Cache<PriceData>, cg: &CoinGeckoRest) {
    let keys = cache.keys().await;
    let ids = keys.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
    let market_results = cg.get_coins_market(&ids).await;
    for market_result in market_results {
        if let Ok(market) = market_result {
            process_market_data(&market, &cache).await;
        } else {
            warn!("failed to get market data");
        }
    }
}

async fn update_coin_list(cg: &CoinGeckoRest, coin_list: &Arc<RwLock<HashSet<String>>>) {
    if let Ok(new_coin_list) = cg.get_coins_list().await {
        let new_coin_set = HashSet::<String>::from_iter(new_coin_list.into_iter().map(|x| x.id));
        let mut locked = coin_list.write().await;
        *locked = new_coin_set;
    } else {
        warn!("Failed to get coin list");
    }
}

async fn process_market_data(market: &Market, cache: &Cache<PriceData>) {
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
    let naive_date_time =
        NaiveDateTime::parse_from_str(market.last_updated.as_str(), "%Y-%m-%dT%H:%M:%S.%fZ")
            .map_err(|_| Error::InvalidTimestamp)?;
    let timestamp =
        u64::try_from(naive_date_time.timestamp()).map_err(|_| Error::InvalidTimestamp)?;
    Ok(PriceData::new(
        market.id.clone(),
        market.current_price.to_string(),
        timestamp,
    ))
}
