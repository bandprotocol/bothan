use std::sync::Arc;

use crate::store::WorkerStore;
use crate::types::AssetInfo;

#[derive(Clone, Debug, PartialEq)]
pub enum AssetState {
    Unsupported,
    Pending,
    Available(AssetInfo),
}

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("Not started")]
    NotStarted,

    #[error("failed to modify query IDs: {0}")]
    ModifyQueryIDsFailed(String),
}

/// The universal trait for all workers that provide asset info.
#[async_trait::async_trait]
pub trait AssetWorker: Send + Sync {
    async fn get_assets(&self, ids: &[&str]) -> Vec<AssetState>;
    async fn add_query_ids(&self, ids: Vec<String>) -> Result<(), Error>;
    async fn remove_query_ids(&self, ids: Vec<String>) -> Result<(), Error>;
}

#[async_trait::async_trait]
pub trait AssetWorkerBuilder<'a> {
    type Opts;
    type Worker: AssetWorker + 'a;
    type Error: std::error::Error;

    fn new(store: WorkerStore, opts: Self::Opts) -> Self;

    async fn build(self) -> Result<Arc<Self::Worker>, Self::Error>;
}
