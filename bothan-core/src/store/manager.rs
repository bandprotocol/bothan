use crate::registry::{Registry, Valid};
use crate::store::error::Error;
use crate::store::SharedStore;

pub struct ManagerStore {
    store: SharedStore,
}

impl ManagerStore {
    pub fn new(store: SharedStore) -> Self {
        Self { store }
    }

    pub async fn set_registry(&self, registry: Registry<Valid>, hash: String) -> Result<(), Error> {
        self.store.set_registry(registry).await?;
        self.store.set_registry_hash(hash).await
    }

    pub async fn get_registry(&self) -> Registry<Valid> {
        self.store.get_registry().await
    }

    pub async fn get_registry_hash(&self) -> Result<Option<String>, Error> {
        self.store.get_registry_hash().await
    }
}
