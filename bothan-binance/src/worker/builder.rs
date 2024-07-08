use std::sync::Arc;

use serde::Deserialize;
use tokio::sync::mpsc::channel;

use bothan_core::store::Store;

use crate::api::websocket::BinanceWebSocketConnector;
use crate::api::websocket::DEFAULT_URL;
use crate::worker::asset_worker::start_asset_worker;
use crate::worker::error::BuildError;
use crate::worker::types::DEFAULT_CHANNEL_SIZE;
use crate::worker::BinanceWorker;

/// Options for configuring the `BinanceWorkerBuilder`.
///
/// `BinanceWorkerBuilderOpts` provides a way to specify custom settings for creating a `BinanceWorker`.
/// This struct allows users to set optional parameters such as the WebSocket URL and the internal channel size,
/// which will be used during the construction of the `BinanceWorker`.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct BinanceWorkerBuilderOpts {
    pub url: Option<String>,
    pub internal_ch_size: Option<usize>,
}

/// Builds a `BinanceWorker` with custom options.
/// Methods can be chained to set the configuration values and the
/// service is constructed by calling the [`build`](BinanceWorkerBuilder::build) method.
/// # Example
/// ```no_run
/// use bothan_binance::BinanceWorkerBuilder;
///
///
/// #[tokio::main]
/// async fn main() {
///     let worker = BinanceWorkerBuilder::default()
///         .build()
///         .await
///         .unwrap();
///
///     // use worker ...
/// }
/// ```
pub struct BinanceWorkerBuilder {
    url: String,
    internal_ch_size: usize,
    store: Arc<Store>,
}

impl BinanceWorkerBuilder {
    /// Returns a new `BinanceWorkerBuilder` with the given options.
    pub fn new(opts: BinanceWorkerBuilderOpts, store: Option<Arc<Store>>) -> Self {
        Self {
            url: opts.url.unwrap_or(DEFAULT_URL.to_string()),
            internal_ch_size: opts.internal_ch_size.unwrap_or(DEFAULT_CHANNEL_SIZE),
            store: store.unwrap_or_default(),
        }
    }

    /// Set the URL for the `BinanceWorker`.
    /// The default URL is `DEFAULT_URL`.
    pub fn with_url(mut self, url: String) -> Self {
        self.url = url;
        self
    }

    /// Set the internal channel size for the `BinanceWorker`.
    /// The default size is `DEFAULT_CHANNEL_SIZE`.
    pub fn with_internal_ch_size(mut self, size: usize) -> Self {
        self.internal_ch_size = size;
        self
    }

    pub fn with_store(mut self, store: Arc<Store>) -> Self {
        self.store = store;
        self
    }

    /// Creates the configured `BinanceWorker`.
    pub async fn build(self) -> Result<Arc<BinanceWorker>, BuildError> {
        let connector = BinanceWebSocketConnector::new(self.url);
        let connection = connector.connect().await?;

        let (sub_tx, sub_rx) = channel(self.internal_ch_size);
        let (unsub_tx, unsub_rx) = channel(self.internal_ch_size);

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

impl Default for BinanceWorkerBuilder {
    /// Create a new `BinanceWorkerBuilder` with the default values.
    fn default() -> Self {
        Self::new(BinanceWorkerBuilderOpts::default(), None)
    }
}
