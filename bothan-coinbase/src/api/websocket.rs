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

pub struct CoinbaseWebSocketConnector {
    url: String,
}

impl CoinbaseWebSocketConnector {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    pub async fn connect(&self) -> Result<CoinbaseWebSocketConnection, Error> {
        let (wss, resp) = connect_async(self.url.clone()).await?;

        let status = resp.status();
        if StatusCode::is_server_error(&status) || StatusCode::is_client_error(&status) {
            warn!("Failed to connect with response code {}", resp.status());
            return Err(Error::ConnectionFailure(resp.status()));
        }

        Ok(CoinbaseWebSocketConnection::new(wss))
    }
}

impl Default for CoinbaseWebSocketConnector {
    fn default() -> Self {
        Self {
            url: DEFAULT_URL.to_string(),
        }
    }
}

pub struct CoinbaseWebSocketConnection {
    sender: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    receiver: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl CoinbaseWebSocketConnection {
    pub fn new(web_socket_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        let (sender, receiver) = web_socket_stream.split();
        Self { sender, receiver }
    }

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
