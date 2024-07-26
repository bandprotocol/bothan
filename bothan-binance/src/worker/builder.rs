use std::sync::Arc;

use tokio::sync::mpsc::channel;

use bothan_core::store::WorkerStore;

use crate::api::websocket::BinanceWebSocketConnector;
use crate::worker::asset_worker::start_asset_worker;
use crate::worker::error::BuildError;
use crate::worker::opts::BinanceWorkerBuilderOpts;
use crate::worker::BinanceWorker;

/// Builds a `BinanceWorker` with custom options.
/// Methods can be chained to set the configuration values and the
/// service is constructed by calling the [`build`](BinanceWorkerBuilder::build) method.
pub struct BinanceWorkerBuilder {
    store: Arc<WorkerStore>,
    opts: BinanceWorkerBuilderOpts,
}

impl BinanceWorkerBuilder {
    /// Returns a new `BinanceWorkerBuilder` with the given options.
    pub fn new(store: Arc<WorkerStore>, opts: BinanceWorkerBuilderOpts) -> Self {
        Self { store, opts }
    }

    /// Set the URL for the `BinanceWorker`.
    /// The default URL is `DEFAULT_URL`.
    pub fn with_url<T: Into<String>>(mut self, url: T) -> Self {
        self.opts.url = url.into();
        self
    }

    /// Set the internal channel size for the `BinanceWorker`.
    /// The default size is `DEFAULT_CHANNEL_SIZE`.
    pub fn with_internal_ch_size(mut self, size: usize) -> Self {
        self.opts.internal_ch_size = size;
        self
    }

    /// Sets the store for the `BinanceWorker`.
    /// If not set, the store is created and owned by the worker.
    pub fn with_store(mut self, store: Arc<WorkerStore>) -> Self {
        self.store = store;
        self
    }

    /// Creates the configured `BinanceWorker`.
    pub async fn build(self) -> Result<Arc<BinanceWorker>, BuildError> {
        let url = self.opts.url;
        let ch_size = self.opts.internal_ch_size;

        let connector = BinanceWebSocketConnector::new(url);
        let connection = connector.connect().await?;

        let (sub_tx, sub_rx) = channel(ch_size);
        let (unsub_tx, unsub_rx) = channel(ch_size);

        let worker = Arc::new(BinanceWorker::new(connector, self.store, sub_tx, unsub_tx));

        tokio::spawn(start_asset_worker(
            Arc::downgrade(&worker),
            connection,
            sub_rx,
            unsub_rx,
        ));

        Ok(worker)
    }
}
