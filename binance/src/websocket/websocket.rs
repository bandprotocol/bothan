use std::sync::Arc;

use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::http::StatusCode;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

use crate::error::Error;
use crate::websocket::types::MiniTickerInfo;
use crate::websocket::util::parse_message;

const DEFAULT_URL: &str = "wss://stream.binance.com:9443/stream";

pub struct BinanceWebsocket {
    url: String,
    sender: Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
    receiver: Option<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    _streamed: Arc<Mutex<bool>>,
}

impl BinanceWebsocket {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            sender: None,
            receiver: None,
            _streamed: Arc::new(Mutex::new(false)),
        }
    }

    pub fn default() -> Self {
        Self::new(DEFAULT_URL)
    }

    pub async fn connect(&mut self) -> Result<(), Error> {
        let (socket, response) = connect_async(&self.url).await?;

        if !(StatusCode::is_server_error(&response.status())
            || StatusCode::is_client_error(&response.status()))
        {
            let (sender, receiver) = socket.split();
            self.sender = Some(sender);
            self.receiver = Some(receiver);
            Ok(())
        } else {
            Err(Error::Unknown)
        }
    }

    pub async fn disconnect(&mut self) -> Result<(), Error> {
        panic!("Not implemented")
    }

    pub async fn subscribe(&mut self, ids: &[&str]) -> Result<(), Error> {
        if let Some(sender) = self.sender.as_mut() {
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
            sender.send(message).await?;
            Ok(())
        } else {
            Err(Error::Unknown)
        }
    }

    pub async fn unsubscribe(&mut self, ids: &[&str]) -> Result<(), Error> {
        if let Some(sender) = self.sender.as_mut() {
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

            sender.send(message).await?;

            Ok(())
        } else {
            Err(Error::Unknown)
        }
    }

    pub async fn next(&mut self) -> Result<MiniTickerInfo, Error> {
        if let Some(receiver) = self.receiver.as_mut() {
            if let Some(Ok(msg)) = receiver.next().await {
                println!("{}", msg.to_string());
                let tt = parse_message(msg.to_string());
                tt.ok_or(Error::Unknown)
            } else {
                Err(Error::Unknown)
            }
        } else {
            Err(Error::Unknown)
        }
    }
}
