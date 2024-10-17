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
            return Err(ConnectionError::UnsuccessfulWebSocketResponse(
                resp.status(),
            ));
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
                            .map_err(|_| MessageError::UnsupportedMessage),
                        Err(_) => Err(MessageError::UnsupportedMessage),
                    }
                }
                Ok(Message::Text(msg)) => serde_json::from_str::<HtxResponse>(&msg)
                    .map_err(|_| MessageError::UnsupportedMessage),
                Err(TungsteniteError::Protocol(_)) | Err(TungsteniteError::ConnectionClosed) => {
                    Err(MessageError::ChannelClosed)
                }
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

#[cfg(test)]
pub(crate) mod test {
    use super::*;
    use crate::api::types::{DataUpdate, HtxResponse, Ping, SubResponse, Tick, UnsubResponse};
    use tokio::sync::mpsc;
    use ws_mock::ws_mock_server::{WsMock, WsMockServer};

    pub(crate) async fn setup_mock_server() -> WsMockServer {
        WsMockServer::start().await
    }

    #[tokio::test]
    async fn test_recv_ping() {
        // Set up the mock server and the WebSocket connector.
        let server = setup_mock_server().await;
        let connector = HtxWebSocketConnector::new(server.uri().await);
        let (mpsc_send, mpsc_recv) = mpsc::channel::<Message>(32);

        // Create a mock ping response.
        let mock_ping = Ping {
            ping: 1492420473027,
        };
        let mock_resp = HtxResponse::Ping(mock_ping);

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
        let connector = HtxWebSocketConnector::new(server.uri().await);
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

    #[tokio::test]
    async fn test_recv_sub_response() {
        // Set up the mock server and the WebSocket connector.
        let server = setup_mock_server().await;
        let connector = HtxWebSocketConnector::new(server.uri().await);
        let (mpsc_send, mpsc_recv) = mpsc::channel::<Message>(32);

        // Create a mock subscribe response.
        let mock_sub_resp = SubResponse {
            id: Some("id1".to_string()),
            status: "ok".to_string(),
            subbed: "market.btcusdt.kline.1min".to_string(),
            timestamp: 1489474081631,
        };
        let mock_resp = HtxResponse::SubResponse(mock_sub_resp);

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
    async fn test_recv_unsub_response() {
        // Set up the mock server and the WebSocket connector.
        let server = setup_mock_server().await;
        let connector = HtxWebSocketConnector::new(server.uri().await);
        let (mpsc_send, mpsc_recv) = mpsc::channel::<Message>(32);

        // Create a mock unsubscribe response.
        let mock_unsub_resp = UnsubResponse {
            id: Some("id4".to_string()),
            status: "ok".to_string(),
            unsubbed: "market.btcusdt.trade.detail".to_string(),
            timestamp: 1494326028889,
        };
        let mock_resp = HtxResponse::UnsubResponse(mock_unsub_resp);

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
    async fn test_recv_data_update() {
        // Set up the mock server and the WebSocket connector.
        let server = setup_mock_server().await;
        let connector = HtxWebSocketConnector::new(server.uri().await);
        let (mpsc_send, mpsc_recv) = mpsc::channel::<Message>(32);

        // Create a mock data update response.
        let mock_tick = Tick {
            open: 51732.0,
            high: 52785.64,
            low: 51000.0,
            close: 52735.63,
            amount: 13259.24137056181,
            vol: 687640987.4125315,
            count: 448737,
            bid: 52732.88,
            bid_size: 0.036,
            ask: 52732.89,
            ask_size: 0.583653,
            last_price: 52735.63,
            last_size: 0.03,
        };

        // Create the mock data update
        let mock_data_update = DataUpdate {
            ch: "market.btcusdt.ticker".to_string(),
            timestamp: 1630982370526,
            tick: mock_tick,
        };

        let mock_resp = HtxResponse::DataUpdate(mock_data_update);

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
}
