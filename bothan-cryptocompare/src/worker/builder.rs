use std::sync::Arc;

use tokio::sync::mpsc::channel;

use bothan_core::store::WorkerStore;
use bothan_core::worker::AssetWorkerBuilder;

use crate::api::CryptoCompareWebSocketConnector;
use crate::worker::asset_worker::start_asset_worker;
use crate::worker::errors::BuildError;
use crate::worker::opts::CryptoCompareWorkerBuilderOpts;
use crate::worker::CryptoCompareWorker;

/// Builds a `CryptoCompareWorker` with custom options.
/// Methods can be chained to set the configuration values and the
/// service is constructed by calling the [`build`](CryptoCompareWorker::build) method.
pub struct CryptoCompareWorkerBuilder {
    store: WorkerStore,
    opts: CryptoCompareWorkerBuilderOpts,
}

impl CryptoCompareWorkerBuilder {
    /// Set the URL for the `CryptoCompareWorker`.
    /// The default URL is `DEFAULT_URL` when no API key is provided
    /// and is `DEFAULT_PRO_URL` when an API key is provided.
    pub fn with_url<T: Into<String>>(mut self, url: T) -> Self {
        self.opts.url = url.into();
        self
    }

    /// Set the api key for the `CryptoCompareWorker`.
    pub fn with_api_key<T: Into<String>>(mut self, api_key: T) -> Self {
        self.opts.api_key = Some(api_key.into());
        self
    }

    /// Set the internal channel size for the `CryptoCompareWorker`.
    /// The default size is `DEFAULT_CHANNEL_SIZE`.
    pub fn with_internal_ch_size(mut self, size: usize) -> Self {
        self.opts.internal_ch_size = size;
        self
    }

    /// Sets the store for the `CryptoCompareWorker`.
    /// If not set, the store is created and owned by the worker.
    pub fn with_store(mut self, store: WorkerStore) -> Self {
        self.store = store;
        self
    }
}

#[async_trait::async_trait]
impl<'a> AssetWorkerBuilder<'a> for CryptoCompareWorkerBuilder {
    type Opts = CryptoCompareWorkerBuilderOpts;
    type Worker = CryptoCompareWorker;
    type Error = BuildError;

    /// Returns a new `CryptoCompareWorkerBuilder` with the given options.
    fn new(store: WorkerStore, opts: Self::Opts) -> Self {
        Self { store, opts }
    }

    /// Returns the name of the worker.
    fn worker_name() -> &'static str {
        "cryptocompare"
    }

    /// Creates the configured `CryptoCompareWorker`.
    async fn build(self) -> Result<Arc<Self::Worker>, Self::Error> {
        let api_key = self.opts.api_key.ok_or(BuildError::MissingApiKey)?;
        let connector = CryptoCompareWebSocketConnector::new(self.opts.url, api_key);
        let connection = connector.connect().await?;

        let (sub_tx, sub_rx) = channel(self.opts.internal_ch_size);
        let (unsub_tx, unsub_rx) = channel(self.opts.internal_ch_size);
        let to_sub = self
            .store
            .get_query_ids()
            .await?
            .into_iter()
            .collect::<Vec<String>>();

        if !to_sub.is_empty() {
            sub_tx.send(to_sub).await?;
        }

        let worker = Arc::new(CryptoCompareWorker::new(
            connector, self.store, sub_tx, unsub_tx,
        ));

        tokio::spawn(start_asset_worker(
            Arc::downgrade(&worker),
            connection,
            sub_rx,
            unsub_rx,
        ));

        Ok(worker)
    }
}
