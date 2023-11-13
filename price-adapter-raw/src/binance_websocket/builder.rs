use reqwest::StatusCode;
use tokio_tungstenite::connect_async;

use crate::error::Error;

use super::websocket::BinanceWebsocket;

/// An object for creating a binance websocket.
#[derive(Default)]
pub struct BinanceWebsocketBuilder {
    url: String,
    query_symbols: Vec<(String, String)>,
}

impl BinanceWebsocketBuilder {
    pub fn new() -> Self {
        Self {
            url: "wss://stream.binance.com:9443".into(),
            ..Default::default()
        }
    }

    pub fn set_query_symbols(mut self, symbols: &[(&str, &str)]) -> Self {
        let symbols = symbols
            .iter()
            .map(|(base, quote)| (base.to_string(), quote.to_string()))
            .collect::<Vec<_>>();

        self.query_symbols = symbols;
        self
    }

    pub fn query_symbols(&self) -> &[(String, String)] {
        &self.query_symbols
    }

    pub async fn build(&self) -> Result<BinanceWebsocket, Error> {
        let streams = self
            .query_symbols
            .iter()
            .map(|(base, quote)| format!("{}{}@miniTicker", base, quote,))
            .collect::<Vec<_>>();

        let ws_url = format!("{}/stream?streams={}", self.url, streams.join("/"));
        let (socket, response) = connect_async(ws_url).await?;

        let response_status = response.status();
        if StatusCode::is_success(&response_status) {
            tracing::error!("query request get error status {}", response_status);
            return Err(Error::ResponseStatusNotOk(response_status));
        }

        Ok(BinanceWebsocket::new(socket, &self.query_symbols))
    }
}
