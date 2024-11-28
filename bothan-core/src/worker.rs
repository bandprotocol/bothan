use crate::store::error::Error as StoreError;
use crate::store::WorkerStore;
use crate::types::AssetInfo;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AssetState {
    Unsupported,
    Pending,
    Available(AssetInfo),
}

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
#[error("failed to modify query IDs: {msg}")]
pub struct SetQueryIDError {
    msg: String,
}

impl SetQueryIDError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}

/// The universal trait for all workers that provide asset info.
#[async_trait::async_trait]
pub trait AssetWorker: Send + Sync {
    async fn get_asset(&self, id: &str) -> Result<AssetState, StoreError>;
    async fn set_query_ids(&self, ids: Vec<String>) -> Result<(), SetQueryIDError>;
}

#[async_trait::async_trait]
pub trait AssetWorkerBuilder<'a> {
    type Opts;
    type Worker: AssetWorker + 'a;
    type Error: std::error::Error;

    fn new(store: WorkerStore, opts: Self::Opts) -> Self;

    fn worker_name() -> &'static str;

    async fn build(self) -> Result<Arc<Self::Worker>, Self::Error>;
}
