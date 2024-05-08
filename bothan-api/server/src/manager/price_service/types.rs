/// This module contains types and structures used in the price service manager.
use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::{Mutex, RwLock};

use crate::proto::query::query::PriceStatus;

/// Type alias for a store of results from a source.
pub(crate) type SourceResultsStore = ResultsStore<f64>;

/// Type alias for a store of results from a signal.
pub(crate) type SignalResultsStore = ResultsStore<Result<f64, PriceStatus>>;

/// Type alias for a map of services.
pub(crate) type ServiceMap<T> = HashMap<String, Arc<Mutex<T>>>;

pub(crate) struct ResultsStore<T: Clone> {
    store: RwLock<HashMap<String, T>>,
}

impl<T: Clone> ResultsStore<T> {
    /// Creates a new `ResultsStore`.
    pub(crate) fn new() -> Self {
        Self {
            store: RwLock::new(HashMap::new()),
        }
    }

    /// Returns the value of the key in the cache or `None` if it is not present in the cache
    pub(crate) async fn get<K: AsRef<str>>(&self, key: &K) -> Option<T> {
        let reader = self.store.read().await;
        reader.get(key.as_ref()).cloned()
    }

    /// Returns a vector of values of the given keys from the cache. If a key is not present in the
    /// cache, the corresponding value in the vector will be `None`.
    pub(crate) async fn get_batched<K: AsRef<str>>(&self, keys: &[K]) -> Vec<Option<T>> {
        let reader = self.store.read().await;
        keys.iter()
            .map(|k| reader.get(k.as_ref()).cloned())
            .collect()
    }

    /// Sets a value in the store by key.
    pub(crate) async fn set<K: Into<String>>(&self, key: K, value: T) {
        let mut writer = self.store.write().await;
        writer.insert(key.into(), value);
    }

    /// Sets multiple values in the store by keys.
    pub(crate) async fn set_batched<K: Into<String>>(&self, values: Vec<(K, T)>) {
        let mut writer = self.store.write().await;
        for (k, v) in values {
            writer.insert(k.into(), v);
        }
    }
}
