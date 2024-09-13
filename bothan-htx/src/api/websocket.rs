use std::io::Read;

use flate2::read::GzDecoder;
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
use crate::api::types::{HtxResponse, Pong};

/// A connector for establishing a WebSocket connection to the Htx API.
pub struct HtxWebSocketConnector {
    url: String,
}

impl HtxWebSocketConnector {
    /// Creates a new instance of `HtxWebSocketConnector`.
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    /// Connects to the Htx WebSocket API.
    pub async fn connect(&self) -> Result<HtxWebSocketConnection, ConnectionError> {
        let (wss, resp) = connect_async(self.url.clone()).await?;

        let status = resp.status();
        if StatusCode::is_server_error(&status) || StatusCode::is_client_error(&status) {
            warn!("failed to connect with response code {}", resp.status());
            return Err(ConnectionError::UnsuccessfulHttpResponse(resp.status()));
        }

        Ok(HtxWebSocketConnection::new(wss))
    }
}

/// Represents an active WebSocket connection to the Htx API.
pub struct HtxWebSocketConnection {
    sender: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    receiver: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl HtxWebSocketConnection {
    /// Creates a new `HtxWebSocketConnection` instance.
    pub fn new(web_socket_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        let (sender, receiver) = web_socket_stream.split();
        Self { sender, receiver }
    }

    /// Subscribes to ticker updates for a single symbol.
    pub async fn subscribe_ticker(&mut self, symbol: &str) -> Result<(), SendError> {
        let formatted_symbol = format!("market.{}.ticker", symbol);
        let payload = json!({
            "sub": formatted_symbol,
        });

        // Send the subscription message.
        let message = Message::Text(payload.to_string());
        self.sender.send(message).await?;
        Ok(())
    }

    /// Unsubscribes from ticker updates for a single symbol.
    pub async fn unsubscribe_ticker(&mut self, symbol: &str) -> Result<(), SendError> {
        let formatted_symbol = format!("market.{}.ticker", symbol);
        let payload = json!({
            "unsub": formatted_symbol,
        });

        // Send the unsubscription message.
        let message = Message::Text(payload.to_string());
        self.sender.send(message).await?;
        Ok(())
    }

    /// Sends a Pong message in response to a Ping message.
    pub async fn send_pong(&mut self, pong: Pong) -> Result<(), SendError> {
        let payload = json!({
            "pong": pong.pong,
        });

        // Send the pong message.
        let message = Message::Text(payload.to_string());
        self.sender.send(message).await?;
        Ok(())
    }

    /// Receives the next message from the WebSocket connection.
    pub async fn next(&mut self) -> Result<HtxResponse, MessageError> {
        if let Some(result_msg) = self.receiver.next().await {
            return match result_msg {
                Ok(Message::Binary(msg)) => {
                    // Decompress the gzip-compressed message
                    let mut decoder = GzDecoder::new(&msg[..]);
                    let mut decompressed_msg = String::new();
                    match decoder.read_to_string(&mut decompressed_msg) {
                        Ok(_) => serde_json::from_str::<HtxResponse>(&decompressed_msg)
                            .map_err(|e| MessageError::UnsupportedMessage),
                        Err(_) => Err(MessageError::UnsupportedMessage),
                    }
                }
                Ok(Message::Text(msg)) => serde_json::from_str::<HtxResponse>(&msg)
                    .map_err(|_| MessageError::UnsupportedMessage),
                Err(err) => match err {
                    TungsteniteError::Protocol(..) | TungsteniteError::ConnectionClosed => {
                        Err(MessageError::ChannelClosed)
                    }
                    _ => Err(MessageError::UnsupportedMessage),
                },
                _ => Err(MessageError::UnsupportedMessage),
            };
        }

        Err(MessageError::ChannelClosed)
    }

    /// Closes the WebSocket connection.
    pub async fn close(&mut self) -> Result<(), SendError> {
        self.sender.close().await?;
        Ok(())
    }
}
