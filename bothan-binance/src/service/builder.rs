use std::sync::Arc;

use serde::Deserialize;
use tokio::sync::Mutex;

use crate::api::types::DEFAULT_URL;
use crate::error::Error;
use crate::types::DEFAULT_CHANNEL_SIZE;
use crate::{BinanceService, BinanceWebSocketConnector};

/// Options for the [`BinanceServiceBuilder`](BinanceServiceBuilder).
#[derive(Clone, Debug, Deserialize)]
pub struct BinanceServiceBuilderOpts {
    pub url: Option<String>,
    pub cmd_ch_size: Option<usize>,
    pub remove_id_ch_size: Option<usize>,
}

/// Builds a Binance service with custom options.
/// Methods can be chained to set the configuration values and the
/// service is constructed by calling the [`build`](BinanceServiceBuilder::build) method.
/// # Example
/// ```no_run rust
/// use bothan_binance::BinanceServiceBuilder;
///
/// async fn main() {
///     let service = BinanceServiceBuilder::default()
///         .with_cmd_ch_size(100)
///         .with_remove_id_ch_size(100)
///         .build()
///         .await
///         .unwrap();
///
///     // use service ...
/// }
/// ```
pub struct BinanceServiceBuilder {
    url: String,
    cmd_ch_size: usize,
    remove_id_ch_size: usize,
}

impl BinanceServiceBuilder {
    /// Returns a new Binance service builder.
    pub fn new(opts: BinanceServiceBuilderOpts) -> Self {
        Self {
            url: opts.url.unwrap_or(DEFAULT_URL.to_string()),
            cmd_ch_size: opts.cmd_ch_size.unwrap_or(DEFAULT_CHANNEL_SIZE),
            remove_id_ch_size: opts.remove_id_ch_size.unwrap_or(DEFAULT_CHANNEL_SIZE),
        }
    }

    /// Set the URL for the Binance service.
    /// The default URL is [`DEFAULT_URL`](DEFAULT_URL).
    pub fn with_url(mut self, url: String) -> Self {
        self.url = url;
        self
    }

    /// Set the internal command channel size for the Binance service.
    /// The default size is [`100`](DEFAULT_CHANNEL_SIZE).
    /// Set the internal command channel size for the Binance service.
    /// The default size is [`DEFAULT_CHANNEL_SIZE`](DEFAULT_CHANNEL_SIZE).
    pub fn with_cmd_ch_size(mut self, size: usize) -> Self {
        self.cmd_ch_size = size;
        self
    }

    /// Set the internal remove ID channel size for the Binance service.
    /// The default size is -[`100`](DEFAULT_CHANNEL_SIZE).
    /// Set the internal remove ID channel size for the Binance service.
    /// The default size is [`DEFAULT_CHANNEL_SIZE`](DEFAULT_CHANNEL_SIZE).
    pub fn with_remove_id_ch_size(mut self, size: usize) -> Self {
        self.remove_id_ch_size = size;
        self
    }

    /// Creates the configured [`BinanceService`](BinanceService).
    pub async fn build(self) -> Result<BinanceService, Error> {
        let connector = BinanceWebSocketConnector::new(self.url);
        let connection = connector.connect().await?;
        let service = BinanceService::new(
            Arc::new(connector),
            Arc::new(Mutex::new(connection)),
            self.cmd_ch_size,
            self.remove_id_ch_size,
        );
        Ok(service)
    }
}

impl Default for BinanceServiceBuilder {
    /// Create a new [`BinanceServiceBuilder`](BinanceServiceBuilder) with the default values.
    fn default() -> Self {
        Self {
            url: DEFAULT_URL.to_string(),
            cmd_ch_size: DEFAULT_CHANNEL_SIZE,
            remove_id_ch_size: DEFAULT_CHANNEL_SIZE,
        }
    }
}
