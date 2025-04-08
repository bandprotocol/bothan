use std::error::Error as StdError;

use async_trait::async_trait;
pub use worker::WorkerStore;

use crate::registry::{Registry, Valid};
use crate::types::AssetInfo;

mod worker;

/// The universal trait for all stores. All implementations must be thread-safe and atomic.
#[async_trait]
pub trait Store: Send + Sync + Clone {
    type Error: StdError + Send + Sync + 'static;

    /// Set the registry in the store.
    async fn set_registry(
        &self,
        registry: Registry<Valid>,
        ipfs_hash: String,
    ) -> Result<(), Self::Error>;
    /// Get the registry in the store.
    async fn get_registry(&self) -> Result<Registry<Valid>, Self::Error>;
    /// Get the IPFS hash of the registry in the store.
    async fn get_registry_ipfs_hash(&self) -> Result<Option<String>, Self::Error>;
    /// Sets the query ids in the store under the given prefix.
    async fn get_asset_info(
        &self,
        prefix: &str,
        id: &str,
    ) -> Result<Option<AssetInfo>, Self::Error>;
    /// Inserts the asset info in the store under the given prefix.
    async fn insert_asset_info(
        &self,
        prefix: &str,
        asset_info: AssetInfo,
    ) -> Result<(), Self::Error>;
    /// Batch inserts the asset info in the store under the given prefix.
    async fn insert_batch_asset_info(
        &self,
        prefix: &str,
        asset_infos: Vec<AssetInfo>,
    ) -> Result<(), Self::Error>;
}
