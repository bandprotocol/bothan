use std::sync::Arc;

use tokio::sync::Mutex;

use crate::api::types::DEFAULT_URL;
use crate::error::Error;
use crate::types::DEFAULT_CHANNEL_SIZE;
use crate::{BinanceService, BinanceWebSocketConnector};

pub struct BinanceServiceBuilder {
    url: String,
    cmd_ch_size: usize,
    rem_id_ch_size: usize,
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

    pub fn with_rem_id_ch_size(mut self, size: usize) -> Self {
        self.rem_id_ch_size = size;
        self
    }

    pub async fn build(self) -> Result<BinanceService, Error> {
        let connector = BinanceWebSocketConnector::new(self.url);
        let connection = connector.connect().await?;
        let service = BinanceService::new(
            Arc::new(connector),
            Arc::new(Mutex::new(connection)),
            self.cmd_ch_size,
            self.rem_id_ch_size,
        )
        .await;
        Ok(service)
    }
}

impl Default for BinanceServiceBuilder {
    fn default() -> Self {
        Self {
            url: DEFAULT_URL.to_string(),
            cmd_ch_size: DEFAULT_CHANNEL_SIZE,
            rem_id_ch_size: DEFAULT_CHANNEL_SIZE,
        }
    }
}
