use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, tungstenite, MaybeTlsStream, WebSocketStream};

use crate::api::{types::BinanceResponse, ConnectionError, Error};

/// A connector for establishing WebSocket connections to Binance.
pub struct BinanceWebSocketConnector {
    url: String,
}

impl BinanceWebSocketConnector {
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
    pub async fn connect(&self) -> Result<BinanceWebSocketConnection, ConnectionError> {
        // Attempt to establish a WebSocket connection.
        let (wss, resp) = connect_async(self.url.clone()).await?;

        // Check the HTTP response status.
        let status = resp.status();
        if status.as_u16() >= 400 {
            return Err(ConnectionError::UnsuccessfulHttpResponse { status });
        }

        // Return the WebSocket connection.
        Ok(BinanceWebSocketConnection::new(wss))
    }
}

/// Represents an active WebSocket connection to Binance.
pub struct BinanceWebSocketConnection {
    sender: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    receiver: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl BinanceWebSocketConnection {
    /// Creates a new `BinanceWebSocketConnection` by splitting the WebSocket stream into sender and receiver.
    pub fn new(web_socket_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        let (sender, receiver) = web_socket_stream.split();
        Self { sender, receiver }
    }

    /// Subscribes to the mini ticker stream for the specified symbol IDs.
    ///
    /// # Examples
    ///
    /// ```no_test
    /// let mut connection = connector.connect().await?;
    /// connection.subscribe_mini_ticker_stream(&["btcusdt"]).await?;
    /// ```
    pub async fn subscribe_mini_ticker_stream(
        &mut self,
        ids: &[&str],
    ) -> Result<(), tungstenite::Error> {
        // Format the stream IDs for subscription.
        let stream_ids = ids
            .iter()
            .map(|id| format!("{}@miniTicker", id))
            .collect::<Vec<_>>();

        // Create the subscription payload.
        let payload = json!({
            "method": "SUBSCRIBE",
            "params": stream_ids,
            "id": rand::random::<u32>()
        });

        // Send the subscription message.
        let message = Message::Text(payload.to_string());
        self.sender.send(message).await
    }

    /// Unsubscribes from the mini ticker stream for the specified symbol IDs.
    ///
    /// # Examples
    ///
    /// ```no_test
    /// let mut connection = connector.connect().await?;
    /// connection.unsubscribe_mini_ticker_stream(&["btcusdt"]).await?;
    /// ```
    pub async fn unsubscribe_mini_ticker_stream(
        &mut self,
        ids: &[&str],
    ) -> Result<(), tungstenite::Error> {
        // Format the stream IDs for unsubscription.
        let stream_ids = ids
            .iter()
            .map(|id| format!("{}@miniTicker", id))
            .collect::<Vec<_>>();

        // Create the unsubscription payload.
        let payload = json!({
            "method": "UNSUBSCRIBE",
            "params": stream_ids,
            "id": rand::random::<u32>()
        });

        // Send the unsubscription message.
        let message = Message::Text(payload.to_string());
        self.sender.send(message).await
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
    pub async fn next(&mut self) -> Result<BinanceResponse, Error> {
        // Wait for the next message.
        if let Some(result_msg) = self.receiver.next().await {
            // Handle the received message.
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
    use tokio::sync::mpsc;
    use ws_mock::ws_mock_server::{WsMock, WsMockServer};

    use crate::api::types::{Data, MiniTickerInfo, StreamResponse};

    use super::*;

    pub(crate) async fn setup_mock_server() -> WsMockServer {
        WsMockServer::start().await
    }

    #[tokio::test]
    async fn test_recv_ticker() {
        // Se up the mock server and the WebSocket connector.
        let server = setup_mock_server().await;
        let connector = BinanceWebSocketConnector::new(server.uri().await);
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
        let mock_resp = StreamResponse {
            stream: "btc@miniTicker".to_string(),
            data: Data::MiniTicker(mock_ticker),
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
        let resp = connection.next().await.unwrap();
        assert_eq!(resp, BinanceResponse::Stream(mock_resp));
    }

    #[tokio::test]
    async fn test_recv_ping() {
        // Set up the mock server and the WebSocket connector.
        let server = setup_mock_server().await;
        let connector = BinanceWebSocketConnector::new(server.uri().await);
        let (mpsc_send, mpsc_recv) = mpsc::channel::<Message>(32);

        // Mount the mock WebSocket server and send a ping message.
        WsMock::new()
            .forward_from_channel(mpsc_recv)
            .mount(&server)
            .await;
        mpsc_send.send(Message::Ping(vec![])).await.unwrap();

        // Connect to the mock WebSocket server and retrieve the ping response.
        let mut connection = connector.connect().await.unwrap();
        let resp = connection.next().await.unwrap();
        assert_eq!(resp, BinanceResponse::Ping);
    }

    #[tokio::test]
    async fn test_recv_close() {
        // Set up the mock server and the WebSocket connector.
        let server = setup_mock_server().await;
        let connector = BinanceWebSocketConnector::new(server.uri().await);
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
