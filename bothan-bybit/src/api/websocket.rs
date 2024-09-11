use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::error::Error as TungsteniteError;
use tokio_tungstenite::tungstenite::http::StatusCode;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tracing::warn;

use crate::api::error::{ConnectionError, MessageError, SendError};
use crate::api::types::BybitResponse;

/// A connector for establishing a WebSocket connection to the Bybit API.
pub struct BybitWebSocketConnector {
    url: String,
}

impl BybitWebSocketConnector {
    /// Creates a new instance of `BybitWebSocketConnector`.
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    /// Connects to the Bybit WebSocket API.
    pub async fn connect(&self) -> Result<BybitWebSocketConnection, ConnectionError> {
        let (wss, resp) = connect_async(self.url.clone()).await?;

        let status = resp.status();
        if StatusCode::is_server_error(&status) || StatusCode::is_client_error(&status) {
            warn!("failed to connect with response code {}", resp.status());
            return Err(ConnectionError::UnsuccessfulHttpResponse(resp.status()));
        }

        Ok(BybitWebSocketConnection::new(wss))
    }
}

/// Represents an active WebSocket connection to the Bybit API.
pub struct BybitWebSocketConnection {
    sender: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    receiver: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl BybitWebSocketConnection {
    /// Creates a new `BybitWebSocketConnection` instance.
    pub fn new(web_socket_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        let (sender, receiver) = web_socket_stream.split();
        Self { sender, receiver }
    }

    /// Subscribes to ticker updates for the given symbols.
    pub async fn subscribe_ticker(&mut self, symbols: &[&str]) -> Result<(), SendError> {
        let payload = json!({
            "op": "subscribe",
            "args": symbols.iter().map(|s| format!("tickers.{}", s)).collect::<Vec<String>>(),
        });

        // Send the subscription message.
        let message = Message::Text(payload.to_string());
        Ok(self.sender.send(message).await?)
    }

    /// Unsubscribes to ticker updates for the given symbols.
    pub async fn unsubscribe_ticker(&mut self, symbols: &[&str]) -> Result<(), SendError> {
        let payload = json!({
            "op": "unsubscribe",
            "args": symbols.iter().map(|s| format!("tickers.{}", s)).collect::<Vec<String>>(),
        });

        // Send the unsubscription message.
        let message = Message::Text(payload.to_string());
        Ok(self.sender.send(message).await?)
    }

    /// Receives the next message from the WebSocket connection.
    pub async fn next(&mut self) -> Result<BybitResponse, MessageError> {
        if let Some(result_msg) = self.receiver.next().await {
            return match result_msg {
                Ok(Message::Text(msg)) => serde_json::from_str::<BybitResponse>(&msg)
                    .map_err(|_| MessageError::UnsupportedMessage),
                Err(err) => match err {
                    TungsteniteError::Protocol(..) => Err(MessageError::ChannelClosed),
                    TungsteniteError::ConnectionClosed => Err(MessageError::ChannelClosed),
                    _ => Err(MessageError::UnsupportedMessage),
                },
                _ => Err(MessageError::UnsupportedMessage),
            };
        }

        Err(MessageError::ChannelClosed)
    }

    pub async fn close(&mut self) -> Result<(), SendError> {
        self.sender.close().await?;
        Ok(())
    }
}
