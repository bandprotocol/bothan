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
    pub async fn connect(&self) -> Result<CoinbaseWebSocketConnection, ConnectionError> {
        let (wss, resp) = connect_async(self.url.clone()).await?;

        let status = resp.status();
        if StatusCode::is_server_error(&status) || StatusCode::is_client_error(&status) {
            warn!("failed to connect with response code {}", resp.status());
            return Err(ConnectionError::UnsuccessfulHttpResponse(resp.status()));
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
    ) -> Result<(), SendError> {
        let request = Request {
            type_: RequestType::Subscribe,
            product_ids: product_ids.iter().map(|id| id.to_string()).collect(),
            channels,
        };

        // Create the subscription payload.
        let payload = json!(request);

        // Send the subscription message.
        let message = Message::Text(payload.to_string());
        Ok(self.sender.send(message).await?)
    }

    /// Unsubscribes from the specified channels and product IDs.
    pub async fn unsubscribe(
        &mut self,
        channels: Vec<Channel>,
        product_ids: &[&str],
    ) -> Result<(), SendError> {
        let request = Request {
            type_: RequestType::Unsubscribe,
            product_ids: product_ids.iter().map(|id| id.to_string()).collect(),
            channels,
        };

        // Create the subscription payload.
        let payload = json!(request);

        // Send the subscription message.
        let message = Message::Text(payload.to_string());
        Ok(self.sender.send(message).await?)
    }

    /// Receives the next message from the WebSocket.
    pub async fn next(&mut self) -> Result<CoinbaseResponse, MessageError> {
        if let Some(result_msg) = self.receiver.next().await {
            return match result_msg {
                Ok(Message::Text(msg)) => serde_json::from_str::<CoinbaseResponse>(&msg)
                    .map_err(|_| MessageError::UnsupportedMessage),
                Ok(Message::Close(_)) => Err(MessageError::ChannelClosed),
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

#[cfg(test)]
pub(crate) mod test {
    use tokio::sync::mpsc;
    use ws_mock::ws_mock_server::{WsMock, WsMockServer};

    use crate::api::types::{CoinbaseResponse, Ticker};

    use super::*;

    pub(crate) async fn setup_mock_server() -> WsMockServer {
        WsMockServer::start().await
    }

    #[tokio::test]
    async fn test_recv_ticker() {
        // Set up the mock server and the WebSocket connector.
        let server = setup_mock_server().await;
        let connector = CoinbaseWebSocketConnector::new(server.uri().await);
        let (mpsc_send, mpsc_recv) = mpsc::channel::<Message>(32);

        // Create a mock ticker response.
        let mock_ticker = Ticker {
            sequence: 1,
            product_id: "BTC-USD".to_string(),
            price: "10000.00".to_string(),
            open_24h: "9000.00".to_string(),
            volume_24h: "1000.00".to_string(),
            low_24h: "9500.00".to_string(),
            high_24h: "10500.00".to_string(),
            volume_30d: "30000.00".to_string(),
            best_bid: "9999.00".to_string(),
            best_bid_size: "0.01".to_string(),
            best_ask: "10001.00".to_string(),
            best_ask_size: "0.01".to_string(),
            side: "buy".to_string(),
            time: "2021-01-01T00:00:00.000Z".to_string(),
            trade_id: 1,
            last_size: "0.01".to_string(),
        };
        let mock_resp = CoinbaseResponse::Ticker(Box::new(mock_ticker));

        // Mount the mock WebSocket server and send the mock response.
        WsMock::new()
            .forward_from_channel(mpsc_recv)
            .mount(&server)
            .await;
        mpsc_send
            .send(Message::Text(serde_json::to_string(&mock_resp).unwrap()))
            .await
            .unwrap();

        // Connect to the mock WebSocket server and retrieve the response.
        let mut connection = connector.connect().await.unwrap();
        let resp = connection.next().await.unwrap();
        assert_eq!(resp, mock_resp);
    }

    #[tokio::test]
    async fn test_recv_close() {
        // Set up the mock server and the WebSocket connector.
        let server = setup_mock_server().await;
        let connector = CoinbaseWebSocketConnector::new(server.uri().await);
        let (mpsc_send, mpsc_recv) = mpsc::channel::<Message>(32);

        // Mount the mock WebSocket server and send a close message.
        WsMock::new()
            .forward_from_channel(mpsc_recv)
            .mount(&server)
            .await;
        mpsc_send.send(Message::Close(None)).await.unwrap();

        // Connect to the mock WebSocket server and verify the connection closure.
        let mut connection = connector.connect().await.unwrap();
        let resp = connection.next().await;
        assert!(resp.is_err());
    }
}
