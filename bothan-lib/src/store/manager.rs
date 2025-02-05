use crate::registry::{Registry, Valid};
use crate::store::Store;

#[derive(Clone)]
pub struct ManagerStore<S: Store> {
    store: S,
}

impl<S: Store> ManagerStore<S> {
    pub fn new(store: S) -> Self {
        Self { store }
    }

    pub async fn set_registry(
        &self,
        registry: Registry<Valid>,
        hash: String,
    ) -> Result<(), S::Error> {
        self.store.set_registry(registry, hash).await
    }

    pub async fn get_registry(&self) -> Registry<Valid> {
        self.store.get_registry().await
    }

    pub async fn get_registry_ipfs_hash(&self) -> Result<Option<String>, S::Error> {
        self.store.get_registry_ipfs_hash().await
    }
}
