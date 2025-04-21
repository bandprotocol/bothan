use crate::store::Store;
use crate::types::AssetInfo;

/// A store wrapper for worker-specific operations with namespace isolation.
///
/// `WorkerStore` provides a simplified interface for workers to interact with the main store,
/// automatically applying a consistent namespace prefix to all operations. This ensures that
/// different workers can operate on isolated sections of the store without conflicts.
#[derive(Clone)]
pub struct WorkerStore<S: Store> {
    store: S,
    prefix: String,
}

impl<S: Store> WorkerStore<S> {
    /// Creates a new WorkerStore with the specified store and unique prefix key.
    ///
    /// The prefix serves as a namespace that is automatically applied to all store operations,
    /// isolating this worker's data from other parts of the system.
    pub fn new<T: Into<String>>(store: &S, prefix: T) -> Self {
        Self {
            store: store.clone(),
            prefix: prefix.into(),
        }
    }

    /// Sets the asset information for a specific asset.
    ///
    /// This method automatically applies the worker's namespace prefix when
    /// storing the asset information in the underlying store.
    ///
    /// # Errors
    ///
    /// Returns the underlying store's error type if the operation fails
    pub async fn set_asset_info(&self, asset: AssetInfo) -> Result<(), S::Error> {
        self.store.insert_asset_info(&self.prefix, asset).await
    }

    /// Sets multiple asset information entries in a single batch operation.
    ///
    /// This method provides an optimized way to store multiple asset entries at once,
    /// while automatically applying the worker's namespace prefix.
    ///
    /// # Errors
    ///
    /// Returns the underlying store's error type if the operation fails
    pub async fn set_batch_asset_info(&self, assets: Vec<AssetInfo>) -> Result<(), S::Error> {
        self.store
            .insert_batch_asset_info(&self.prefix, assets)
            .await
    }
}
