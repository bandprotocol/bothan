use std::io::Read;

use bothan_lib::types::AssetInfo;
use bothan_lib::worker::websocket::{AssetInfoProvider, AssetInfoProviderConnector, Data};
use flate2::read::GzDecoder;
use futures_util::{SinkExt, StreamExt};
use rust_decimal::Decimal;
use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite};
use tracing::warn;

use crate::api::error::{Error, ListeningError};
use crate::api::types::Response;

/// A connector for establishing a WebSocket connection to the Htx API.
pub struct WebSocketConnector {
    url: String,
}

impl WebSocketConnector {
    /// Creates a new instance of `HtxWebSocketConnector`.
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    /// Connects to the Htx WebSocket API.
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

/// Represents an active WebSocket connection to the Htx API.
pub struct WebSocketConnection {
    ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl WebSocketConnection {
    /// Creates a new `HtxWebSocketConnection` instance.
    pub fn new(ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        Self { ws_stream }
    }

    /// Subscribes to ticker updates for a single symbol.
    pub async fn subscribe_ticker(&mut self, symbol: &str) -> Result<(), tungstenite::Error> {
        let formatted_symbol = format!("market.{}.ticker", symbol);
        let payload = json!({
            "sub": formatted_symbol,
        });

        // Send the subscription message.
        let message = Message::Text(payload.to_string());
        self.ws_stream.send(message).await?;
        Ok(())
    }

    /// Unsubscribes from ticker updates for a single symbol.
    pub async fn unsubscribe_ticker(&mut self, symbol: &str) -> Result<(), tungstenite::Error> {
        let formatted_symbol = format!("market.{}.ticker", symbol);
        let payload = json!({
            "unsub": formatted_symbol,
        });

        // Send the unsubscription message.
        let message = Message::Text(payload.to_string());
        self.ws_stream.send(message).await?;
        Ok(())
    }

    /// Sends a Pong message in response to a Ping message.
    pub async fn send_pong(&mut self, pong: u64) -> Result<(), tungstenite::Error> {
        let payload = json!({
            "pong": pong,
        });

        // Send the pong message.
        let message = Message::Text(payload.to_string());
        self.ws_stream.send(message).await?;
        Ok(())
    }

    /// Receives the next message from the WebSocket connection.
    pub async fn next(&mut self) -> Option<Result<Response, Error>> {
        match self.ws_stream.next().await {
            Some(Ok(Message::Binary(msg))) => Some(decode_response(&msg)),
            Some(Ok(Message::Ping(_))) => None,
            Some(Ok(Message::Close(_))) => None,
            Some(Ok(_)) => Some(Err(Error::UnsupportedWebsocketMessageType)),
            Some(Err(_)) => None, // Consider the connection closed if error detected
            None => None,
        }
    }

    /// Closes the WebSocket connection.
    pub async fn close(&mut self) -> Result<(), tungstenite::Error> {
        self.ws_stream.close(None).await?;
        Ok(())
    }
}

fn decode_response(msg: &[u8]) -> Result<Response, Error> {
    let mut decoder = GzDecoder::new(msg);
    let mut decompressed_msg = String::new();
    decoder.read_to_string(&mut decompressed_msg)?;
    Ok(serde_json::from_str::<Response>(&decompressed_msg)?)
}

#[async_trait::async_trait]
impl AssetInfoProvider for WebSocketConnection {
    type SubscriptionError = tungstenite::Error;
    type ListeningError = ListeningError;

    async fn subscribe(&mut self, ids: &[String]) -> Result<(), Self::SubscriptionError> {
        for id in ids {
            self.subscribe_ticker(id).await?;
        }

        Ok(())
    }

    async fn next(&mut self) -> Option<Result<Data, Self::ListeningError>> {
        let msg = WebSocketConnection::next(self).await?;
        Some(match msg {
            Ok(Response::DataUpdate(d)) => parse_data(d),
            Ok(Response::Ping(p)) => reply_pong(self, p.ping).await,
            Ok(Response::Error(e)) => {
                warn!("received error in response: {:?}", e);
                Ok(Data::Unused)
            }
            Err(e) => Err(ListeningError::Error(e)),
            _ => Ok(Data::Unused),
        })
    }

    async fn try_close(mut self) {
        tokio::spawn(async move { self.close().await });
    }
}

fn parse_data(data: super::types::Data) -> Result<Data, ListeningError> {
    let id = data
        .ch
        .split('.')
        .nth(1)
        .ok_or(ListeningError::InvalidChannelId)?
        .to_string();
    let asset_info = AssetInfo::new(
        id,
        Decimal::from_f64_retain(data.tick.last_price).ok_or(ListeningError::InvalidPrice)?,
        data.timestamp / 1000, // convert from millisecond to second
    );
    Ok(Data::AssetInfo(vec![asset_info]))
}

async fn reply_pong(
    connection: &mut WebSocketConnection,
    ping: u64,
) -> Result<Data, ListeningError> {
    connection.send_pong(ping).await?;
    Ok(Data::Ping)
}

#[cfg(test)]
pub(crate) mod test {
    use std::io::Write;

    use flate2::Compression;
    use flate2::write::GzEncoder;
    use tokio::sync::mpsc;
    use ws_mock::ws_mock_server::{WsMock, WsMockServer};

    use super::*;
    use crate::api::types::{Data, Ping, Response, Subscribed, Tick, Unsubscribed};

    pub(crate) async fn setup_mock_server() -> WsMockServer {
        WsMockServer::start().await
    }

    #[tokio::test]
    async fn test_recv_ping() {
        // Set up the mock server and the WebSocket connector.
        let server = setup_mock_server().await;
        let connector = WebSocketConnector::new(server.uri().await);
        let (mpsc_send, mpsc_recv) = mpsc::channel::<Message>(32);

        // Create a mock ping response.
        let mock_ping = Ping {
            ping: 1492420473027,
        };
        let mock_resp = Response::Ping(mock_ping);

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        let _ = encoder
            .write(serde_json::to_string(&mock_resp).unwrap().as_bytes())
            .unwrap();
        let encoded = encoder.finish().unwrap();

        // Mount the mock WebSocket server and send the mock response.
        WsMock::new()
            .forward_from_channel(mpsc_recv)
            .mount(&server)
            .await;
        mpsc_send.send(Message::Binary(encoded)).await.unwrap();

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

    #[tokio::test]
    async fn test_recv_sub_response() {
        // Set up the mock server and the WebSocket connector.
        let server = setup_mock_server().await;
        let connector = WebSocketConnector::new(server.uri().await);
        let (mpsc_send, mpsc_recv) = mpsc::channel::<Message>(32);

        // Create a mock subscribe response.
        let mock_sub_resp = Subscribed {
            id: Some("id1".to_string()),
            status: "ok".to_string(),
            subbed: "market.btcusdt.kline.1min".to_string(),
            timestamp: 1489474081631,
        };
        let mock_resp = Response::Subscribed(mock_sub_resp);

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        let _ = encoder
            .write(serde_json::to_string(&mock_resp).unwrap().as_bytes())
            .unwrap();
        let encoded = encoder.finish().unwrap();

        // Mount the mock WebSocket server and send the mock response.
        WsMock::new()
            .forward_from_channel(mpsc_recv)
            .mount(&server)
            .await;
        mpsc_send.send(Message::Binary(encoded)).await.unwrap();

        // Connect to the mock WebSocket server and retrieve the response.
        let mut connection = connector.connect().await.unwrap();
        let resp = connection.next().await.unwrap().unwrap();
        assert_eq!(resp, mock_resp);
    }

    #[tokio::test]
    async fn test_recv_unsub_response() {
        // Set up the mock server and the WebSocket connector.
        let server = setup_mock_server().await;
        let connector = WebSocketConnector::new(server.uri().await);
        let (mpsc_send, mpsc_recv) = mpsc::channel::<Message>(32);

        // Create a mock unsubscribe response.
        let mock_unsub_resp = Unsubscribed {
            id: Some("id4".to_string()),
            status: "ok".to_string(),
            unsubbed: "market.btcusdt.trade.detail".to_string(),
            timestamp: 1494326028889,
        };
        let mock_resp = Response::Unsubscribed(mock_unsub_resp);

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        let _ = encoder
            .write(serde_json::to_string(&mock_resp).unwrap().as_bytes())
            .unwrap();
        let encoded = encoder.finish().unwrap();

        // Mount the mock WebSocket server and send the mock response.
        WsMock::new()
            .forward_from_channel(mpsc_recv)
            .mount(&server)
            .await;
        mpsc_send.send(Message::Binary(encoded)).await.unwrap();

        // Connect to the mock WebSocket server and retrieve the response.
        let mut connection = connector.connect().await.unwrap();
        let resp = connection.next().await.unwrap().unwrap();
        assert_eq!(resp, mock_resp);
    }

    #[tokio::test]
    async fn test_recv_data_update() {
        // Set up the mock server and the WebSocket connector.
        let server = setup_mock_server().await;
        let connector = WebSocketConnector::new(server.uri().await);
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
        let mock_data_update = Data {
            ch: "market.btcusdt.ticker".to_string(),
            timestamp: 1630982370526,
            tick: mock_tick,
        };

        let mock_resp = Response::DataUpdate(mock_data_update);

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        let _ = encoder
            .write(serde_json::to_string(&mock_resp).unwrap().as_bytes())
            .unwrap();
        let encoded = encoder.finish().unwrap();

        // Mount the mock WebSocket server and send the mock response.
        WsMock::new()
            .forward_from_channel(mpsc_recv)
            .mount(&server)
            .await;
        mpsc_send.send(Message::Binary(encoded)).await.unwrap();

        // Connect to the mock WebSocket server and retrieve the response.
        let mut connection = connector.connect().await.unwrap();
        let resp = connection.next().await.unwrap().unwrap();
        assert_eq!(resp, mock_resp);
    }
}
