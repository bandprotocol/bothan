use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::error::Error as TungsteniteError;
use tokio_tungstenite::tungstenite::http::StatusCode;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tracing::warn;

use crate::api::error::Error;
use crate::api::types::message::{InstrumentType, Op, PriceRequestArgument, WebSocketMessage};
use crate::api::types::OkxResponse;

/// A connector for establishing a WebSocket connection to the OKX API.
pub struct OkxWebSocketConnector {
    url: String,
}

impl OkxWebSocketConnector {
    /// Creates a new instance of `OkxWebSocketConnector`.
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    /// Connects to the OKX WebSocket API.
    pub async fn connect(&self) -> Result<OkxWebSocketConnection, Error> {
        let (wss, resp) = connect_async(self.url.clone()).await?;

        let status = resp.status();
        if StatusCode::is_server_error(&status) || StatusCode::is_client_error(&status) {
            warn!("failed to connect with response code {}", resp.status());
            return Err(Error::ConnectionFailure(resp.status()));
        }

        Ok(OkxWebSocketConnection::new(wss))
    }
}

/// Represents an active WebSocket connection to the OKX API.
pub struct OkxWebSocketConnection {
    sender: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    receiver: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl OkxWebSocketConnection {
    /// Creates a new `OkxWebSocketConnection` instance.
    pub fn new(web_socket_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        let (sender, receiver) = web_socket_stream.split();
        Self { sender, receiver }
    }

    /// Subscribes to ticker updates for the given instrument IDs.
    pub async fn subscribe_ticker(&mut self, inst_ids: &[&str]) -> Result<(), Error> {
        let ticker_args = build_ticker_arguments(inst_ids);
        let msg = WebSocketMessage {
            op: Op::Subscribe,
            args: Some(ticker_args),
        };
        let message = Message::Text(serde_json::to_string(&msg)?);
        Ok(self.sender.send(message).await?)
    }

    /// Unsubscribes from ticker updates for the given instrument IDs.
    pub async fn unsubscribe_ticker(&mut self, inst_ids: &[&str]) -> Result<(), Error> {
        let ticker_args = build_ticker_arguments(inst_ids);
        let msg = WebSocketMessage {
            op: Op::Unsubscribe,
            args: Some(ticker_args),
        };
        let message = Message::Text(serde_json::to_string(&msg)?);
        Ok(self.sender.send(message).await?)
    }

    /// Receives the next message from the WebSocket connection.
    pub async fn next(&mut self) -> Result<OkxResponse, Error> {
        if let Some(result_msg) = self.receiver.next().await {
            return match result_msg {
                Ok(Message::Text(msg)) => {
                    serde_json::from_str::<OkxResponse>(&msg).map_err(|_| Error::UnsupportedMessage)
                }
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

/// Builds a ticker request with the given parameters.
fn build_ticker_arguments(inst_ids: &[&str]) -> Vec<PriceRequestArgument> {
    let inst_ids = inst_ids
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    inst_ids
        .iter()
        .map(|id| PriceRequestArgument {
            channel: "tickers".to_string(),
            inst_type: Some(InstrumentType::Spot),
            inst_family: None,
            inst_id: Some(id.clone()),
        })
        .collect()
}
