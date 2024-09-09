use std::sync::Arc;

use tokio::time::Duration;

use bothan_core::store::WorkerStore;
use bothan_core::worker::AssetWorkerBuilder;

use crate::api::error::BuildError;
use crate::api::HtxRestAPIBuilder;
use crate::worker::asset_worker::start_asset_worker;
use crate::worker::opts::HtxWorkerBuilderOpts;
use crate::worker::HtxWorker;

/// Builds a `HtxWorker` with custom options.
/// Methods can be chained to set the configuration values and the
/// service is constructed by calling the [`build`](HtxWorker::build) method.
pub struct HtxWorkerBuilder {
    store: WorkerStore,
    opts: HtxWorkerBuilderOpts,
}

impl HtxWorkerBuilder {
    /// Set the URL for the `HtxWorker`.
    /// The default URL is `DEFAULT_URL` when no API key is provided
    /// and is `DEFAULT_PRO_URL` when an API key is provided.
    pub fn with_url<T: Into<String>>(mut self, url: T) -> Self {
        self.opts.url = url.into();
        self
    }

    /// Sets the update interval for the `HtxWorker`.
    /// The default interval is `DEFAULT_UPDATE_INTERVAL`.
    pub fn with_update_interval(mut self, update_interval: Duration) -> Self {
        self.opts.update_interval = update_interval;
        self
    }

    /// Sets the store for the `HtxWorker`.
    /// If not set, the store is created and owned by the worker.
    pub fn with_store(mut self, store: WorkerStore) -> Self {
        self.store = store;
        self
    }
}

#[async_trait::async_trait]
impl<'a> AssetWorkerBuilder<'a> for HtxWorkerBuilder {
    type Opts = HtxWorkerBuilderOpts;
    type Worker = HtxWorker;
    type Error = BuildError;

    /// Returns a new `HtxWorkerBuilder` with the given options.
    fn new(store: WorkerStore, opts: Self::Opts) -> Self {
        Self { store, opts }
    }

    /// Returns the name of the worker.
    fn worker_name() -> &'static str {
        "htx"
    }

    /// Creates the configured `HtxWorker`.
    async fn build(self) -> Result<Arc<Self::Worker>, Self::Error> {
        let api = HtxRestAPIBuilder::new(self.opts.url).build()?;

        let worker = Arc::new(HtxWorker::new(api, self.store));

        start_asset_worker(Arc::downgrade(&worker), self.opts.update_interval);

        Ok(worker)
    }
}
