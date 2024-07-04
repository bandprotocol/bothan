use std::sync::Arc;

use serde::Deserialize;
use tokio::sync::mpsc::channel;

use crate::api::types::DEFAULT_URL;
use crate::store::asset_worker::start_asset_worker;
use crate::store::types::DEFAULT_CHANNEL_SIZE;
use crate::store::BinanceWorker;
use crate::BinanceWebSocketConnector;

/// Options for the `BinanceServiceBuilder`.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct BinanceWorkerBuilderOpts {
    pub url: Option<String>,
    pub internal_ch_size: Option<usize>,
}

/// Builds a Binance service with custom options.
/// Methods can be chained to set the configuration values and the
/// service is constructed by calling the [`build`](BinanceServiceBuilder::build) method.
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
}

impl BinanceWorkerBuilder {
    /// Returns a new `BinanceStoreBuilder` with the given options.
    pub fn new(opts: BinanceWorkerBuilderOpts) -> Self {
        Self {
            url: opts.url.unwrap_or(DEFAULT_URL.to_string()),
            internal_ch_size: opts.internal_ch_size.unwrap_or(DEFAULT_CHANNEL_SIZE),
        }
    }

    /// Set the URL for the BinanceStore.
    /// The default URL is `DEFAULT_URL`.
    pub fn with_url(mut self, url: String) -> Self {
        self.url = url;
        self
    }

    /// Set the internal channel size for the BinanceStore.
    /// The default size is `DEFAULT_CHANNEL_SIZE`.
    pub fn with_internal_ch_size(mut self, size: usize) -> Self {
        self.internal_ch_size = size;
        self
    }

    /// Creates the configured `BinanceService`.
    pub async fn build(self) -> Result<Arc<BinanceWorker>, anyhow::Error> {
        let connector = BinanceWebSocketConnector::new(self.url);
        let connection = connector.connect().await?;

        let (sub_tx, sub_rx) = channel(self.internal_ch_size);
        let (unsub_tx, unsub_rx) = channel(self.internal_ch_size);

        let worker = Arc::new(BinanceWorker::new(connector, sub_tx, unsub_tx));

        start_asset_worker(Arc::downgrade(&worker), connection, sub_rx, unsub_rx).await;
        Ok(worker)
    }
}

impl Default for BinanceWorkerBuilder {
    /// Create a new `BinanceServiceBuilder` with the default values.
    fn default() -> Self {
        Self::new(BinanceWorkerBuilderOpts::default())
    }
}
