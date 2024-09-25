use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::error::Error as TungsteniteError;
use tokio_tungstenite::tungstenite::http::StatusCode;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tracing::warn;

use crate::api::error::{ConnectionError, MessageError, SendError};
use crate::api::types::message::{InstrumentType, Op, PriceRequestArgument, WebSocketMessage};
use crate::api::types::OkxResponse;

/// A connector for establishing a WebSocket connection to the OKX API.
pub struct OkxWebSocketConnector {
    url: String,
}

impl OkxWebSocketConnector {
    /// Creates a new instance of `OkxWebSocketConnector`.
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    /// Connects to the OKX WebSocket API.
    pub async fn connect(&self) -> Result<OkxWebSocketConnection, ConnectionError> {
        let (wss, resp) = connect_async(self.url.clone()).await?;

        let status = resp.status();
        if StatusCode::is_server_error(&status) || StatusCode::is_client_error(&status) {
            warn!("failed to connect with response code {}", resp.status());
            return Err(ConnectionError::UnsuccessfulWebSocketResponse(
                resp.status(),
            ));
        }

        Ok(OkxWebSocketConnection::new(wss))
    }
}

/// Represents an active WebSocket connection to the OKX API.
pub struct OkxWebSocketConnection {
    sender: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    receiver: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl OkxWebSocketConnection {
    /// Creates a new `OkxWebSocketConnection` instance.
    pub fn new(web_socket_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        let (sender, receiver) = web_socket_stream.split();
        Self { sender, receiver }
    }

    /// Subscribes to ticker updates for the given instrument IDs.
    pub async fn subscribe_ticker(&mut self, inst_ids: &[&str]) -> Result<(), SendError> {
        let ticker_args = build_ticker_arguments(inst_ids);
        let msg = WebSocketMessage {
            op: Op::Subscribe,
            args: Some(ticker_args),
        };

        // Send the subscription message.
        let message = Message::Text(serde_json::to_string(&msg)?);
        Ok(self.sender.send(message).await?)
    }

    /// Unsubscribes from ticker updates for the given instrument IDs.
    pub async fn unsubscribe_ticker(&mut self, inst_ids: &[&str]) -> Result<(), SendError> {
        let ticker_args = build_ticker_arguments(inst_ids);
        let msg = WebSocketMessage {
            op: Op::Unsubscribe,
            args: Some(ticker_args),
        };

        // Send the unsubscription message.
        let message = Message::Text(serde_json::to_string(&msg)?);
        Ok(self.sender.send(message).await?)
    }

    /// Receives the next message from the WebSocket connection.
    pub async fn next(&mut self) -> Result<OkxResponse, MessageError> {
        if let Some(result_msg) = self.receiver.next().await {
            return match result_msg {
                Ok(Message::Text(msg)) => serde_json::from_str::<OkxResponse>(&msg)
                    .map_err(|_| MessageError::UnsupportedMessage),
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
fn build_ticker_arguments(inst_ids: &[&str]) -> Vec<PriceRequestArgument> {
    let inst_ids = inst_ids
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    inst_ids
        .iter()
        .map(|id| PriceRequestArgument {
            channel: "tickers".to_string(),
            inst_type: Some(InstrumentType::Spot),
            inst_family: None,
            inst_id: Some(id.clone()),
        })
        .collect()
}

#[cfg(test)]
pub(crate) mod test {
    use tokio::sync::mpsc;
    use ws_mock::ws_mock_server::{WsMock, WsMockServer};

    use crate::api::types::{ChannelArgument, ChannelResponse, OkxResponse, PushData, TickerData};

    use super::*;

    pub(crate) async fn setup_mock_server() -> WsMockServer {
        WsMockServer::start().await
    }

    #[tokio::test]
    async fn test_recv_ticker() {
        // Set up the mock server and the WebSocket connector.
        let server = setup_mock_server().await;
        let connector = OkxWebSocketConnector::new(server.uri().await);
        let (mpsc_send, mpsc_recv) = mpsc::channel::<Message>(32);

        // Create a mock ticker data.
        let mock_ticker = TickerData {
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
        let mock_resp = OkxResponse::ChannelResponse(ChannelResponse::Ticker(PushData {
            arg: ChannelArgument {
                channel: "tickers".to_string(),
                inst_id: "BTC-USDT".to_string(),
            },
            data: vec![mock_ticker],
        }));

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
        let connector = OkxWebSocketConnector::new(server.uri().await);
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
