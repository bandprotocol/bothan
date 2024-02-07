use std::collections::hash_map::{Entry, HashMap};
use std::ops::Sub;
use std::sync::Arc;

use tokio::select;
use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, MutexGuard};
use tokio::time::{interval, Instant};
use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::cache::error::Error;
use crate::cache::types::{StoredPriceData, DEFAULT_EVICTION_CHECK_INTERVAL, DEFAULT_TIMEOUT};
use crate::types::PriceData;

type Map = HashMap<String, Option<StoredPriceData>>;
type Store = Mutex<Map>;

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
        let store: Arc<Store> = Arc::new(Mutex::new(HashMap::new()));
        let token = CancellationToken::new();

        start_eviction_process(store.clone(), token.clone(), sender);

        Self { store, token }
    }

    pub async fn set_pending(&self, id: String) -> Result<(), Error> {
        match self.store.lock().await.entry(id.to_ascii_lowercase()) {
            Entry::Occupied(_) => Err(Error::AlreadySet),
            Entry::Vacant(entry) => {
                entry.insert(None);
                Ok(())
            }
        }
    }

    pub async fn set_data(&self, id: String, data: PriceData) -> Result<(), Error> {
        match self.store.lock().await.entry(id.to_ascii_lowercase()) {
            Entry::Occupied(mut entry) => {
                match entry.get_mut() {
                    Some(stored) => {
                        stored.update(data);
                    }
                    None => {
                        entry.insert(Some(StoredPriceData::new(data)));
                    }
                };
                Ok(())
            }
            Entry::Vacant(_) => Err(Error::PendingNotSet),
        }
    }

    pub async fn get(&self, id: &str) -> Result<PriceData, Error> {
        get_value(id, &mut self.store.lock().await)
    }

    pub async fn get_batch(&self, ids: &[&str]) -> Vec<Result<PriceData, Error>> {
        let mut locked_map = self.store.lock().await;
        ids.iter()
            .map(|id| get_value(id, &mut locked_map))
            .collect()
    }

    pub async fn keys(&self) -> Vec<String> {
        self.store
            .lock()
            .await
            .iter()
            .map(|(k, _)| k.clone())
            .collect()
    }
}

fn start_eviction_process(
    store: Arc<Store>,
    token: CancellationToken,
    sender: Sender<Vec<String>>,
) {
    tokio::spawn(async move {
        let mut interval = interval(DEFAULT_EVICTION_CHECK_INTERVAL);
        loop {
            select! {
                _ = interval.tick() => {
                    remove_timed_out_data(&store, &sender).await;
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

async fn remove_timed_out_data(store: &Store, sender: &Sender<Vec<String>>) {
    let mut locked_map = store.lock().await;
    let mut evicted_keys = Vec::new();

    // Remove entries that needs to be evicted and collect their keys into a vec
    // to be sent to the sender to unsubscribe
    locked_map.retain(|k, v| {
        if let Some(price_data) = v {
            if is_timed_out(price_data.last_used) {
                evicted_keys.push(k.clone());
                return false;
            }
        }
        true
    });

    if !evicted_keys.is_empty() {
        info!("evicting timed out symbols: {:?}", evicted_keys);
        let _res = sender.send(evicted_keys).await;
    }
}

fn get_value(id: &str, locked_map: &mut MutexGuard<Map>) -> Result<PriceData, Error> {
    match locked_map.entry(id.to_ascii_lowercase()) {
        Entry::Occupied(mut entry) => match entry.get_mut() {
            Some(stored) => {
                stored.bump_last_used();
                Ok(stored.data.clone())
            }
            None => Err(Error::Invalid),
        },
        Entry::Vacant(_) => Err(Error::DoesNotExist),
    }
}
