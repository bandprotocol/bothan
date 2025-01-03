use std::sync::Arc;

use tokio::sync::mpsc::channel;

use crate::api::CoinbaseWebSocketConnector;
use crate::worker::asset_worker::start_asset_worker;
use crate::worker::error::BuildError;
use crate::worker::opts::CoinbaseWorkerBuilderOpts;
use crate::worker::CoinbaseWorker;
use bothan_core::store::WorkerStore;
use bothan_core::worker::AssetWorkerBuilder;

/// Builds a `CoinbaseWorker` with custom options.
/// Methods can be chained to set the configuration values and the
/// service is constructed by calling the [`build`](CoinbaseWorkerBuilder::build) method.
pub struct CoinbaseWorkerBuilder {
    store: WorkerStore,
    opts: CoinbaseWorkerBuilderOpts,
}

impl CoinbaseWorkerBuilder {
    /// Returns a new `CoinbaseWorkerBuilder` with the given options.
    pub fn new(store: WorkerStore, opts: CoinbaseWorkerBuilderOpts) -> Self {
        Self { store, opts }
    }

    /// Set the URL for the `CoinbaseWorker`.
    /// The default URL is `DEFAULT_URL`.
    pub fn with_url<T: Into<String>>(mut self, url: T) -> Self {
        self.opts.url = url.into();
        self
    }

    /// Set the internal channel size for the `CoinbaseWorker`.
    /// The default size is `DEFAULT_CHANNEL_SIZE`.
    pub fn with_internal_ch_size(mut self, size: usize) -> Self {
        self.opts.internal_ch_size = size;
        self
    }

    /// Sets the store for the `CoinbaseWorker`.
    /// If not set, the store is created and owned by the worker.
    pub fn with_store(mut self, store: WorkerStore) -> Self {
        self.store = store;
        self
    }
}

#[async_trait::async_trait]
impl<'a> AssetWorkerBuilder<'a> for CoinbaseWorkerBuilder {
    type Opts = CoinbaseWorkerBuilderOpts;
    type Worker = CoinbaseWorker;
    type Error = BuildError;

    /// Returns a new `CoinbaseWorkerBuilder` with the given options.
    fn new(store: WorkerStore, opts: Self::Opts) -> Self {
        Self { store, opts }
    }

    /// Returns the name of the worker.
    fn worker_name() -> &'static str {
        "coinbase"
    }

    /// Creates the configured `CoinbaseWorker`.
    async fn build(self) -> Result<Arc<CoinbaseWorker>, BuildError> {
        let url = self.opts.url;
        let ch_size = self.opts.internal_ch_size;

        let connector = CoinbaseWebSocketConnector::new(url);
        let connection = connector.connect().await?;

        let (sub_tx, sub_rx) = channel(ch_size);
        let (unsub_tx, unsub_rx) = channel(ch_size);

        let to_sub = self
            .store
            .get_query_ids()
            .await?
            .into_iter()
            .collect::<Vec<String>>();

        if !to_sub.is_empty() {
            // Unwrap here as the channel is guaranteed to be open
            sub_tx.send(to_sub).await.unwrap();
        }

        let worker = Arc::new(CoinbaseWorker::new(connector, self.store, sub_tx, unsub_tx));

        tokio::spawn(start_asset_worker(
            Arc::downgrade(&worker),
            connection,
            sub_rx,
            unsub_rx,
        ));

        Ok(worker)
    }
}
