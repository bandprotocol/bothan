use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;

use alloy::eips::BlockId;
use alloy::providers::{Network, Provider};
use alloy::rpc::types::eth::Block;
use alloy::transports::Transport;
use tokio::time::Interval;
use tracing::{debug, warn};

use bothan_core::cache::{Cache, Error as CacheError};
use bothan_core::service::{Error as ServiceError, Service, ServiceResult};
use bothan_core::types::PriceData;

use crate::contract::v3::get_spot_price;

pub mod builder;
pub mod error;
pub mod types;

pub struct UniswapV3Service<P: Provider<T, N>, T: Transport + Clone, N: Network> {
    // NOTE: Keeping provider for future use
    _provider: P,
    transport: PhantomData<T>,
    network: PhantomData<fn() -> N>,
    cache: Arc<Cache<PriceData>>,
}

impl<P: Provider<T, N>, T: Transport + Clone, N: Network> UniswapV3Service<P, T, N> {
    pub async fn new(provider: P, update_interval: Duration, inverse: bool) -> Self {
        let cache = Arc::new(Cache::new(None));

        start_service(
            provider.root().clone(),
            cache.clone(),
            tokio::time::interval(update_interval),
            inverse,
        )
        .await;

        Self {
            _provider: provider,
            cache,
            transport: Default::default(),
            network: Default::default(),
        }
    }
}

#[async_trait::async_trait]
impl<P: Provider<T, N> + 'static, T: Transport + Clone, N: Network> Service
    for UniswapV3Service<P, T, N>
{
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
                    Err(ServiceError::PendingResult)
                }
                Err(CacheError::Invalid) => Err(ServiceError::InvalidSymbol),
                Err(e) => panic!("unexpected error: {}", e), // This should never happen
            })
            .collect();

        if !to_set_pending.is_empty() {
            self.cache.set_batch_pending(to_set_pending).await;
        }

        result
    }
}

async fn start_service<P: Provider<T, N> + 'static, T: Transport + Clone, N: Network>(
    provider: P,
    cache: Arc<Cache<PriceData>>,
    mut update_price_interval: Interval,
    inverse: bool,
) {
    tokio::spawn(async move {
        loop {
            update_price_interval.tick().await;
            if let Ok(Some(block)) = provider.get_block(BlockId::latest(), false).await {
                for key in cache.keys().await {
                    tokio::spawn(update_id(
                        provider.root().clone(),
                        cache.clone(),
                        key,
                        block.clone(),
                        inverse,
                    ));
                }
            } else {
                warn!("failed to get block number")
            }
        }
    });
}

async fn update_id<P: Provider<T, N>, T: Transport + Clone, N: Network>(
    provider: P,
    cache: Arc<Cache<PriceData>>,
    addr: String,
    block: Block,
    inverse: bool,
) {
    if let Some(block_number) = block.header.number {
        let block_id = BlockId::number(block_number);
        let timestamp = block.header.timestamp;

        if let Ok(price) = get_spot_price(provider, &addr, Some(block_id)).await {
            let price_data = match inverse {
                true => PriceData {
                    id: addr.clone(),
                    price: (1.0 / price).to_string(),
                    timestamp,
                },
                false => PriceData {
                    id: addr.clone(),
                    price: price.to_string(),
                    timestamp,
                },
            };

            if cache.set_data(addr, price_data).await.is_err() {
                warn!("failed to set data")
            } else {
                debug!("updated price data")
            }
        } else {
            warn!("failed to get contract price")
        }
    } else {
        warn!("block not finalized")
    }
}
