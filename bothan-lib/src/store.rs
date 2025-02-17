use crate::registry::{Registry, Valid};
use crate::types::AssetInfo;
use async_trait::async_trait;
use std::error::Error as StdError;

pub use registry::RegistryStore;
pub use worker::WorkerStore;

mod registry;
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
    async fn get_registry(&self) -> Registry<Valid>;
    /// Get the IPFS hash of the registry in the store.
    async fn get_registry_ipfs_hash(&self) -> Result<Option<String>, Self::Error>;
    /// Sets the query ids in the store under the given prefix.
    async fn set_query_ids(&self, prefix: &str, ids: Vec<String>) -> Result<(), Self::Error>;
    /// Gets the query ids in the store under the given prefix.
    async fn get_query_ids(&self, prefix: &str) -> Result<Option<Vec<String>>, Self::Error>;
    /// Inserts query ids in the store under the given prefix.
    async fn insert_query_ids(&self, prefix: &str, ids: Vec<String>) -> Result<(), Self::Error>;
    /// Removes query ids in the store under the given prefix.
    async fn remove_query_ids(&self, prefix: &str, ids: &[String]) -> Result<(), Self::Error>;
    /// Checks if the store contains the query id under the given prefix.
    async fn contains_query_id(&self, prefix: &str, id: &str) -> Result<bool, Self::Error>;
    /// Gets the asset info in the store under the given prefix.
    async fn get_asset_info(
        &self,
        prefix: &str,
        id: &str,
    ) -> Result<Option<AssetInfo>, Self::Error>;
    /// Inserts the asset info in the store under the given prefix.
    async fn insert_asset_info(
        &self,
        prefix: &str,
        asset_info: (String, AssetInfo),
    ) -> Result<(), Self::Error>;
    /// Batch inserts the asset info in the store under the given prefix.
    async fn insert_asset_info_batch(
        &self,
        prefix: &str,
        asset_infos: Vec<(String, AssetInfo)>,
    ) -> Result<(), Self::Error>;
}
