use std::collections::HashMap;

use tokio::sync::RwLock;

use crate::proto::query::query::PriceOption;

pub(crate) type SourceResultsStore = ResultsStore<f64>;
pub(crate) type SignalResultsStore = ResultsStore<Result<f64, PriceOption>>;

pub(crate) struct ResultsStore<T: Clone> {
    store: RwLock<HashMap<String, T>>,
}

impl<T: Clone> ResultsStore<T> {
    pub(crate) fn new() -> Self {
        Self {
            store: RwLock::new(HashMap::new()),
        }
    }

    pub(crate) async fn get<K: AsRef<str>>(&self, key: &K) -> Option<T> {
        let reader = self.store.read().await;
        reader.get(key.as_ref()).cloned()
    }

    pub(crate) async fn get_batched<K: AsRef<str>>(&self, keys: &[K]) -> Vec<Option<T>> {
        let reader = self.store.read().await;
        keys.iter()
            .map(|k| reader.get(k.as_ref()).cloned())
            .collect()
    }

    pub(crate) async fn set<K: Into<String>>(&self, key: K, value: T) {
        let mut writer = self.store.write().await;
        writer.insert(key.into(), value);
    }

    pub(crate) async fn set_batched<K: Into<String>>(&self, values: Vec<(K, T)>) {
        let mut writer = self.store.write().await;
        for (k, v) in values {
            writer.insert(k.into(), v);
        }
    }
}
