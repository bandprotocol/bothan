use serde::Deserialize;
use std::sync::Arc;

use bothan_core::service::Service;
use tokio::sync::Mutex;

use crate::api::types::DEFAULT_URL;
use crate::error::Error;
use crate::types::DEFAULT_CHANNEL_SIZE;
use crate::{OkxService, OkxWebSocketConnector};

/// Options for configuring the `OkxServiceBuilder`.
#[derive(Clone, Debug, Deserialize)]
pub struct OkxServiceBuilderOpts {
    pub url: Option<String>,
    pub cmd_ch_size: Option<usize>,
    pub remove_id_ch_size: Option<usize>,
}

/// A builder for creating instances of `OkxService`.
/// Methods can be chained to set the configuration values and the
/// service is constructed by calling the [`build`](OkxServiceBuilder::build) method.
/// # Example
/// ```no_run
/// use bothan_okx::OkxServiceBuilder;
///
/// #[tokio::main]
/// async fn main() {
///     let service = OkxServiceBuilder::default()
///         .with_url("wss://ws.okx.com:8443/ws/v5/public".to_string())
///         .with_cmd_ch_size(100)
///         .with_rem_id_ch_size(100)
///         .build()
///         .await
///         .unwrap();
///
///     // use service ...
/// }
/// ```
pub struct OkxServiceBuilder {
    url: String,
    cmd_ch_size: usize,
    remove_id_ch_size: usize,
}

impl OkxServiceBuilder {
    /// Creates a new builder instance from the provided options.
    pub fn new(opts: OkxServiceBuilderOpts) -> Self {
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

    /// Builds the `OkxService` instance.
    pub async fn build(self) -> Result<OkxService, Error> {
        let connector = OkxWebSocketConnector::new(self.url);
        let connection = connector.connect().await?;

        let mut service = OkxService::new(
            Arc::new(connector),
            Arc::new(Mutex::new(connection)),
            self.cmd_ch_size,
            self.remove_id_ch_size,
        );

        // Subscribe to a single symbol first to keep connection alive
        // TODO: find a better solution
        let _ = service.get_price_data(&["BTC-USDT"]).await;

        Ok(service)
    }
}

impl Default for OkxServiceBuilder {
    /// Creates a default `OkxServiceBuilder` instance with default values.
    fn default() -> Self {
        Self {
            url: DEFAULT_URL.to_string(),
            cmd_ch_size: DEFAULT_CHANNEL_SIZE,
            remove_id_ch_size: DEFAULT_CHANNEL_SIZE,
        }
    }
}
