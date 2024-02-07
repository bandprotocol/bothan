use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::http::StatusCode;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tracing::warn;

use crate::api::error::Error;
use crate::api::types::BinanceResponse;

const DEFAULT_URL: &str = "wss://stream.binance.com:9443/stream";

pub struct BinanceWebsocket {
    url: String,
    sender: Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
    receiver: Option<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
}

impl BinanceWebsocket {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            sender: None,
            receiver: None,
        }
    }

    pub async fn connect(&mut self) -> Result<(), Error> {
        let (socket, response) = connect_async(&self.url).await?;

        let status = response.status();
        if StatusCode::is_server_error(&status) || StatusCode::is_client_error(&status) {
            return Err(Error::ConnectionFailure(status));
        }

        let (sender, receiver) = socket.split();
        self.sender = Some(sender);
        self.receiver = Some(receiver);

        Ok(())
    }

    pub async fn disconnect(&mut self) -> Result<(), Error> {
        let mut sender = self.sender.take().ok_or(Error::NotConnected())?;
        // Ignore result as we just want to send a close message
        if sender.send(Message::Close(None)).await.is_err() {
            warn!("unable to send close frame")
        }

        self.receiver = None;
        Ok(())
    }

    pub async fn subscribe(&mut self, ids: &[&str]) -> Result<(), Error> {
        let sender = self.sender.as_mut().ok_or(Error::NotConnected())?;

        let stream_ids = ids
            .iter()
            .map(|id| format!("{}@miniTicker", id))
            .collect::<Vec<_>>();

        let payload = json!({
            "method": "SUBSCRIBE",
            "params": stream_ids,
            "id": rand::random::<u32>()
        });

        let message = Message::Text(payload.to_string());
        Ok(sender.send(message).await?)
    }

    pub async fn unsubscribe(&mut self, ids: &[&str]) -> Result<(), Error> {
        let sender = self.sender.as_mut().ok_or(Error::NotConnected())?;

        let stream_ids = ids
            .iter()
            .map(|id| format!("{}@miniTicker", id))
            .collect::<Vec<_>>();

        let payload = json!({
            "method": "UNSUBSCRIBE",
            "params": stream_ids,
            "id": rand::random::<u32>()
        });

        let message = Message::Text(payload.to_string());
        Ok(sender.send(message).await?)
    }

    pub async fn next(&mut self) -> Result<BinanceResponse, Error> {
        let receiver = self.receiver.as_mut().ok_or(Error::NotConnected())?;

        match receiver.next().await {
            Some(result_msg) => {
                let msg = result_msg?;
                Ok(serde_json::from_str::<BinanceResponse>(&msg.to_string())?)
            }
            None => {
                // internal channel closed, this should never happen
                panic!()
            }
        }
    }
}

impl Default for BinanceWebsocket {
    fn default() -> Self {
        Self::new(DEFAULT_URL)
    }
}
