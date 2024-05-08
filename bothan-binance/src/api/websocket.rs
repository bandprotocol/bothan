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

/// Binance WebSocket Connector
pub struct BinanceWebSocketConnector {
    url: String,
}

impl BinanceWebSocketConnector {
    /// Create a new BinanceWebSocketConnector
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    /// Creates a new connection to the Binance WebSocket while returning a `BinanceWebSocketConnection`.
    /// If the connection fails, an `Error` is returned.
    pub async fn connect(&self) -> Result<BinanceWebSocketConnection, Error> {
        let (wss, resp) = connect_async(self.url.clone()).await?;

        let status = resp.status();
        if StatusCode::is_server_error(&status) || StatusCode::is_client_error(&status) {
            warn!("Failed to connect with response code {}", resp.status());
            return Err(Error::ConnectionFailure(resp.status()));
        }

        Ok(BinanceWebSocketConnection::new(wss))
    }
}

/// Binance WebSocket Connection
pub struct BinanceWebSocketConnection {
    sender: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    receiver: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl BinanceWebSocketConnection {
    /// Create a new BinanceWebSocketConnection
    pub fn new(web_socket_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        let (sender, receiver) = web_socket_stream.split();
        Self { sender, receiver }
    }

    /// Subscribes to a list of symbols. If the subscription fails, an `Error` is returned.
    /// Equivalent to subscribing to the `miniTicker` stream.
    pub async fn subscribe(&mut self, ids: &[&str]) -> Result<(), Error> {
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
        Ok(self.sender.send(message).await?)
    }

    /// Unsubscribes from a list of symbols. If unable to subscribe, an `Error` is returned.
    /// Equivalent to unsubscribing from the `miniTicker` stream.
    pub async fn unsubscribe(&mut self, ids: &[&str]) -> Result<(), Error> {
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
        Ok(self.sender.send(message).await?)
    }

    /// Awaits and returns the next message from the WebSocket connection. If the message is
    /// successfully received, a `BinanceResponse` is returned.
    pub async fn next(&mut self) -> Result<BinanceResponse, Error> {
        if let Some(result_msg) = self.receiver.next().await {
            return match result_msg {
                Ok(Message::Text(msg)) => Ok(serde_json::from_str::<BinanceResponse>(&msg)?),
                Ok(Message::Ping(_)) => Ok(BinanceResponse::Ping),
                Ok(Message::Close(_)) => Err(Error::ChannelClosed),
                _ => Err(Error::UnsupportedMessage),
            };
        }

        Err(Error::ChannelClosed)
    }
}

#[cfg(test)]
pub(crate) mod test {
    use crate::api::types::{Data, MiniTickerInfo, StreamResponse};
    use tokio::sync::mpsc;
    use ws_mock::ws_mock_server::{WsMock, WsMockServer};

    use super::*;

    pub(crate) async fn setup_mock_server() -> WsMockServer {
        WsMockServer::start().await
    }

    #[tokio::test]
    async fn test_recv_ticker() {
        let server = setup_mock_server().await;
        let connector = BinanceWebSocketConnector::new(server.uri().await);
        let (mpsc_send, mpsc_recv) = mpsc::channel::<Message>(32);
        let mock_ticker = MiniTickerInfo {
            event_time: 10000,
            symbol: "BTC".to_string(),
            close_price: "420000".to_string(),
            open_price: "420001".to_string(),
            high_price: "420003".to_string(),
            low_price: "4200".to_string(),
            base_volume: "1100000213".to_string(),
            quote_volume: "1231".to_string(),
        };
        let mock_resp = StreamResponse {
            stream: "btc@miniTicker".to_string(),
            data: Data::MiniTicker(mock_ticker),
        };

        WsMock::new()
            .forward_from_channel(mpsc_recv)
            .mount(&server)
            .await;
        mpsc_send
            .send(Message::Text(serde_json::to_string(&mock_resp).unwrap()))
            .await
            .unwrap();

        let mut connection = connector.connect().await.unwrap();

        let resp = connection.next().await.unwrap();
        assert_eq!(resp, BinanceResponse::Stream(mock_resp));
    }

    #[tokio::test]
    async fn test_recv_ping() {
        let server = setup_mock_server().await;
        let connector = BinanceWebSocketConnector::new(server.uri().await);
        let (mpsc_send, mpsc_recv) = mpsc::channel::<Message>(32);

        WsMock::new()
            .forward_from_channel(mpsc_recv)
            .mount(&server)
            .await;

        mpsc_send.send(Message::Ping(vec![])).await.unwrap();

        let mut connection = connector.connect().await.unwrap();

        let resp = connection.next().await.unwrap();
        assert_eq!(resp, BinanceResponse::Ping);
    }

    #[tokio::test]
    async fn test_recv_close() {
        let server = setup_mock_server().await;
        let connector = BinanceWebSocketConnector::new(server.uri().await);
        let (mpsc_send, mpsc_recv) = mpsc::channel::<Message>(32);

        WsMock::new()
            .forward_from_channel(mpsc_recv)
            .mount(&server)
            .await;

        mpsc_send.send(Message::Close(None)).await.unwrap();

        let mut connection = connector.connect().await.unwrap();

        let resp = connection.next().await;
        assert!(resp.is_err());
    }
}
