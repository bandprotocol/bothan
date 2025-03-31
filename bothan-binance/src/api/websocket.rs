use std::str::FromStr;

use bothan_lib::types::AssetInfo;
use bothan_lib::worker::websocket::{AssetInfoProvider, AssetInfoProviderConnector, Data};
use futures_util::{SinkExt, StreamExt};
use rand::random;
use rust_decimal::Decimal;
use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite};

use crate::api::error::{Error, PollingError};
use crate::api::msgs::{Event, MiniTickerInfo, StreamEventData};

pub const DEFAULT_URL: &str = "wss://stream.binance.com:9443/stream";

/// A connector for establishing WebSocket connections to Binance.
pub struct WebSocketConnector {
    url: String,
}

impl WebSocketConnector {
    /// Creates a new `BinanceWebSocketConnector` with the given URL.
    ///
    /// # Examples
    ///
    /// ```no_test
    /// let connector = BinanceWebSocketConnector::new("wss://example.com/socket");
    /// ```
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    /// Establishes a WebSocket connection to the Binance server.
    ///
    /// # Examples
    ///
    /// ```no_test
    /// let connector = BinanceWebSocketConnector::new("wss://example.com/socket");
    /// let connection = connector.connect().await?;
    /// ```
    pub async fn connect(&self) -> Result<WebSocketConnection, tungstenite::Error> {
        // Attempt to establish a WebSocket connection.
        let (wss, _) = connect_async(self.url.clone()).await?;
        Ok(WebSocketConnection::new(wss))
    }
}

#[async_trait::async_trait]
impl AssetInfoProviderConnector for WebSocketConnector {
    type Provider = WebSocketConnection;
    type Error = tungstenite::Error;

    async fn connect(&self) -> Result<WebSocketConnection, Self::Error> {
        WebSocketConnector::connect(self).await
    }
}

/// Represents an active WebSocket connection to Binance.
pub struct WebSocketConnection {
    ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl WebSocketConnection {
    /// Creates a new `BinanceWebSocketConnection`
    pub fn new(ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        Self { ws_stream }
    }

    /// Subscribes to the mini ticker stream for the specified symbol IDs.
    ///
    /// # Examples
    ///
    /// ```no_test
    /// let mut connection = connector.connect().await?;
    /// connection.subscribe_mini_ticker_stream(&["btcusdt"]).await?;
    /// ```
    pub async fn subscribe_mini_ticker_stream<K: AsRef<str>>(
        &mut self,
        id: i64,
        tickers: &[K],
    ) -> Result<(), tungstenite::Error> {
        // Format the stream IDs for subscription.
        let tickers = tickers
            .iter()
            .map(|id| format!("{}@miniTicker", id.as_ref()))
            .collect::<Vec<_>>();

        // Create the subscription payload.
        let payload = json!({
            "method": "SUBSCRIBE",
            "params": tickers,
            "id": id,
        });

        // Send the subscription message.
        let message = Message::Text(payload.to_string());
        self.ws_stream.send(message).await?;
        Ok(())
    }

    /// Unsubscribes from the mini ticker stream for the specified symbol IDs.
    ///
    /// # Examples
    ///
    /// ```no_test
    /// let mut connection = connector.connect().await?;
    /// connection.unsubscribe_mini_ticker_stream(&["btcusdt"]).await?;
    /// ```
    pub async fn unsubscribe_mini_ticker_stream<K: AsRef<str>>(
        &mut self,
        id: i64,
        tickers: &[K],
    ) -> Result<(), tungstenite::Error> {
        // Format the stream IDs for unsubscription.
        let stream_ids = tickers
            .iter()
            .map(|id| format!("{}@miniTicker", id.as_ref()))
            .collect::<Vec<_>>();

        // Create the unsubscription payload.
        let payload = json!({
            "method": "UNSUBSCRIBE",
            "params": stream_ids,
            "id":id,
        });

        // Send the unsubscription message.
        let message = Message::Text(payload.to_string());
        self.ws_stream.send(message).await?;
        Ok(())
    }

    /// Retrieves the next message from the WebSocket stream.
    ///
    /// # Examples
    ///
    /// ```no_test
    /// let mut connection = connector.connect().await?;
    /// if let Ok(response) = connection.next().await {
    ///     println!("Received response: {:?}", response);
    /// }
    /// ```
    pub async fn next(&mut self) -> Option<Result<Event, Error>> {
        match self.ws_stream.next().await {
            Some(Ok(Message::Text(msg))) => match serde_json::from_str::<Event>(&msg) {
                Ok(msg) => Some(Ok(msg)),
                Err(e) => Some(Err(Error::ParseFailed(e))),
            },
            Some(Ok(Message::Ping(_))) => Some(Ok(Event::Ping)),
            Some(Ok(Message::Close(_))) => None,
            Some(Ok(_)) => Some(Err(Error::UnsupportedWebsocketMessageType)),
            Some(Err(_)) => None, // Consider the connection closed if error detected
            None => None,
        }
    }

    pub async fn close(&mut self) -> Result<(), tungstenite::Error> {
        self.ws_stream.close(None).await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl AssetInfoProvider for WebSocketConnection {
    type SubscriptionError = tungstenite::Error;
    type PollingError = PollingError;

    async fn subscribe(&mut self, ids: &[String]) -> Result<(), Self::SubscriptionError> {
        self.subscribe_mini_ticker_stream(random(), ids).await?;
        Ok(())
    }

    async fn next(&mut self) -> Option<Result<Data, Self::PollingError>> {
        WebSocketConnection::next(self).await.map(|r| {
            Ok(match r? {
                Event::Stream(se) => match se.data {
                    StreamEventData::MiniTicker(i) => parse_mini_ticker(i)?,
                },
                Event::Ping => Data::Ping,
                _ => Data::Unused,
            })
        })
    }

    async fn try_close(mut self) {
        tokio::spawn(async move { self.close().await });
    }
}

fn parse_mini_ticker(mini_ticker: MiniTickerInfo) -> Result<Data, rust_decimal::Error> {
    let asset_info = AssetInfo::new(
        mini_ticker.symbol.to_ascii_lowercase(),
        Decimal::from_str(&mini_ticker.close_price)?,
        mini_ticker.event_time,
    );
    Ok(Data::AssetInfo(vec![asset_info]))
}

#[cfg(test)]
pub(crate) mod test {
    use tokio::sync::mpsc;
    use ws_mock::ws_mock_server::{WsMock, WsMockServer};

    use super::*;
    use crate::api::msgs::{Event, MiniTickerInfo, StreamEvent};

    pub(crate) async fn setup_mock_server() -> WsMockServer {
        WsMockServer::start().await
    }

    #[tokio::test]
    async fn test_recv_ticker() {
        // Set up the mock server and the WebSocket connector.
        let server = setup_mock_server().await;
        let connector = WebSocketConnector::new(server.uri().await);
        let (mpsc_send, mpsc_recv) = mpsc::channel::<Message>(32);

        // Create a mock mini ticker response.
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
        let mock_resp = StreamEvent {
            stream: "btc@miniTicker".to_string(),
            data: StreamEventData::MiniTicker(mock_ticker),
        };

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
        assert_eq!(resp, Event::Stream(mock_resp));
    }

    #[tokio::test]
    async fn test_recv_ping() {
        // Set up the mock server and the WebSocket connector.
        let server = setup_mock_server().await;
        let connector = WebSocketConnector::new(server.uri().await);
        let (mpsc_send, mpsc_recv) = mpsc::channel::<Message>(32);

        // Mount the mock WebSocket server and send a ping message.
        WsMock::new()
            .forward_from_channel(mpsc_recv)
            .mount(&server)
            .await;
        mpsc_send.send(Message::Ping(vec![])).await.unwrap();

        // Connect to the mock WebSocket server and retrieve the ping response.
        let mut connection = connector.connect().await.unwrap();
        let resp = connection.next().await.unwrap().unwrap();
        assert_eq!(resp, Event::Ping);
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
