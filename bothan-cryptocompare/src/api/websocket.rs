use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, tungstenite, MaybeTlsStream, WebSocketStream};
use url::Url;

use crate::api::errors::{ConnectionError, MessageError};
use crate::api::msgs::Packet;

/// The default URL for the CryptoCompare API.
pub const DEFAULT_URL: &str = "wss://data-streamer.cryptocompare.com";

pub struct CryptoCompareWebSocketConnector {
    url: String,
    api_key: String,
}

impl CryptoCompareWebSocketConnector {
    pub fn new(url: String, api_key: String) -> Self {
        Self { url, api_key }
    }

    pub async fn connect(&self) -> Result<CryptoCompareWebSocketConnection, ConnectionError> {
        let url = Url::parse_with_params(&self.url, &[("api_key", &self.api_key)])?;
        let (wss, resp) = connect_async(url.as_str()).await?;

        let status = resp.status();
        if status.as_u16() >= 400 {
            return Err(ConnectionError::UnsuccessfulWebSocketResponse(status));
        }

        Ok(CryptoCompareWebSocketConnection::new(wss))
    }
}

pub struct CryptoCompareWebSocketConnection {
    ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl CryptoCompareWebSocketConnection {
    pub fn new(ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        Self { ws_stream }
    }

    pub async fn subscribe_latest_tick_adaptive_inclusion<T: AsRef<str>>(
        &mut self,
        instruments: &[T],
    ) -> Result<(), tungstenite::Error> {
        let payload = json!({
            "action": "SUBSCRIBE",
            "type": "index_cc_v1_latest_tick",
            "groups": ["VALUE"],
            "market": "cadli",
            "instruments": instruments.iter().map(|s| s.as_ref()).collect::<Vec<&str>>()
        });

        let message = Message::Text(payload.to_string());
        self.ws_stream.send(message).await?;
        Ok(())
    }

    pub async fn unsubscribe_latest_tick_adaptive_inclusion<T: AsRef<str>>(
        &mut self,
        instruments: &[T],
    ) -> Result<(), tungstenite::Error> {
        let payload = json!({
            "action": "UNSUBSCRIBE",
            "type": "index_cc_v1_latest_tick",
            "groups": ["VALUE"],
            "market": "cadli",
            "instruments": instruments.iter().map(|s| s.as_ref()).collect::<Vec<&str>>()
        });

        let message = Message::Text(payload.to_string());
        self.ws_stream.send(message).await?;
        Ok(())
    }

    pub async fn next(&mut self) -> Result<Packet, MessageError> {
        if let Some(Ok(result_msg)) = self.ws_stream.next().await {
            match result_msg {
                Message::Text(msg) => Ok(serde_json::from_str::<Packet>(&msg)?),
                Message::Close(_) => Err(MessageError::ChannelClosed),
                // We skip deserializing ping as it's the only packet that we get from this message type
                Message::Ping(_) => Ok(Packet::Ping),
                _ => Err(MessageError::UnsupportedMessage),
            }
        } else {
            Err(MessageError::ChannelClosed)
        }
    }

    pub async fn close(&mut self) -> Result<(), tungstenite::Error> {
        self.ws_stream.close(None).await?;
        Ok(())
    }
}
