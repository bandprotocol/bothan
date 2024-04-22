use std::sync::Arc;

use tokio::sync::Mutex;

use crate::api::types::DEFAULT_URL;
use crate::error::Error;
use crate::types::DEFAULT_CHANNEL_SIZE;
use crate::{KrakenService, KrakenWebSocketConnector};

pub struct KrakenServiceBuilder {
    url: String,
    cmd_ch_size: usize,
    remove_id_ch_size: usize,
}

impl KrakenServiceBuilder {
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
    fn default() -> Self {
        Self {
            url: DEFAULT_URL.to_string(),
            cmd_ch_size: DEFAULT_CHANNEL_SIZE,
            remove_id_ch_size: DEFAULT_CHANNEL_SIZE,
        }
    }
}
