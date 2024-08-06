use std::collections::HashSet;

use tracing::{error, info};

use crate::registry::Registry;
use crate::store::types::Key;
use crate::store::SharedStore;

pub struct ManagerStore {
    store: SharedStore,
}

impl ManagerStore {
    pub fn new(store: SharedStore) -> Self {
        Self { store }
    }

    pub async fn set_active_signal_ids(&self, signal_ids: Vec<String>) {
        let new_active_set = signal_ids.into_iter().collect::<HashSet<String>>();
        self.store.set_active_signal_ids(new_active_set).await;

        let key = Key::ActiveSignalIDs;
        match self.store.save_state(&key).await {
            Ok(_) => info!("saved {} state successfully", key),
            Err(e) => error!("failed to save {} state: {}", key, e),
        }
    }

    pub async fn get_active_signal_ids(&self) -> HashSet<String> {
        self.store.get_active_signal_ids().await
    }

    pub async fn set_registry(&self, registry: Registry) {
        self.store.set_registry(registry).await;

        let key = Key::Registry;
        match self.store.save_state(&key).await {
            Ok(_) => info!("saved {} state successfully", key),
            Err(e) => error!("failed to save {} state: {}", key, e),
        }
    }

    pub async fn get_registry(&self) -> Registry {
        self.store.get_registry().await
    }
}
