use crate::types::AssetInfo;

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

/// Type alias for a service result, which is either a valid result or an error.
/// The universal trait for all services that provide price data.
#[async_trait::async_trait]
pub trait AssetStore: Send + Sync + 'static {
    async fn start(&mut self);
    async fn get_asset(&self, ids: &[&str]) -> Vec<AssetStatus>;
    async fn add_query_ids(&mut self, ids: &[&str]) -> Result<(), Error>;
    async fn remove_query_ids(&mut self, ids: &[&str]) -> Result<(), Error>;
    async fn get_query_ids(&self) -> Vec<String>;
}
