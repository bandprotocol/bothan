use crate::types::AssetInfo;

#[derive(Debug)]
pub enum AssetStatus {
    Unsupported,
    Pending,
    Available(AssetInfo),
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Not started")]
    NotStarted,

    #[error("failed to modify query IDs: {0}")]
    ModifyQueryIDsFailed(String),
}

/// The universal trait for all workers that provide asset info.
#[async_trait::async_trait]
pub trait AssetWorker {
    async fn get_assets<K: AsRef<str> + Send + Sync>(&self, ids: &[K]) -> Vec<AssetStatus>;
    async fn add_query_ids<K: Into<String> + Send + Sync>(&self, ids: Vec<K>) -> Result<(), Error>;
    async fn remove_query_ids<K: AsRef<str> + Send + Sync>(&self, ids: &[K]) -> Result<(), Error>;
    async fn get_query_ids(&self) -> Vec<String>;
}
