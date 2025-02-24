use crate::registry::{Registry, Valid};
use crate::store::Store;

#[derive(Clone)]
pub struct RegistryStore<S: Store> {
    store: S,
}

impl<S: Store> RegistryStore<S> {
    /// Creates a new RegistryStore from an existing store.
    pub fn new(store: S) -> Self {
        Self { store }
    }

    /// Sets the current registry and its hash.
    pub async fn set_registry(
        &self,
        registry: Registry<Valid>,
        hash: String,
    ) -> Result<(), S::Error> {
        self.store.set_registry(registry, hash).await
    }

    /// Gets the current registry.
    pub async fn get_registry(&self) -> Registry<Valid> {
        self.store.get_registry().await
    }

    /// Gets the current registry hash.
    pub async fn get_registry_ipfs_hash(&self) -> Result<Option<String>, S::Error> {
        self.store.get_registry_ipfs_hash().await
    }
}
