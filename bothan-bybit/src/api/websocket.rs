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

#[cfg(test)]
pub(crate) mod test {
    use tokio::sync::mpsc;
    use ws_mock::ws_mock_server::{WsMock, WsMockServer};

    // use crate::api::msgs::{Data, MiniTickerInfo, StreamResponse};
    use crate::api::types::{BybitResponse, PublicMessageResponse, PublicTickerResponse, Ticker};

    use super::*;

    pub(crate) async fn setup_mock_server() -> WsMockServer {
        WsMockServer::start().await
    }

    #[tokio::test]
    async fn test_recv_public_ticker() {
        // Set up the mock server and the WebSocket connector.
        let server = setup_mock_server().await;
        let connector = BybitWebSocketConnector::new(server.uri().await);
        let (mpsc_send, mpsc_recv) = mpsc::channel::<Message>(32);

        // Create a mock ticker response.
        let mock_ticker = Ticker {
            symbol: "BTCUSDT".to_string(),
            last_price: "42000.00".to_string(),
            high_price24h: "44000.00".to_string(),
            low_price24h: "40000.00".to_string(),
            prev_price24h: "40000.00".to_string(),
            volume24h: "100000.00".to_string(),
            turnover24h: "4200000000.00".to_string(),
            price24h_pcnt: "0.05".to_string(),
            usd_index_price: "42000.00".to_string(),
        };

        // Create the mock PublicTickerResponse.
        let mock_resp = BybitResponse::PublicTicker(PublicTickerResponse {
            topic: "tickers.BTCUSDT".to_string(),
            ts: 1673853746003,
            ticker_type: "snapshot".to_string(),
            cs: 2588407389,
            data: mock_ticker,
        });

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

        // Assert that the received response matches the mock response.
        assert_eq!(resp, mock_resp);
    }

    /// Test for receiving a public message response from the WebSocket.
    #[tokio::test]
    async fn test_recv_public_message() {
        // Set up the mock server and the WebSocket connector.
        let server = setup_mock_server().await;
        let connector = BybitWebSocketConnector::new(server.uri().await);
        let (mpsc_send, mpsc_recv) = mpsc::channel::<Message>(32);

        // Create a mock public message response.
        let mock_message = PublicMessageResponse {
            success: true,
            ret_msg: "subscribe".to_string(),
            conn_id: "2324d924-aa4d-45b0-a858-7b8be29ab52b".to_string(),
            req_id: Some("10001".to_string()),
            op: "subscribe".to_string(),
        };

        // Create the mock BybitResponse with the PublicMessageResponse.
        let mock_resp = BybitResponse::PublicMessage(mock_message);

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

        // Assert that the received response matches the mock response.
        assert_eq!(resp, mock_resp);
    }

    #[tokio::test]
    async fn test_recv_close() {
        // Set up the mock server and the WebSocket connector.
        let server = setup_mock_server().await;
        let connector = BybitWebSocketConnector::new(server.uri().await);
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
