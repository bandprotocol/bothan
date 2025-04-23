use error::AssetWorkerError;

use crate::store::Store;

pub mod error;
pub mod rest;
pub mod websocket;

/// The universal trait for all workers that poll and update asset info.
#[async_trait::async_trait]
pub trait AssetWorker: Send + Sync + Sized {
    type Opts;

    /// The name of the worker.
    /// When used with a registry, it will match the name specified in the registry.
    fn name(&self) -> &'static str;

    /// Build the worker with the given options and query ids.
    async fn build<S: Store + 'static>(
        opts: Self::Opts,
        store: &S,
        ids: Vec<String>,
    ) -> Result<Self, AssetWorkerError>;
}
