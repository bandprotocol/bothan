//! OKX WebSocket API client implementation.
//!
//! This module provides the [`WebSocketConnector`] and [`WebSocketConnection`] for interacting
//! with the OKX WebSocket API. It enables real-time streaming of market data, such as ticker
//! updates, and is used internally to implement the [`AssetInfoProvider`] trait for asset workers.
//!
//! This module provides:
//!
//! - Establishes WebSocket connections to OKX servers
//! - Subscribes and unsubscribes to ticker streams for specified symbols
//! - Processes incoming WebSocket messages and transforms them into [`AssetInfo`] for use in workers
//! - Handles connection management, including ping/pong keep-alive and graceful closing
//! - Supports OKX v5 WebSocket API format

use bothan_lib::types::AssetInfo;
use bothan_lib::worker::websocket::{AssetInfoProvider, AssetInfoProviderConnector, Data};
use futures_util::{SinkExt, StreamExt};
use rust_decimal::Decimal;
use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite};

use crate::api::error::{Error, ListeningError};
use crate::api::types::ticker::{InstrumentType, Ticker};
use crate::api::types::{Response, subscription, ticker};

/// A connector for establishing WebSocket connections to the OKX WebSocket API.
///
/// The `WebSocketConnector` provides methods to create a new connector and connect to the WebSocket server.
/// It handles the initial connection setup and returns a `WebSocketConnection` for further operations.
///
/// # Examples
///
/// ```rust,no_run
/// use bothan_okx::api::websocket::WebSocketConnector;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let connector = WebSocketConnector::new("wss://ws.okx.com:8443/ws/v5/public");
///     let connection = connector.connect().await?;
///     Ok(())
/// }
/// ```
pub struct WebSocketConnector {
    /// The WebSocket URL for the OKX API.
    url: String,
}

impl WebSocketConnector {
    /// Creates a new instance of `WebSocketConnector` with the given URL.
    ///
    /// # Parameters
    ///
    /// - `url`: The WebSocket URL for the OKX API
    ///
    /// # Returns
    ///
    /// A new `WebSocketConnector` instance with the specified URL.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bothan_okx::api::websocket::WebSocketConnector;
    ///
    /// let connector = WebSocketConnector::new("wss://ws.okx.com:8443/ws/v5/public");
    /// ```
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    /// Connects to the OKX WebSocket API.
    ///
    /// This method establishes a WebSocket connection to the OKX server and returns
    /// a `WebSocketConnection` instance for further operations.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `WebSocketConnection` on success,
    /// or a `tungstenite::Error` if the connection fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bothan_okx::api::websocket::WebSocketConnector;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let connector = WebSocketConnector::new("wss://ws.okx.com:8443/ws/v5/public");
    ///     let connection = connector.connect().await?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `tungstenite::Error` if:
    /// - The WebSocket connection cannot be established
    /// - The URL is invalid
    /// - Network connectivity issues occur
    pub async fn connect(&self) -> Result<WebSocketConnection, tungstenite::Error> {
        let (wss, _) = connect_async(self.url.clone()).await?;

        Ok(WebSocketConnection::new(wss))
    }
}

#[async_trait::async_trait]
impl AssetInfoProviderConnector for WebSocketConnector {
    type Provider = WebSocketConnection;
    type Error = tungstenite::Error;

    /// Connects to the OKX WebSocket API and returns a `WebSocketConnection`.
    ///
    /// This method is part of the `AssetInfoProviderConnector` trait implementation,
    /// providing a standardized way to establish WebSocket connections for asset workers.
    async fn connect(&self) -> Result<WebSocketConnection, Self::Error> {
        WebSocketConnector::connect(self).await
    }
}

/// Represents an active WebSocket connection to the OKX API.
///
/// The `WebSocketConnection` encapsulates the WebSocket stream and provides methods for
/// subscribing to ticker streams, receiving messages, handling ping/pong keep-alive,
/// and closing the connection gracefully.
///
/// # Examples
///
/// ```rust,no_run
/// use bothan_okx::api::websocket::WebSocketConnection;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Assuming you have a connection
///     // let mut connection = /* ... */;
///     // connection.subscribe_ticker(&["BTC-USDT"]).await?;
///     Ok(())
/// }
/// ```
pub struct WebSocketConnection {
    /// The underlying WebSocket stream for communication with the OKX API.
    ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl WebSocketConnection {
    /// Creates a new `WebSocketConnection` instance.
    ///
    /// # Parameters
    ///
    /// - `ws_stream`: The WebSocket stream for communication with the OKX API
    ///
    /// # Returns
    ///
    /// A new `WebSocketConnection` instance.
    pub fn new(ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        Self { ws_stream }
    }

    /// Subscribes to ticker updates for the given instrument IDs.
    ///
    /// This method sends a subscription request to the OKX WebSocket API for the specified
    /// instrument IDs. The subscription is formatted according to the OKX v5 API specification.
    ///
    /// # Parameters
    ///
    /// - `inst_ids`: A slice of instrument identifiers to subscribe to (e.g., "BTC-USDT")
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating success or failure of the subscription request.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use bothan_okx::api::websocket::WebSocketConnection;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     // Assuming you have a connection
    ///     // let mut connection = /* ... */;
    ///     // connection.subscribe_ticker(&["BTC-USDT", "ETH-USDT"]).await?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `tungstenite::Error` if the subscription request fails.
    pub async fn subscribe_ticker<T: ToString>(
        &mut self,
        inst_ids: &[T],
    ) -> Result<(), tungstenite::Error> {
        let ticker_args = build_ticker_request(inst_ids);
        let msg = subscription::Request {
            op: subscription::Operation::Subscribe,
            args: Some(ticker_args),
        };

        // Send the subscription message.
        // Note: json!() should never panic here
        let message = Message::Text(json!(msg).to_string());
        self.ws_stream.send(message).await
    }

    /// Unsubscribes from ticker updates for the given instrument IDs.
    ///
    /// This method sends an unsubscription request to the OKX WebSocket API for the specified
    /// instrument IDs. The unsubscription is formatted according to the OKX v5 API specification.
    ///
    /// # Parameters
    ///
    /// - `inst_ids`: A slice of instrument identifiers to unsubscribe from (e.g., "BTC-USDT")
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating success or failure of the unsubscription request.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use bothan_okx::api::websocket::WebSocketConnection;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     // Assuming you have a connection
    ///     // let mut connection = /* ... */;
    ///     // connection.unsubscribe_ticker(&["BTC-USDT"]).await?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `tungstenite::Error` if the unsubscription request fails.
    pub async fn unsubscribe_ticker<T: ToString>(
        &mut self,
        inst_ids: &[T],
    ) -> Result<(), tungstenite::Error> {
        let ticker_args = build_ticker_request(inst_ids);
        let msg = subscription::Request {
            op: subscription::Operation::Unsubscribe,
            args: Some(ticker_args),
        };

        // Send the unsubscription message.
        // Note: json!() should never panic here
        let message = Message::Text(json!(msg).to_string());
        self.ws_stream.send(message).await
    }

    /// Receives the next message from the WebSocket connection.
    ///
    /// This method listens for incoming WebSocket messages and processes them.
    /// OKX sends text-based JSON messages, which are parsed into `Response` types.
    ///
    /// # Returns
    ///
    /// Returns an `Option<Result<Response, Error>>` where:
    /// - `Some(Ok(response))` contains a parsed response
    /// - `Some(Err(error))` contains a parsing or I/O error
    /// - `None` indicates the connection is closed or no message is available
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use bothan_okx::api::websocket::WebSocketConnection;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     // Assuming you have a connection
    ///     // let mut connection = /* ... */;
    ///     // if let Some(result) = connection.next().await {
    ///     //     match result {
    ///     //         Ok(response) => println!("Received: {:?}", response);
    ///     //         Err(error) => eprintln!("Error: {:?}", error);
    ///     //     }
    ///     // }
    ///     Ok(())
    /// }
    /// ```
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

    /// Closes the WebSocket connection gracefully.
    ///
    /// This method sends a close frame to the WebSocket server and waits for the connection to close.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating success or failure of closing the connection.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use bothan_okx::api::websocket::WebSocketConnection;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     // Assuming you have a connection
    ///     // let mut connection = /* ... */;
    ///     // connection.close().await?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `tungstenite::Error` if the connection cannot be closed properly.
    pub async fn close(&mut self) -> Result<(), tungstenite::Error> {
        self.ws_stream.close(None).await?;
        Ok(())
    }
}

/// Builds a ticker request with the given parameters.
///
/// This function creates ticker subscription requests for the OKX WebSocket API
/// based on the provided instrument IDs. It formats the requests according to
/// the OKX v5 API specification.
///
/// # Parameters
///
/// - `inst_ids`: A slice of instrument identifiers to create requests for
///
/// # Returns
///
/// A vector of ticker request objects ready for WebSocket transmission.
fn build_ticker_request<T: ToString>(inst_ids: &[T]) -> Vec<ticker::Request> {
    let inst_ids = inst_ids
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    inst_ids
        .into_iter()
        .map(|id| ticker::Request {
            channel: "tickers".to_string(),
            inst_type: Some(InstrumentType::Spot),
            inst_family: None,
            inst_id: Some(id),
        })
        .collect()
}

/// Parses a WebSocket message string into a `Response` type.
///
/// This function deserializes JSON messages from the OKX WebSocket API
/// into the appropriate `Response` enum variant.
///
/// # Parameters
///
/// - `msg`: The JSON message string to parse
///
/// # Returns
///
/// Returns a `Result` containing a parsed `Response` on success,
/// or an `Error` if parsing fails.
fn parse_msg(msg: String) -> Result<Response, Error> {
    Ok(serde_json::from_str::<Response>(&msg)?)
}

#[async_trait::async_trait]
impl AssetInfoProvider for WebSocketConnection {
    type SubscriptionError = tungstenite::Error;
    type ListeningError = ListeningError;

    /// Subscribes to asset information updates for the given list of asset IDs.
    ///
    /// This method sends subscription requests to the OKX WebSocket API for each asset ID.
    ///
    /// # Parameters
    ///
    /// - `ids`: A slice of asset identifiers to subscribe to
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating success or failure of the subscription requests.
    ///
    /// # Errors
    ///
    /// Returns a `tungstenite::Error` if any subscription request fails.
    async fn subscribe(&mut self, ids: &[String]) -> Result<(), Self::SubscriptionError> {
        self.subscribe_ticker(ids).await?;
        Ok(())
    }

    /// Processes the next message from the WebSocket stream.
    ///
    /// This method handles incoming messages from the OKX WebSocket API, including:
    /// - Market data updates (converted to `AssetInfo`)
    /// - Ping messages (responded to with pong)
    /// - Other message types (ignored)
    ///
    /// # Returns
    ///
    /// Returns an `Option<Result<Data, ListeningError>>` where:
    /// - `Some(Ok(data))` contains processed asset data or ping information
    /// - `Some(Err(error))` contains a processing error
    /// - `None` indicates no message is available
    async fn next(&mut self) -> Option<Result<Data, Self::ListeningError>> {
        WebSocketConnection::next(self).await.map(|r| match r? {
            Response::TickersChannel(data) => parse_tickers(data.data),
            Response::Ping => Ok(Data::Ping),
            _ => Ok(Data::Unused),
        })
    }

    /// Attempts to close the WebSocket connection gracefully.
    ///
    /// This method spawns a background task to close the connection,
    /// ensuring that the close operation doesn't block the current thread.
    async fn try_close(mut self) {
        tokio::spawn(async move { self.close().await });
    }
}

/// Parses ticker data from the OKX WebSocket API into `AssetInfo`.
///
/// This function converts a vector of OKX ticker data into `AssetInfo` instances
/// for use in asset workers. It extracts the instrument ID, last price, and timestamp
/// from each ticker.
///
/// # Parameters
///
/// - `tickers`: A vector of ticker data from the OKX API
///
/// # Returns
///
/// Returns a `Result` containing `AssetInfo` data on success,
/// or a `ListeningError` if parsing fails.
///
/// # Errors
///
/// Returns a `ListeningError` if:
/// - The price data contains invalid values
/// - The timestamp cannot be parsed
fn parse_tickers(tickers: Vec<Ticker>) -> Result<Data, ListeningError> {
    Ok(Data::AssetInfo(
        tickers
            .into_iter()
            .map(parse_ticker)
            .collect::<Result<Vec<AssetInfo>, ListeningError>>()?,
    ))
}

/// Parses a single ticker from the OKX WebSocket API into `AssetInfo`.
///
/// This function extracts the asset identifier, last price, and timestamp from
/// an OKX ticker and creates an `AssetInfo` instance.
///
/// # Parameters
///
/// - `ticker`: The ticker data from the OKX API
///
/// # Returns
///
/// Returns a `Result` containing `AssetInfo` data on success,
/// or a `ListeningError` if parsing fails.
///
/// # Errors
///
/// Returns a `ListeningError` if:
/// - The price data contains invalid values
/// - The timestamp cannot be parsed
fn parse_ticker(ticker: Ticker) -> Result<AssetInfo, ListeningError> {
    Ok(AssetInfo::new(
        ticker.inst_id,
        Decimal::from_str_exact(&ticker.last)?,
        str::parse::<i64>(&ticker.ts)? / 1000,
    ))
}

#[cfg(test)]
pub(crate) mod test {
    use tokio::sync::mpsc;
    use ws_mock::ws_mock_server::{WsMock, WsMockServer};

    use super::*;
    use crate::api::types::{ChannelArgument, PushData};

    pub(crate) async fn setup_mock_server() -> WsMockServer {
        WsMockServer::start().await
    }

    #[tokio::test]
    async fn test_recv_ticker() {
        // Set up the mock server and the WebSocket connector.
        let server = setup_mock_server().await;
        let connector = WebSocketConnector::new(server.uri().await);
        let (mpsc_send, mpsc_recv) = mpsc::channel::<Message>(32);

        // Create a mock ticker data.
        let mock_ticker = Ticker {
            inst_type: "SPOT".to_string(),
            inst_id: "BTC-USDT".to_string(),
            last: "10000".to_string(),
            last_sz: "5000".to_string(),
            ask_px: "10001".to_string(),
            ask_sz: "5000".to_string(),
            bid_px: "9999".to_string(),
            bid_sz: "5000".to_string(),
            open_24h: "10000".to_string(),
            high_24h: "10000".to_string(),
            low_24h: "10000".to_string(),
            vol_ccy_24h: "10000".to_string(),
            vol_24h: "10000".to_string(),
            sod_utc0: "10000".to_string(),
            sod_utc8: "10000".to_string(),
            ts: "10000".to_string(),
        };

        let mock_resp = Response::TickersChannel(PushData {
            arg: ChannelArgument {
                channel: "tickers".to_string(),
                inst_id: "BTC-USDT".to_string(),
            },
            data: vec![mock_ticker],
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
