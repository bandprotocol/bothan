use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::error::Error as TungsteniteError;
use tokio_tungstenite::tungstenite::http::StatusCode;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tracing::warn;

use crate::api::error::Error;
use crate::api::types::channels::Channel;
use crate::api::types::request::{Request, RequestType};
use crate::api::types::{CoinbaseResponse, DEFAULT_URL};

/// A connector for establishing a WebSocket connection to the Coinbase API.
pub struct CoinbaseWebSocketConnector {
    url: String,
}

impl CoinbaseWebSocketConnector {
    /// Creates a new `CoinbaseWebSocketConnector`.
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    /// Connects to the WebSocket and returns a `CoinbaseWebSocketConnection`.
    pub async fn connect(&self) -> Result<CoinbaseWebSocketConnection, Error> {
        let (wss, resp) = connect_async(self.url.clone()).await?;

        let status = resp.status();
        if StatusCode::is_server_error(&status) || StatusCode::is_client_error(&status) {
            warn!("failed to connect with response code {}", resp.status());
            return Err(Error::ConnectionFailure(resp.status()));
        }

        Ok(CoinbaseWebSocketConnection::new(wss))
    }
}

impl Default for CoinbaseWebSocketConnector {
    /// Creates a default `CoinbaseWebSocketConnector` with the default URL.
    fn default() -> Self {
        Self {
            url: DEFAULT_URL.to_string(),
        }
    }
}

/// A connection to the Coinbase WebSocket API.
pub struct CoinbaseWebSocketConnection {
    sender: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    receiver: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl CoinbaseWebSocketConnection {
    /// Creates a new `CoinbaseWebSocketConnection`.
    pub fn new(web_socket_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        let (sender, receiver) = web_socket_stream.split();
        Self { sender, receiver }
    }

    /// Subscribes to the specified channels and product IDs.
    pub async fn subscribe(
        &mut self,
        channels: Vec<Channel>,
        product_ids: &[&str],
    ) -> Result<(), Error> {
        let request = Request {
            type_: RequestType::Subscribe,
            product_ids: product_ids.iter().map(|id| id.to_string()).collect(),
            channels,
        };

        let message = Message::Text(serde_json::to_string(&request)?);
        Ok(self.sender.send(message).await?)
    }

    /// Unsubscribes from the specified channels and product IDs.
    pub async fn unsubscribe(
        &mut self,
        channels: Vec<Channel>,
        product_ids: &[&str],
    ) -> Result<(), Error> {
        let request = Request {
            type_: RequestType::Unsubscribe,
            product_ids: product_ids.iter().map(|id| id.to_string()).collect(),
            channels,
        };
        let message = Message::Text(serde_json::to_string(&request)?);
        Ok(self.sender.send(message).await?)
    }

    /// Receives the next message from the WebSocket.
    pub async fn next(&mut self) -> Result<CoinbaseResponse, Error> {
        if let Some(result_msg) = self.receiver.next().await {
            return match result_msg {
                Ok(Message::Text(msg)) => serde_json::from_str::<CoinbaseResponse>(&msg)
                    .map_err(|_| Error::UnsupportedMessage),
                Ok(Message::Close(_)) => Err(Error::ChannelClosed),
                Err(err) => match err {
                    TungsteniteError::Protocol(..) => Err(Error::ChannelClosed),
                    TungsteniteError::ConnectionClosed => Err(Error::ChannelClosed),
                    _ => Err(Error::UnsupportedMessage),
                },
                _ => Err(Error::UnsupportedMessage),
            };
        }

        Err(Error::ChannelClosed)
    }
}
