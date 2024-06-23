use std::sync::Arc;

use bothan_core::service::Service;
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::api::types::DEFAULT_URL;
use crate::error::Error;
use crate::types::DEFAULT_CHANNEL_SIZE;
use crate::{CoinbaseService, CoinbaseWebSocketConnector};

/// Options for configuring the `CoinbaseServiceBuilder`.
#[derive(Clone, Debug, Deserialize)]
pub struct CoinbaseServiceBuilderOpts {
    /// The URL for the Coinbase API.
    pub url: Option<String>,
    /// The size of the command channel.
    pub cmd_ch_size: Option<usize>,
    /// The size of the remove ID channel.
    pub remove_id_ch_size: Option<usize>,
}

/// A builder for creating instances of `CoinbaseService`.
pub struct CoinbaseServiceBuilder {
    url: String,
    cmd_ch_size: usize,
    remove_id_ch_size: usize,
}

/// A builder for creating instances of `CoinbaseService`.
/// Methods can be chained to set the configuration values and the
/// service is constructed by calling the [`build`](CoinbaseServiceBuilder::build) method.
/// # Example
/// ```no_run
/// use bothan_coinbase::CoinbaseServiceBuilder;
///
/// #[tokio::main]
/// async fn main() {
///     let service = CoinbaseServiceBuilder::default()
///         .with_cmd_ch_size(100)
///         .with_rem_id_ch_size(100)
///         .build()
///         .await
///         .unwrap();
///
///     // use service ...
/// }
/// ```
impl CoinbaseServiceBuilder {
    /// Creates a new `CoinbaseServiceBuilder` with the given options.
    pub fn new(opts: CoinbaseServiceBuilderOpts) -> Self {
        Self {
            url: opts.url.unwrap_or(DEFAULT_URL.to_string()),
            cmd_ch_size: opts.cmd_ch_size.unwrap_or(DEFAULT_CHANNEL_SIZE),
            remove_id_ch_size: opts.remove_id_ch_size.unwrap_or(DEFAULT_CHANNEL_SIZE),
        }
    }

    /// Sets the URL for the Coinbase API.
    /// The default URL is `DEFAULT_URL`.
    pub fn with_url(mut self, url: String) -> Self {
        self.url = url;
        self
    }

    /// Sets the size of the command channel.
    /// The default size is `DEFAULT_CHANNEL_SIZE`.
    pub fn with_cmd_ch_size(mut self, size: usize) -> Self {
        self.cmd_ch_size = size;
        self
    }

    /// Sets the size of the remove ID channel.
    /// The default size is `DEFAULT_CHANNEL_SIZE`.
    pub fn with_rem_id_ch_size(mut self, size: usize) -> Self {
        self.remove_id_ch_size = size;
        self
    }

    /// Builds the `CoinbaseService` instance.
    pub async fn build(self) -> Result<CoinbaseService, Error> {
        let connector = CoinbaseWebSocketConnector::new(self.url);
        let connection = connector.connect().await?;

        let mut service = CoinbaseService::new(
            Arc::new(connector),
            Arc::new(Mutex::new(connection)),
            self.cmd_ch_size,
            self.remove_id_ch_size,
        );

        // Subscribe to a single symbol first to keep the connection alive
        // TODO: find a better solution
        let _ = service.get_price_data(&["BTC-USD"]).await;

        Ok(service)
    }
}

impl Default for CoinbaseServiceBuilder {
    /// Creates a default `CoinbaseServiceBuilder` instance with default values.
    fn default() -> Self {
        Self {
            url: DEFAULT_URL.to_string(),
            cmd_ch_size: DEFAULT_CHANNEL_SIZE,
            remove_id_ch_size: DEFAULT_CHANNEL_SIZE,
        }
    }
}
