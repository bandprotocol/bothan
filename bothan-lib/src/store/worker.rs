use crate::store::Store;
use crate::types::AssetInfo;

#[derive(Clone)]
pub struct WorkerStore<S: Store> {
    store: S,
    prefix: String,
}

impl<S: Store> WorkerStore<S> {
    /// Creates a new WorkerStore with the specified store and unique prefix key.
    pub fn new<T: Into<String>>(store: &S, prefix: T) -> Self {
        Self {
            store: store.clone(),
            prefix: prefix.into(),
        }
    }

    /// Set the asset state for the specified query id.
    pub async fn set_asset_info(&self, asset: AssetInfo) -> Result<(), S::Error> {
        self.store.insert_asset_info(&self.prefix, asset).await
    }

    /// Sets multiple asset states for the specified query ids.
    pub async fn set_batch_asset_info(&self, assets: Vec<AssetInfo>) -> Result<(), S::Error> {
        self.store
            .insert_batch_asset_info(&self.prefix, assets)
            .await
    }
}
