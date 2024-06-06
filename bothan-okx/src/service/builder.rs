use serde::Deserialize;
use std::sync::Arc;

use bothan_core::service::Service;
use tokio::sync::Mutex;

use crate::api::types::DEFAULT_URL;
use crate::error::Error;
use crate::types::DEFAULT_CHANNEL_SIZE;
use crate::{OkxService, OkxWebSocketConnector};

#[derive(Clone, Debug, Deserialize)]
pub struct OkxServiceBuilderOpts {
    pub url: Option<String>,
    pub cmd_ch_size: Option<usize>,
    pub remove_id_ch_size: Option<usize>,
}

pub struct OkxServiceBuilder {
    url: String,
    cmd_ch_size: usize,
    remove_id_ch_size: usize,
}

impl OkxServiceBuilder {
    pub fn new(opts: OkxServiceBuilderOpts) -> Self {
        Self {
            url: opts.url.unwrap_or(DEFAULT_URL.to_string()),
            cmd_ch_size: opts.cmd_ch_size.unwrap_or(DEFAULT_CHANNEL_SIZE),
            remove_id_ch_size: opts.remove_id_ch_size.unwrap_or(DEFAULT_CHANNEL_SIZE),
        }
    }

    pub fn with_url(mut self, url: String) -> Self {
        self.url = url;
        self
    }

    pub fn with_cmd_ch_size(mut self, size: usize) -> Self {
        self.cmd_ch_size = size;
        self
    }

    pub fn with_rem_id_ch_size(mut self, size: usize) -> Self {
        self.remove_id_ch_size = size;
        self
    }

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
    fn default() -> Self {
        Self {
            url: DEFAULT_URL.to_string(),
            cmd_ch_size: DEFAULT_CHANNEL_SIZE,
            remove_id_ch_size: DEFAULT_CHANNEL_SIZE,
        }
    }
}
