use crate::store::Store;
use crate::types::AssetState;
use error::AssetWorkerError;

pub mod error;
pub mod rest;

/// The universal trait for all workers that provide asset info.
#[async_trait::async_trait]
pub trait AssetWorker<S: Store>: Send + Sync + Sized {
    type Opts;

    fn name(&self) -> &'static str;
    async fn build(opts: Self::Opts, store: &S) -> Result<Self, AssetWorkerError>;
    async fn get_asset(&self, id: &str) -> Result<AssetState, AssetWorkerError>;
    async fn set_query_ids(&self, ids: Vec<String>) -> Result<(), AssetWorkerError>;
}
