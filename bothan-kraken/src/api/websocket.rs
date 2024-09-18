use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::error::Error as TungsteniteError;
use tokio_tungstenite::tungstenite::http::StatusCode;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tracing::warn;

use crate::api::error::{ConnectionError, MessageError, SendError};
use crate::api::types::channel::ticker::{EventTrigger, TickerRequestParameters};
use crate::api::types::message::{Method, PublicMessage};
use crate::api::types::KrakenResponse;

/// A connector for establishing a WebSocket connection to the Kraken API.
pub struct KrakenWebSocketConnector {
    url: String,
}

impl KrakenWebSocketConnector {
    /// Creates a new instance of `KrakenWebSocketConnector`.
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    /// Connects to the Kraken WebSocket API.
    pub async fn connect(&self) -> Result<KrakenWebSocketConnection, ConnectionError> {
        let (wss, resp) = connect_async(self.url.clone()).await?;

        let status = resp.status();
        if StatusCode::is_server_error(&status) || StatusCode::is_client_error(&status) {
            warn!("failed to connect with response code {}", resp.status());
            return Err(ConnectionError::UnsuccessfulWebSocketResponse(
                resp.status(),
            ));
        }

        Ok(KrakenWebSocketConnection::new(wss))
    }
}

/// Represents an active WebSocket connection to the Kraken API.
pub struct KrakenWebSocketConnection {
    sender: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    receiver: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl KrakenWebSocketConnection {
    /// Creates a new `KrakenWebSocketConnection` instance.
    pub fn new(web_socket_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        let (sender, receiver) = web_socket_stream.split();
        Self { sender, receiver }
    }

    /// Sends a ping message to the WebSocket server.
    pub async fn ping(&mut self) -> Result<(), SendError> {
        let msg = Message::Ping("".into());
        Ok(self.sender.send(msg).await?)
    }

    /// Subscribes to ticker updates for the given symbols.
    pub async fn subscribe_ticker(
        &mut self,
        symbols: &[&str],
        event_trigger: Option<EventTrigger>,
        snapshot: Option<bool>,
    ) -> Result<(), SendError> {
        let ticker_param = build_ticker_request(symbols, event_trigger, snapshot);
        let msg = PublicMessage {
            method: Method::Subscribe,
            params: Some(ticker_param),
            req_id: None,
        };

        // Send the unsubscription message.
        let message = Message::Text(serde_json::to_string(&msg)?);
        Ok(self.sender.send(message).await?)
    }

    /// Unsubscribes from ticker updates for the given symbols.
    pub async fn unsubscribe_ticker(&mut self, symbols: &[&str]) -> Result<(), SendError> {
        let params = build_ticker_request(symbols, None, None);
        let msg = PublicMessage {
            method: Method::Unsubscribe,
            params: Some(params),
            req_id: None,
        };
        // Send the unsubscription message.
        let message = Message::Text(serde_json::to_string(&msg)?);
        Ok(self.sender.send(message).await?)
    }

    /// Receives the next message from the WebSocket connection.
    pub async fn next(&mut self) -> Result<KrakenResponse, MessageError> {
        if let Some(result_msg) = self.receiver.next().await {
            return match result_msg {
                Ok(Message::Text(msg)) => serde_json::from_str::<KrakenResponse>(&msg)
                    .map_err(|_| MessageError::UnsupportedMessage),
                Ok(Message::Pong(_)) => Ok(KrakenResponse::Pong),
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

/// Builds a ticker request with the given parameters.
fn build_ticker_request(
    symbols: &[&str],
    event_trigger: Option<EventTrigger>,
    snapshot: Option<bool>,
) -> TickerRequestParameters {
    let symbols = symbols
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    TickerRequestParameters {
        channel: "ticker".to_string(),
        symbol: symbols,
        event_trigger,
        snapshot,
    }
}

#[cfg(test)]
pub(crate) mod test {
    use tokio::sync::mpsc;
    use ws_mock::ws_mock_server::{WsMock, WsMockServer};

    use crate::api::types::{ChannelResponse, KrakenResponse, TickerResponse};

    use super::*;

    pub(crate) async fn setup_mock_server() -> WsMockServer {
        WsMockServer::start().await
    }

    #[tokio::test]
    async fn test_recv_ticker() {
        // Set up the mock server and the WebSocket connector.
        let server = setup_mock_server().await;
        let connector = KrakenWebSocketConnector::new(server.uri().await);
        let (mpsc_send, mpsc_recv) = mpsc::channel::<Message>(32);

        // Create a mock ticker response.
        let mock_ticker = TickerResponse {
            symbol: "BTC".to_string(),
            bid: 42000.00,
            bid_qty: 50000.00,
            ask: 42001.00,
            ask_qty: 50000.00,
            last: 42000.00,
            volume: 100000.00,
            vwap: 42000.00,
            low: 40000.00,
            high: 44000.00,
            change: 2000.00,
            change_pct: 0.05,
        };
        let mock_resp = KrakenResponse::Channel(ChannelResponse::Ticker(vec![mock_ticker]));

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
    async fn test_recv_pong() {
        // Set up the mock server and the WebSocket connector.
        let server = setup_mock_server().await;
        let connector = KrakenWebSocketConnector::new(server.uri().await);
        let (mpsc_send, mpsc_recv) = mpsc::channel::<Message>(32);

        // Mount the mock WebSocket server and send a ping message.
        WsMock::new()
            .forward_from_channel(mpsc_recv)
            .mount(&server)
            .await;
        mpsc_send.send(Message::Pong(vec![])).await.unwrap();

        // Connect to the mock WebSocket server and retrieve the ping response.
        let mut connection = connector.connect().await.unwrap();
        let resp = connection.next().await.unwrap();
        assert_eq!(resp, KrakenResponse::Pong);
    }

    #[tokio::test]
    async fn test_recv_close() {
        // Set up the mock server and the WebSocket connector.
        let server = setup_mock_server().await;
        let connector = KrakenWebSocketConnector::new(server.uri().await);
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
