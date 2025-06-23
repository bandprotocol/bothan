//! Coinbase WebSocket API client implementation.
//!
//! This module provides the [`WebSocketConnector`] and [`WebSocketConnection`] for interacting
//! with the Coinbase WebSocket API. It enables real-time streaming of market data, such as ticker
//! updates, and is used internally to implement the [`AssetInfoProvider`] trait for asset workers.
//!
//! This module provides:
//!
//! - Establishes WebSocket connections to Coinbase servers
//! - Subscribes and unsubscribes to ticker streams for specified symbols
//! - Processes incoming WebSocket messages, including ticker updates and ping events
//! - Transforms WebSocket messages into [`AssetInfo`] for use in workers
//! - Handles connection management, including closing connections gracefully

use bothan_lib::types::AssetInfo;
use bothan_lib::worker::websocket::{AssetInfoProvider, AssetInfoProviderConnector, Data};
use futures_util::{SinkExt, StreamExt};
use rust_decimal::Decimal;
use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite};
use tracing::warn;

use crate::api::Ticker;
use crate::api::error::{Error, ListeningError};
use crate::api::types::channels::Channel;
use crate::api::types::{DEFAULT_URL, Response};

/// A connector for establishing a WebSocket connection to the Coinbase API.
pub struct WebSocketConnector {
    url: String,
}

/// This struct is responsible for creating a WebSocket connection to the Coinbase API.
///
/// # Examples
///
/// ```rust
/// use bothan_coinbase::api::websocket::WebSocketConnector;
///
/// let connector = WebSocketConnector::new("wss://ws-feed.exchange.coinbase.com");
/// let connection = connector.connect();
/// ```
impl WebSocketConnector {
    /// Creates a new `CoinbaseWebSocketConnector`.
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    /// Connects to the WebSocket and returns a `CoinbaseWebSocketConnection`.
    pub async fn connect(&self) -> Result<WebSocketConnection, tungstenite::Error> {
        let (wss, _) = connect_async(&self.url).await?;

        Ok(WebSocketConnection::new(wss))
    }
}

#[async_trait::async_trait]
impl AssetInfoProviderConnector for WebSocketConnector {
    type Provider = WebSocketConnection;
    type Error = tungstenite::Error;

    /// Connects to the WebSocket and returns a `WebSocketConnection`.
    async fn connect(&self) -> Result<WebSocketConnection, Self::Error> {
        WebSocketConnector::connect(self).await
    }
}

impl Default for WebSocketConnector {
    /// Creates a default `CoinbaseWebSocketConnector` with the default URL.
    fn default() -> Self {
        Self {
            url: DEFAULT_URL.to_string(),
        }
    }
}

/// A connection to the Coinbase WebSocket API.
pub struct WebSocketConnection {
    ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

/// Represents a WebSocket connection to the Coinbase API.
/// This struct provides methods for subscribing and unsubscribing to channels,
/// receiving messages, and closing the connection.
impl WebSocketConnection {
    /// Creates a new `CoinbaseWebSocketConnection`.
    pub fn new(ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        Self { ws_stream }
    }

    /// Subscribes to the specified channels and product IDs.
    ///
    /// This method sends a subscription request to the WebSocket server for the specified channels
    /// and product IDs.
    ///
    /// # Parameters
    ///
    /// - `channels`: A vector of channels to subscribe to.
    /// - `product_ids`: A slice of product IDs to subscribe to.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating success or failure of the subscription request.
    ///
    /// # Errors
    ///
    /// Returns a [`tungstenite::Error`] if the WebSocket subscription request fails.  
    pub async fn subscribe<T: AsRef<str>>(
        &mut self,
        channels: Vec<Channel>,
        product_ids: &[T],
    ) -> Result<(), tungstenite::Error> {
        let request = json!(
            {
                "type": "subscribe",
                "product_ids": product_ids.iter().map(AsRef::as_ref).collect::<Vec<&str>>(),
                "channels": channels
            }
        );

        // Send the unsubscription message.
        let message = Message::Text(request.to_string());
        self.ws_stream.send(message).await
    }

    /// Unsubscribes from the specified channels and product IDs.
    ///
    /// This method sends an unsubscription request to the WebSocket server for the specified channels
    /// and product IDs.
    ///
    /// # Parameters
    ///
    /// - `channels`: A vector of channels to unsubscribe from.
    /// - `product_ids`: A slice of product IDs to unsubscribe from.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating success or failure of the unsubscription request.
    ///
    /// # Errors
    ///
    /// Returns a [`tungstenite::Error`] if the WebSocket unsubscription request fails.
    pub async fn unsubscribe<T: AsRef<str>>(
        &mut self,
        channels: Vec<Channel>,
        product_ids: &[T],
    ) -> Result<(), tungstenite::Error> {
        let request = json!(
            {
                "type": "unsubscribe",
                "product_ids": product_ids.iter().map(AsRef::as_ref).collect::<Vec<&str>>(),
                "channels": channels
            }
        );

        // Send the unsubscription message.
        let message = Message::Text(request.to_string());
        self.ws_stream.send(message).await
    }

    /// Receives the next message from the WebSocket.
    ///
    /// This method listens for incoming messages from the WebSocket stream and processes them.
    /// It handles different message types, including text messages, ping messages, and close messages.
    pub async fn next(&mut self) -> Option<Result<Response, Error>> {
        match self.ws_stream.next().await {
            Some(Ok(Message::Text(msg))) => Some(parse_msg(msg)),
            Some(Ok(Message::Ping(_))) => Some(Ok(Response::Ping)),
            Some(Ok(Message::Close(_))) => None,
            Some(Ok(_)) => Some(Err(Error::UnsupportedWebsocketMessageType)),
            Some(Err(_)) => None, // Consider the connection closed if error detected
            None => None,
        }
    }

    /// Closes the WebSocket connection.
    ///
    /// This method sends a close message to the WebSocket server and waits for the connection to close.
    pub async fn close(&mut self) -> Result<(), tungstenite::Error> {
        self.ws_stream.close(None).await
    }
}

fn parse_msg(msg: String) -> Result<Response, Error> {
    Ok(serde_json::from_str::<Response>(&msg)?)
}

#[async_trait::async_trait]
impl AssetInfoProvider for WebSocketConnection {
    type SubscriptionError = tungstenite::Error;
    type ListeningError = ListeningError;

    /// Subscribes to the given list of asset IDs.
    ///
    /// This method sends a subscription request to the WebSocket server for the specified asset IDs.
    ///
    /// # Parameters
    /// - `ids`: A slice of asset IDs to subscribe to.
    ///
    /// # Returns
    /// Returns a `Result` indicating success or failure of the subscription request.
    ///
    /// # Errors
    /// If the WebSocket connection fails to send the subscription message, it returns a `tungstenite::Error`.
    async fn subscribe(&mut self, ids: &[String]) -> Result<(), Self::SubscriptionError> {
        self.subscribe(vec![Channel::Ticker], ids).await?;
        Ok(())
    }

    /// Receives the next asset info update from the WebSocket connection.
    ///
    /// This method listens for incoming messages from the WebSocket stream and processes them.
    /// It handles different message types, including ticker updates, ping messages, and error messages.
    ///
    /// # Returns
    /// Returns an `Option` containing a `Result` with the parsed `Data` or an error.
    ///
    /// # Errors
    /// If the message cannot be parsed or if there is an error in processing the ticker data,
    async fn next(&mut self) -> Option<Result<Data, Self::ListeningError>> {
        WebSocketConnection::next(self).await.map(|r| {
            Ok(match r? {
                Response::Ticker(t) => parse_ticker(t)?,
                Response::Ping => Data::Ping,
                Response::Error(e) => {
                    warn!("received error in response: {:?}", e);
                    Data::Unused
                }
                _ => Data::Unused,
            })
        })
    }

    /// Attempts to close the WebSocket connection.
    ///
    /// This method spawns a task to close the WebSocket connection asynchronously.
    async fn try_close(mut self) {
        tokio::spawn(async move { self.close().await });
    }
}

fn parse_ticker(ticker: Box<Ticker>) -> Result<Data, ListeningError> {
    let asset_info = AssetInfo::new(
        ticker.product_id,
        Decimal::from_str_exact(&ticker.price)?,
        chrono::DateTime::parse_from_rfc3339(&ticker.time)?.timestamp(),
    );
    Ok(Data::AssetInfo(vec![asset_info]))
}

#[cfg(test)]
pub(crate) mod test {
    use tokio::sync::mpsc;
    use ws_mock::ws_mock_server::{WsMock, WsMockServer};

    use super::*;
    use crate::api::Ticker;
    use crate::api::types::Response;

    pub(crate) async fn setup_mock_server() -> WsMockServer {
        WsMockServer::start().await
    }

    #[tokio::test]
    async fn test_recv_ticker() {
        // Set up the mock server and the WebSocket connector.
        let server = setup_mock_server().await;
        let connector = WebSocketConnector::new(server.uri().await);
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
        let mock_resp = Response::Ticker(Box::new(mock_ticker));

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
        let resp = connection.next().await.unwrap().unwrap();
        assert_eq!(resp, mock_resp);
    }

    #[tokio::test]
    async fn test_recv_close() {
        // Set up the mock server and the WebSocket connector.
        let server = setup_mock_server().await;
        let connector = WebSocketConnector::new(server.uri().await);
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
        assert!(resp.is_none());
    }
}
