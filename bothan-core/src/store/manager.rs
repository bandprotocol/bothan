use std::collections::HashSet;

use tracing::{error, info};

use crate::registry::Registry;
use crate::store::Store;

pub struct ManagerStore {
    store: Store,
}

impl ManagerStore {
    pub fn new(store: Store) -> Self {
        Self { store }
    }

    pub async fn set_active_signal_ids(&self, signal_ids: Vec<String>) {
        let mut store = self.store.inner.lock().await;
        let new_active_set = signal_ids.into_iter().collect::<HashSet<String>>();

        let save_result = self
            .store
            .save_state("active_signal_ids", &store.active_signal_ids)
            .await;

        match save_result {
            Ok(_) => info!("active_signal_ids state saved successfully"),
            Err(e) => error!("failed to save active_signal_ids state: {}", e),
        }
        store.active_signal_ids = new_active_set;
    }

    pub async fn get_active_signal_ids(&self) -> HashSet<String> {
        self.store.inner.lock().await.active_signal_ids.clone()
    }

    pub async fn set_registry(&self, registry: Registry) {
        let mut store = self.store.inner.lock().await;

        let save_result = self.store.save_state("registry", &store.registry).await;

        match save_result {
            Ok(_) => info!("registry state saved successfully"),
            Err(e) => error!("failed to save registry state: {}", e),
        }
        store.registry = registry;
    }

    pub async fn get_registry(&self) -> Registry {
        self.store.inner.lock().await.registry.clone()
    }
}
