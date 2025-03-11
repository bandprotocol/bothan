use bothan_lib::types::AssetInfo;
use bothan_lib::worker::websocket::{AssetInfoProvider, AssetInfoProviderConnector, Data};
use futures_util::{SinkExt, StreamExt};
use rust_decimal::Decimal;
use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite};

use crate::api::error::{Error, PollingError};
use crate::api::types::ticker::{InstrumentType, Ticker};
use crate::api::types::{Response, subscription, ticker};

/// A connector for establishing a WebSocket connection to the OKX API.
pub struct WebSocketConnector {
    url: String,
}

impl WebSocketConnector {
    /// Creates a new instance of `OkxWebSocketConnector`.
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    /// Connects to the OKX WebSocket API.
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

/// Represents an active WebSocket connection to the OKX API.
pub struct WebSocketConnection {
    ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl WebSocketConnection {
    /// Creates a new `OkxWebSocketConnection` instance.
    pub fn new(ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        Self { ws_stream }
    }

    /// Subscribes to ticker updates for the given instrument IDs.
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
    pub async fn unsubscribe_tickerr<T: ToString>(
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

    /// Receives the next message from the WebSocket.
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

/// Builds a ticker request with the given parameters.
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

fn parse_msg(msg: String) -> Result<Response, Error> {
    Ok(serde_json::from_str::<Response>(&msg)?)
}

#[async_trait::async_trait]
impl AssetInfoProvider for WebSocketConnection {
    type SubscriptionError = tungstenite::Error;
    type PollingError = PollingError;

    async fn subscribe(&mut self, ids: &[String]) -> Result<(), Self::SubscriptionError> {
        self.subscribe_ticker(ids).await?;
        Ok(())
    }

    async fn next(&mut self) -> Option<Result<Data, Self::PollingError>> {
        WebSocketConnection::next(self).await.map(|r| match r? {
            Response::TickersChannel(data) => parse_tickers(data.data),
            _ => Ok(Data::Unused),
        })
    }

    async fn try_close(mut self) {
        tokio::spawn(async move { self.close().await });
    }
}

fn parse_tickers(tickers: Vec<Ticker>) -> Result<Data, PollingError> {
    Ok(Data::AssetInfo(
        tickers
            .into_iter()
            .map(parse_ticker)
            .collect::<Result<Vec<AssetInfo>, PollingError>>()?,
    ))
}

fn parse_ticker(ticker: Ticker) -> Result<AssetInfo, PollingError> {
    Ok(AssetInfo::new(
        ticker.inst_id,
        Decimal::from_str_exact(&ticker.last)?,
        str::parse::<i64>(&ticker.ts)?,
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
        let resp = connection.next().await.unwrap();
        assert!(resp.is_err());
    }
}
