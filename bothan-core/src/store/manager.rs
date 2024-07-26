use std::collections::HashSet;
use std::sync::Arc;

use tracing::error;

use crate::registry::Registry;
use crate::store::Store;

pub struct ManagerStore {
    store: Arc<Store>,
}

impl ManagerStore {
    pub async fn set_active_signal_ids(&self, signal_ids: Vec<String>) {
        let mut active_signal_ids = self.store.active_signal_ids.write().await;
        let new_active_set = signal_ids.into_iter().collect::<HashSet<String>>();

        if let Err(e) = self.store.save_state("active_signal_ids", &new_active_set) {
            error!("failed to save active signal ids state: {}", e);
        }
        *active_signal_ids = new_active_set;
    }

    pub async fn get_active_signal_ids(&self) -> HashSet<String> {
        self.store.active_signal_ids.read().await.clone()
    }

    pub async fn set_registry(&self, registry: Registry) {
        let mut current_registry = self.store.registry.write().await;
        if let Err(e) = self.store.save_state("registry", &registry) {
            error!("failed to save registry state: {}", e);
        }
        *current_registry = registry;
    }

    pub async fn get_registry(&self) -> Registry {
        self.store.registry.read().await.clone()
    }
}

impl From<Arc<Store>> for ManagerStore {
    fn from(store: Arc<Store>) -> Self {
        Self { store }
    }
}
