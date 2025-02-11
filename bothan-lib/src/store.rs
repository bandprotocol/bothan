use crate::registry::{Registry, Valid};
use crate::types::AssetInfo;
use async_trait::async_trait;
use std::error::Error as StdError;

pub use manager::ManagerStore;
pub use worker::WorkerStore;

mod manager;
mod worker;

// Note: All implementations here should be atomic
#[async_trait]
pub trait Store: Send + Sync + Clone {
    type Error: StdError + Send + Sync + 'static;

    async fn set_registry(
        &self,
        registry: Registry<Valid>,
        ipfs_hash: String,
    ) -> Result<(), Self::Error>;
    async fn get_registry(&self) -> Registry<Valid>;
    async fn get_registry_ipfs_hash(&self) -> Result<Option<String>, Self::Error>;
    async fn set_query_ids(&self, source_id: &str, ids: Vec<String>) -> Result<(), Self::Error>;
    async fn get_query_ids(&self, source_id: &str) -> Result<Option<Vec<String>>, Self::Error>;
    async fn insert_query_ids(&self, source_id: &str, ids: Vec<String>) -> Result<(), Self::Error>;
    async fn remove_query_ids(&self, source_id: &str, ids: &[String]) -> Result<(), Self::Error>;
    async fn contains_query_id(&self, source_id: &str, id: &str) -> Result<bool, Self::Error>;
    async fn get_asset_info(
        &self,
        source_id: &str,
        id: &str,
    ) -> Result<Option<AssetInfo>, Self::Error>;
    async fn insert_asset_info(
        &self,
        source_id: &str,
        asset_info: (String, AssetInfo),
    ) -> Result<(), Self::Error>;
    async fn insert_asset_info_batch(
        &self,
        source_id: &str,
        asset_infos: Vec<(String, AssetInfo)>,
    ) -> Result<(), Self::Error>;
}
