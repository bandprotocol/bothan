use std::sync::Arc;

use serde::Deserialize;
use tokio::sync::Mutex;

use crate::api::types::DEFAULT_URL;
use crate::api::ConnectionError;
use crate::store::types::DEFAULT_CHANNEL_SIZE;
use crate::{BinanceStore, BinanceWebSocketConnector};

/// Options for the `BinanceServiceBuilder`.
#[derive(Clone, Debug, Deserialize)]
pub struct BinanceStoreBuilderOpts {
    pub url: Option<String>,
    pub cmd_ch_size: Option<usize>,
    pub remove_id_ch_size: Option<usize>,
}

/// Builds a Binance service with custom options.
/// Methods can be chained to set the configuration values and the
/// service is constructed by calling the [`build`](BinanceServiceBuilder::build) method.
/// # Example
/// ```no_run
/// use bothan_binance::BinanceStoreBuilder;
///
///
/// #[tokio::main]
/// async fn main() {
///     let service = BinanceStoreBuilder::default()
///         .build()
///         .await
///         .unwrap();
///
///     // use service ...
/// }
/// ```
pub struct BinanceStoreBuilder {
    url: String,
    internal_ch_size: usize,
}

impl BinanceStoreBuilder {
    /// Returns a new `BinanceStoreBuilder` with the given options.
    pub fn new(opts: BinanceStoreBuilderOpts) -> Self {
        Self {
            url: opts.url.unwrap_or(DEFAULT_URL.to_string()),
            internal_ch_size: opts.cmd_ch_size.unwrap_or(DEFAULT_CHANNEL_SIZE),
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
    pub async fn build(self) -> Result<BinanceStore, ConnectionError> {
        let connector = BinanceWebSocketConnector::new(self.url);
        let connection = connector.connect().await?;

        let service = BinanceStore::new(
            Arc::new(connector),
            Arc::new(Mutex::new(connection)),
            self.internal_ch_size,
        );

        Ok(service)
    }
}

impl Default for BinanceStoreBuilder {
    /// Create a new `BinanceServiceBuilder` with the default values.
    fn default() -> Self {
        Self {
            url: DEFAULT_URL.to_string(),
            internal_ch_size: DEFAULT_CHANNEL_SIZE,
        }
    }
}
