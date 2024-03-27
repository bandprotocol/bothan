use std::sync::Arc;

use serde::Deserialize;
use tokio::sync::Mutex;

use crate::api::types::DEFAULT_URL;
use crate::error::Error;
use crate::types::DEFAULT_CHANNEL_SIZE;
use crate::{BinanceService, BinanceWebSocketConnector};

#[derive(Debug, Deserialize)]
pub struct BinanceServiceBuilderOpts {
    pub url: Option<String>,
    pub cmd_ch_size: Option<usize>,
    pub remove_id_ch_size: Option<usize>,
}

pub struct BinanceServiceBuilder {
    url: String,
    cmd_ch_size: usize,
    remove_id_ch_size: usize,
}

impl BinanceServiceBuilder {
    pub fn with_url(mut self, url: String) -> Self {
        self.url = url;
        self
    }

    pub fn with_cmd_ch_size(mut self, size: usize) -> Self {
        self.cmd_ch_size = size;
        self
    }

    pub fn with_remove_id_ch_size(mut self, size: usize) -> Self {
        self.remove_id_ch_size = size;
        self
    }

    pub fn new(opts: BinanceServiceBuilderOpts) -> Self {
        Self {
            url: opts.url.unwrap_or(DEFAULT_URL.to_string()),
            cmd_ch_size: opts.cmd_ch_size.unwrap_or(DEFAULT_CHANNEL_SIZE),
            remove_id_ch_size: opts.remove_id_ch_size.unwrap_or(DEFAULT_CHANNEL_SIZE),
        }
    }

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
    fn default() -> Self {
        Self {
            url: DEFAULT_URL.to_string(),
            cmd_ch_size: DEFAULT_CHANNEL_SIZE,
            remove_id_ch_size: DEFAULT_CHANNEL_SIZE,
        }
    }
}
