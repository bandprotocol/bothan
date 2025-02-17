use crate::store::Store;
use crate::types::AssetState;
use error::AssetWorkerError;

pub mod error;
pub mod rest;

/// The universal trait for all workers that provide asset info.
#[async_trait::async_trait]
pub trait AssetWorker<S: Store>: Send + Sync + Sized {
    type Opts;

    /// The name of the worker.
    fn name(&self) -> &'static str;
    /// Build the worker with the given options.
    async fn build(opts: Self::Opts, store: &S) -> Result<Self, AssetWorkerError>;
    /// Get the asset info for the given id.
    async fn get_asset(&self, id: &str) -> Result<AssetState, AssetWorkerError>;
    /// Set the query ids for the worker.
    async fn set_query_ids(&self, ids: Vec<String>) -> Result<(), AssetWorkerError>;
}
