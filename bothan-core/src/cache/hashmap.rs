use std::collections::hash_map::{Entry as MapEntry, HashMap};
use std::ops::Sub;
use std::sync::Arc;

use futures::future::join_all;
use tokio::select;
use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, MutexGuard};
use tokio::time::{interval, Instant};
use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::cache::error::Error;
use crate::cache::types::{Entry, DEFAULT_EVICTION_CHECK_INTERVAL, DEFAULT_TIMEOUT};

type Store<T> = HashMap<String, Option<Entry<T>>>;

pub struct Cache<T> {
    store: Arc<Mutex<Store<T>>>,
    token: CancellationToken,
}

impl<T> Drop for Cache<T> {
    fn drop(&mut self) {
        self.token.cancel()
    }
}

// Naive implementation of a cache. Eviction is done on a separate task at a regular interval
// for O(n) key eviction
// TODO: Write a better implementation
impl<T: Send + Clone + 'static> Cache<T> {
    pub fn new(sender: Option<Sender<Vec<String>>>) -> Self {
        let store: Arc<Mutex<Store<T>>> = Arc::new(Mutex::new(HashMap::new()));
        let token = CancellationToken::new();

        start_eviction_process(store.clone(), token.clone(), sender);

        Self { store, token }
    }

    pub async fn set_pending(&self, id: String) {
        self.store
            .lock()
            .await
            .entry(id.to_ascii_lowercase())
            .or_insert(None);
    }

    pub async fn set_batch_pending(&self, ids: Vec<String>) {
        let handles = ids.into_iter().map(|id| self.set_pending(id));
        join_all(handles).await;
    }

    pub async fn set_data(&self, id: String, data: T) -> Result<(), Error> {
        match self.store.lock().await.entry(id.to_ascii_lowercase()) {
            MapEntry::Occupied(mut entry) => {
                match entry.get_mut() {
                    Some(stored) => {
                        stored.update(data);
                    }
                    None => {
                        entry.insert(Some(Entry::new(data)));
                    }
                };
                Ok(())
            }
            MapEntry::Vacant(_) => Err(Error::PendingNotSet),
        }
    }

    pub async fn get(&self, id: &str) -> Result<T, Error> {
        get_value(id, &mut self.store.lock().await)
    }

    pub async fn get_batch(&self, ids: &[&str]) -> Vec<Result<T, Error>> {
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

fn start_eviction_process<T: Send + Clone + 'static>(
    store: Arc<Mutex<Store<T>>>,
    token: CancellationToken,
    sender: Option<Sender<Vec<String>>>,
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

async fn remove_timed_out_data<T: Send + Clone + 'static>(
    store: &Arc<Mutex<Store<T>>>,
    sender: &Option<Sender<Vec<String>>>,
) {
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
        if let Some(sender) = sender {
            // TODO: Handle this
            let _res = sender.send(evicted_keys).await;
        }
    }
}

fn get_value<T: Send + Clone + 'static>(
    id: &str,
    locked_map: &mut MutexGuard<Store<T>>,
) -> Result<T, Error> {
    match locked_map.entry(id.to_ascii_lowercase()) {
        MapEntry::Occupied(mut entry) => match entry.get_mut() {
            Some(stored) => {
                stored.bump_last_used();
                Ok(stored.data.clone())
            }
            None => Err(Error::Invalid),
        },
        MapEntry::Vacant(_) => Err(Error::DoesNotExist),
    }
}
