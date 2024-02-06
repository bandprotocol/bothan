use std::collections::HashMap;
use std::ops::Sub;
use std::sync::Arc;

use tokio::select;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use tokio::time::{interval, Instant};
use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::cache::error::Error;
use crate::cache::types::{StoredPriceData, DEFAULT_CLEANUP_INTERVAL, DEFAULT_TIMEOUT};
use crate::types::PriceData;

type Store = RwLock<HashMap<String, Option<StoredPriceData>>>;

pub struct Cache {
    store: Arc<Store>,
    token: CancellationToken,
}

impl Drop for Cache {
    fn drop(&mut self) {
        self.token.cancel()
    }
}

impl Cache {
    pub fn new(sender: Sender<Vec<String>>) -> Self {
        let store: Arc<Store> = Arc::new(RwLock::new(HashMap::new()));
        let token = CancellationToken::new();

        start_cleanup_process(store.clone(), token.clone(), sender);

        Self { store, token }
    }

    pub async fn set_pending(&self, id: String) -> Result<(), Error> {
        let mut writer = self.store.write().await;
        if writer.contains_key(&id.to_ascii_lowercase()) {
            writer.insert(id.to_ascii_lowercase(), None);
            Ok(())
        } else {
            Err(Error::AlreadySet)
        }
    }

    pub async fn set_data(&self, id: String, data: PriceData) {
        let mut writer = self.store.write().await;
        let to_store = match writer.get(&id.to_ascii_lowercase()) {
            Some(Some(stored_data)) => StoredPriceData {
                data,
                last_used: stored_data.last_used,
            },
            _ => StoredPriceData {
                data,
                last_used: Instant::now(),
            },
        };
        writer.insert(id.to_ascii_lowercase(), Some(to_store));
    }

    pub async fn get(&self, id: &str) -> Result<PriceData, Error> {
        let reader = self.store.read().await;

        match reader.get(&id.to_ascii_lowercase()) {
            Some(Some(r)) => Ok(r.data.clone()),
            Some(None) => Err(Error::Invalid),
            None => Err(Error::DoesNotExist),
        }
    }

    pub async fn keys(&self) -> Vec<String> {
        self.store
            .read()
            .await
            .iter()
            .map(|(k, _)| k.clone())
            .collect()
    }
}

fn start_cleanup_process(store: Arc<Store>, token: CancellationToken, sender: Sender<Vec<String>>) {
    tokio::spawn(async move {
        let mut interval = interval(DEFAULT_CLEANUP_INTERVAL);
        loop {
            select! {
                _ = interval.tick() => {
                    remove_timeout_data(&store, &sender).await;
                }
                _ = token.cancelled() => {
                    break
                }
            }
        }
    });
}

fn is_timed_out(last_used: Instant) -> bool {
    Instant::now().sub(last_used) > DEFAULT_TIMEOUT
}

async fn remove_timeout_data(store: &Store, sender: &Sender<Vec<String>>) {
    let reader = store.read().await;
    let keys = reader
        .iter()
        .filter_map(|(k, v)| {
            if let Some(price_data) = v {
                if is_timed_out(price_data.last_used) {
                    return Some(k.clone());
                }
            }
            None
        })
        .collect::<Vec<String>>();

    if !keys.is_empty() {
        info!("Removing unused keys: {:?}", keys);
        let _res = sender.send(keys).await;
    }
}
