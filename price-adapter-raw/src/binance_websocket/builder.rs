use reqwest::StatusCode;
use tokio_tungstenite::connect_async;

use crate::error::Error;

use super::{websocket::BinanceWebsocket, WEBSOCKET_URL};

/// An object for creating a binance websocket.
#[derive(Default)]
pub struct BinanceWebsocketBuilder {
    url: String,
    ids: Vec<String>,
}

impl BinanceWebsocketBuilder {
    pub fn new() -> Self {
        Self {
            url: WEBSOCKET_URL.into(),
            ..Default::default()
        }
    }

    pub fn set_ids(mut self, ids: &[&str]) -> Self {
        self.ids = ids.iter().map(|id| id.to_string()).collect();
        self
    }

    pub fn ids(&self) -> &[String] {
        &self.ids
    }

    pub async fn build(&self) -> Result<BinanceWebsocket, Error> {
        let streams = self
            .ids
            .iter()
            .map(|id| format!("{}@miniTicker", id))
            .collect::<Vec<_>>();

        let ws_url = format!("{}/stream?streams={}", self.url, streams.join("/"));
        let (socket, response) = connect_async(ws_url).await?;

        let response_status = response.status();
        if StatusCode::is_success(&response_status) {
            tracing::error!("query request get error status {}", response_status);
            return Err(Error::ResponseStatusNotOk(response_status));
        }

        Ok(BinanceWebsocket::new(socket))
    }
}
