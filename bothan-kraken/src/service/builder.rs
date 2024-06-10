use std::sync::Arc;

use serde::Deserialize;
use tokio::sync::Mutex;

use crate::api::types::DEFAULT_URL;
use crate::error::Error;
use crate::types::DEFAULT_CHANNEL_SIZE;
use crate::{KrakenService, KrakenWebSocketConnector};

/// Options for configuring the `KrakenServiceBuilder`.
#[derive(Clone, Debug, Deserialize)]
pub struct KrakenServiceBuilderOpts {
    pub url: Option<String>,
    pub cmd_ch_size: Option<usize>,
    pub remove_id_ch_size: Option<usize>,
}

/// A builder for creating instances of `KrakenService`.
/// Methods can be chained to set the configuration values and the
/// service is constructed by calling the [`build`](KrakenServiceBuilder::build) method.
/// # Example
/// ```no_run
/// use bothan_kraken::KrakenServiceBuilder;
///
/// #[tokio::main]
/// async fn main() {
///     let service = KrakenServiceBuilder::default()
///         .with_url("wss://ws.kraken.com/v2".to_string())
///         .with_cmd_ch_size(100)
///         .with_rem_id_ch_size(100)
///         .build()
///         .await
///         .unwrap();
///
///     // use service ...
/// }
/// ```
pub struct KrakenServiceBuilder {
    url: String,
    cmd_ch_size: usize,
    remove_id_ch_size: usize,
}

impl KrakenServiceBuilder {
    /// Creates a new builder instance from the provided options.
    pub fn new(opts: KrakenServiceBuilderOpts) -> Self {
        Self {
            url: opts.url.unwrap_or(DEFAULT_URL.to_string()),
            cmd_ch_size: opts.cmd_ch_size.unwrap_or(DEFAULT_CHANNEL_SIZE),
            remove_id_ch_size: opts.remove_id_ch_size.unwrap_or(DEFAULT_CHANNEL_SIZE),
        }
    }

    /// Sets the URL for the WebSocket connection.
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

    /// Builds the `KrakenService` instance.
    pub async fn build(self) -> Result<KrakenService, Error> {
        let connector = KrakenWebSocketConnector::new(self.url);
        let connection = connector.connect().await?;

        let service = KrakenService::new(
            Arc::new(connector),
            Arc::new(Mutex::new(connection)),
            self.cmd_ch_size,
            self.remove_id_ch_size,
        );

        Ok(service)
    }
}

impl Default for KrakenServiceBuilder {
    /// Creates a default `KrakenServiceBuilder` instance with default values.
    fn default() -> Self {
        Self {
            url: DEFAULT_URL.to_string(),
            cmd_ch_size: DEFAULT_CHANNEL_SIZE,
            remove_id_ch_size: DEFAULT_CHANNEL_SIZE,
        }
    }
}
