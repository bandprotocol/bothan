use std::sync::Arc;

use tokio::sync::mpsc::channel;

use crate::api::BybitWebSocketConnector;
use crate::worker::asset_worker::start_asset_worker;
use crate::worker::error::BuildError;
use crate::worker::opts::BybitWorkerBuilderOpts;
use crate::worker::BybitWorker;
use bothan_core::store::WorkerStore;
use bothan_core::worker::AssetWorkerBuilder;

/// Builds a `BybitWorker` with custom options.
/// Methods can be chained to set the configuration values and the
/// service is constructed by calling the [`build`](BybitWorkerBuilder::build) method.
pub struct BybitWorkerBuilder {
    store: WorkerStore,
    opts: BybitWorkerBuilderOpts,
}

impl BybitWorkerBuilder {
    /// Returns a new `BybitWorkerBuilder` with the given options.
    pub fn new(store: WorkerStore, opts: BybitWorkerBuilderOpts) -> Self {
        Self { store, opts }
    }

    /// Set the URL for the `BybitWorker`.
    /// The default URL is `DEFAULT_URL`.
    pub fn with_url<T: Into<String>>(mut self, url: T) -> Self {
        self.opts.url = url.into();
        self
    }

    /// Set the internal channel size for the `BybitWorker`.
    /// The default size is `DEFAULT_CHANNEL_SIZE`.
    pub fn with_internal_ch_size(mut self, size: usize) -> Self {
        self.opts.internal_ch_size = size;
        self
    }

    /// Sets the store for the `BybitWorker`.
    /// If not set, the store is created and owned by the worker.
    pub fn with_store(mut self, store: WorkerStore) -> Self {
        self.store = store;
        self
    }
}

#[async_trait::async_trait]
impl<'a> AssetWorkerBuilder<'a> for BybitWorkerBuilder {
    type Opts = BybitWorkerBuilderOpts;
    type Worker = BybitWorker;
    type Error = BuildError;

    /// Returns a new `BybitWorkerBuilder` with the given options.
    fn new(store: WorkerStore, opts: Self::Opts) -> Self {
        Self { store, opts }
    }

    /// Returns the name of the worker.
    fn worker_name() -> &'static str {
        "bybit"
    }

    /// Creates the configured `BybitWorker`.
    async fn build(self) -> Result<Arc<BybitWorker>, BuildError> {
        let url = self.opts.url;
        let ch_size = self.opts.internal_ch_size;

        let connector = BybitWebSocketConnector::new(url);
        let connection = connector.connect().await?;

        let (sub_tx, sub_rx) = channel(ch_size);
        let (unsub_tx, unsub_rx) = channel(ch_size);

        let worker = Arc::new(BybitWorker::new(connector, self.store, sub_tx, unsub_tx));

        tokio::spawn(start_asset_worker(
            Arc::downgrade(&worker),
            connection,
            sub_rx,
            unsub_rx,
        ));

        Ok(worker)
    }
}