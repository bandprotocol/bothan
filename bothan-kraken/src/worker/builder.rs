use std::sync::Arc;

use tokio::sync::mpsc::channel;

use bothan_core::store::Store;

use crate::api::KrakenWebSocketConnector;
use crate::worker::asset_worker::start_asset_worker;
use crate::worker::error::BuildError;
use crate::worker::opts::KrakenWorkerBuilderOpts;
use crate::worker::KrakenWorker;

/// Builds a `KrakenWorker` with custom options.
/// Methods can be chained to set the configuration values and the
/// service is constructed by calling the [`build`](KrakenWorkerBuilder::build) method.
/// # Example
/// ```no_run
/// use bothan_kraken::KrakenWorkerBuilder;
///
///
/// #[tokio::main]
/// async fn main() {
///     let worker = KrakenWorkerBuilder::default()
///         .build()
///         .await
///         .unwrap();
///
///     // use worker ...
/// }
/// ```
pub struct KrakenWorkerBuilder {
    store: Arc<Store>,
    opts: KrakenWorkerBuilderOpts,
}

impl KrakenWorkerBuilder {
    /// Returns a new `KrakenWorkerBuilder` with the given options.
    pub fn new(store: Arc<Store>, opts: KrakenWorkerBuilderOpts) -> Self {
        Self { store, opts }
    }

    /// Set the URL for the `KrakenWorker`.
    /// The default URL is `DEFAULT_URL`.
    pub fn with_url<T: Into<String>>(mut self, url: T) -> Self {
        self.opts.url = url.into();
        self
    }

    /// Set the internal channel size for the `KrakenWorker`.
    /// The default size is `DEFAULT_CHANNEL_SIZE`.
    pub fn with_internal_ch_size(mut self, size: usize) -> Self {
        self.opts.internal_ch_size = size;
        self
    }

    /// Sets the store for the `KrakenWorker`.
    /// If not set, the store is created and owned by the worker.
    pub fn with_store(mut self, store: Arc<Store>) -> Self {
        self.store = store;
        self
    }

    /// Creates the configured `KrakenWorker`.
    pub async fn build(self) -> Result<Arc<KrakenWorker>, BuildError> {
        let url = self.opts.url;
        let ch_size = self.opts.internal_ch_size;

        let connector = KrakenWebSocketConnector::new(url);
        let connection = connector.connect().await?;

        let (sub_tx, sub_rx) = channel(ch_size);
        let (unsub_tx, unsub_rx) = channel(ch_size);

        let worker = Arc::new(KrakenWorker::new(connector, self.store, sub_tx, unsub_tx));

        tokio::spawn(start_asset_worker(
            Arc::downgrade(&worker),
            connection,
            sub_rx,
            unsub_rx,
        ));

        Ok(worker)
    }
}

impl Default for KrakenWorkerBuilder {
    /// Create a new `KrakenWorkerBuilder` with its default values.
    fn default() -> Self {
        Self::new(
            Arc::new(Store::default()),
            KrakenWorkerBuilderOpts::default(),
        )
    }
}
