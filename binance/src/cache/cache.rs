use std::ops::Sub;
use std::sync::Arc;

use crate::cache::error::Error;
use dashmap::{DashMap, DashSet};
use tokio::select;
use tokio::sync::mpsc::Sender;
use tokio::time::{interval, Duration, Instant};
use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::types::PriceData;

// TODO: move out into its own crate
pub const DEFAULT_TIMEOUT: u64 = 6000;

#[derive(Debug, Clone)]
pub struct StoredPriceData {
    pub data: PriceData,
    pub last_used: Instant,
}

pub struct Cache {
    price_map: Arc<DashMap<String, StoredPriceData>>,
    subscription_map: Arc<DashSet<String>>,
    token: CancellationToken,
}

impl Drop for Cache {
    fn drop(&mut self) {
        self.token.cancel()
    }
}

impl Cache {
    pub fn new(sender: Sender<Vec<String>>) -> Self {
        let price_map = Arc::new(DashMap::<String, StoredPriceData>::new());
        let subscription_map = Arc::new(DashSet::<String>::new());
        let token = CancellationToken::new();

        let cloned_token = token.clone();
        let cloned_price_map = price_map.clone();
        let cloned_subscription_map = subscription_map.clone();

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));
            loop {
                select! {
                    _ = interval.tick() => {
                        let keys = cloned_price_map.iter().filter_map(|r| {
                            let stored = r.value();
                            if let Some((k, _)) = cloned_price_map.remove_if(&stored.data.id, |_, v| check_timeout(stored.last_used)) {
                                cloned_subscription_map.remove(&k);
                                Some(k)
                            } else {
                                None
                            }
                        }).collect::<Vec<String>>();
                        info!("Removing unused keys: {:?}", keys);
                        let _res = sender.send(keys).await;
                    }
                    _ = cloned_token.cancelled() => {
                        break
                    }
                }
            }
        });

        Self {
            price_map,
            subscription_map,
            token,
        }
    }

    pub fn set_pending(&self, id: String) {
        self.subscription_map.insert(id.to_ascii_lowercase());
    }

    pub fn set_data(&self, id: String, data: PriceData) {
        if let Some((k, mut v)) = self.price_map.remove(&id) {
            v.data = data;
            self.price_map.insert(k, v);
        } else {
            self.price_map.insert(
                id.to_ascii_lowercase(),
                StoredPriceData {
                    data,
                    last_used: Instant::now(),
                },
            );
        }
    }

    pub fn get(&self, id: &str) -> Result<PriceData, Error> {
        if let Some(r) = self.price_map.get(&id.to_ascii_lowercase()) {
            Ok(r.data.clone())
        } else {
            if let Some(_) = self.subscription_map.get(&id.to_ascii_lowercase()) {
                Err(Error::Invalid)
            } else {
                Err(Error::DoesNotExist)
            }
        }
    }
}

fn check_timeout(last_used: Instant) -> bool {
    Instant::now().sub(last_used) > Duration::from_secs(DEFAULT_TIMEOUT)
}
