use bothan_lib::types::AssetInfo;
use bothan_lib::worker::websocket::{AssetInfoProvider, AssetInfoProviderConnector, Data};
use futures_util::{SinkExt, StreamExt};
use rust_decimal::Decimal;
use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite};

use crate::api::error::{Error, ListeningError};
use crate::api::types::{MAX_ARGS, PublicTickerResponse, Response};

/// A connector for establishing a WebSocket connection to the Bybit API.
pub struct WebSocketConnector {
    url: String,
}

impl WebSocketConnector {
    /// Creates a new instance of `BybitWebSocketConnector`.
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    /// Connects to the Bybit WebSocket API.
    pub async fn connect(&self) -> Result<WebSocketConnection, tungstenite::Error> {
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

/// Represents an active WebSocket connection to the Bybit API.
pub struct WebSocketConnection {
    ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl WebSocketConnection {
    /// Creates a new `BybitWebSocketConnection` instance.
    pub fn new(ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        Self { ws_stream }
    }

    /// Subscribes to ticker updates for the given symbols.
    pub async fn subscribe_ticker<T: AsRef<str>>(
        &mut self,
        symbols: &[T],
    ) -> Result<(), tungstenite::Error> {
        let payload = json!({
            "op": "subscribe",
            "args": symbols.iter().map(|s| format!("tickers.{}", s.as_ref())).collect::<Vec<String>>(),
        });

        // Send the subscription message.
        let message = Message::Text(payload.to_string());
        self.ws_stream.send(message).await
    }

    /// Unsubscribes to ticker updates for the given symbols.
    pub async fn unsubscribe_ticker<T: AsRef<str>>(
        &mut self,
        symbols: &[T],
    ) -> Result<(), tungstenite::Error> {
        let payload = json!({
            "op": "unsubscribe",
            "args": symbols.iter().map(|s| format!("tickers.{}", s.as_ref())).collect::<Vec<String>>(),
        });

        // Send the unsubscription message.
        let message = Message::Text(payload.to_string());
        self.ws_stream.send(message).await
    }

    /// Receives the next message from the WebSocket connection.
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

    pub async fn close(&mut self) -> Result<(), tungstenite::Error> {
        self.ws_stream.close(None).await?;
        Ok(())
    }
}

fn parse_msg(msg: String) -> Result<Response, Error> {
    Ok(serde_json::from_str::<Response>(&msg)?)
}

#[async_trait::async_trait]
impl AssetInfoProvider for WebSocketConnection {
    type SubscriptionError = tungstenite::Error;
    type ListeningError = ListeningError;

    async fn subscribe(&mut self, ids: &[String]) -> Result<(), Self::SubscriptionError> {
        for chunk in ids.chunks(MAX_ARGS) {
            self.subscribe_ticker(chunk).await?;
        }
        Ok(())
    }

    async fn next(&mut self) -> Option<Result<Data, Self::ListeningError>> {
        WebSocketConnection::next(self).await.map(|r| {
            Ok(match r? {
                Response::PublicTicker(t) => parse_public_ticker(t)?,
                _ => Data::Unused,
            })
        })
    }

    async fn try_close(mut self) {
        tokio::spawn(async move { self.close().await });
    }
}

fn parse_public_ticker(ticker: PublicTickerResponse) -> Result<Data, rust_decimal::Error> {
    let asset_info = AssetInfo::new(
        ticker.data.symbol,
        Decimal::from_str_exact(&ticker.data.last_price)?,
        ticker.ts,
    );
    Ok(Data::AssetInfo(vec![asset_info]))
}

#[cfg(test)]
pub(crate) mod test {
    use tokio::sync::mpsc;
    use ws_mock::ws_mock_server::{WsMock, WsMockServer};

    use super::*;
    use crate::api::types::{PublicMessageResponse, PublicTickerResponse, Response, Ticker};

    pub(crate) async fn setup_mock_server() -> WsMockServer {
        WsMockServer::start().await
    }

    #[tokio::test]
    async fn test_recv_public_ticker() {
        // Set up the mock server and the WebSocket connector.
        let server = setup_mock_server().await;
        let connector = WebSocketConnector::new(server.uri().await);
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
        let mock_resp = Response::PublicTicker(PublicTickerResponse {
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
        let resp = connection.next().await.unwrap().unwrap();

        // Assert that the received response matches the mock response.
        assert_eq!(resp, mock_resp);
    }

    /// Test for receiving a public message response from the WebSocket.
    #[tokio::test]
    async fn test_recv_public_message() {
        // Set up the mock server and the WebSocket connector.
        let server = setup_mock_server().await;
        let connector = WebSocketConnector::new(server.uri().await);
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
        let mock_resp = Response::PublicMessage(mock_message);

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

        // Assert that the received response matches the mock response.
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
