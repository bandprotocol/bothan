use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::error::Error as TungsteniteError;
use tokio_tungstenite::tungstenite::http::StatusCode;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tracing::warn;

use crate::api::error::Error;
use crate::api::types::channel::ticker::{EventTrigger, TickerRequestParameters};
use crate::api::types::message::{Method, PublicMessage};
use crate::api::types::KrakenResponse;

pub struct KrakenWebSocketConnector {
    url: String,
}

impl KrakenWebSocketConnector {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    pub async fn connect(&self) -> Result<KrakenWebSocketConnection, Error> {
        let (wss, resp) = connect_async(self.url.clone()).await?;

        let status = resp.status();
        if StatusCode::is_server_error(&status) || StatusCode::is_client_error(&status) {
            warn!("Failed to connect with response code {}", resp.status());
            return Err(Error::ConnectionFailure(resp.status()));
        }

        Ok(KrakenWebSocketConnection::new(wss))
    }
}

pub struct KrakenWebSocketConnection {
    sender: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    receiver: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl KrakenWebSocketConnection {
    pub fn new(web_socket_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        let (sender, receiver) = web_socket_stream.split();
        Self { sender, receiver }
    }

    pub async fn subscribe_ticker(
        &mut self,
        symbols: &[&str],
        event_trigger: Option<EventTrigger>,
        snapshot: Option<bool>,
    ) -> Result<(), Error> {
        let ticker_param = build_ticker_request(symbols, event_trigger, snapshot);
        let msg = PublicMessage {
            method: Method::Subscribe,
            params: Some(ticker_param),
            req_id: None,
        };
        let message = Message::Text(serde_json::to_string(&msg)?);
        Ok(self.sender.send(message).await?)
    }

    pub async fn unsubscribe_ticker(&mut self, symbols: &[&str]) -> Result<(), Error> {
        let params = build_ticker_request(symbols, None, None);
        let msg = PublicMessage {
            method: Method::Unsubscribe,
            params: Some(params),
            req_id: None,
        };
        let message = Message::Text(serde_json::to_string(&msg)?);
        Ok(self.sender.send(message).await?)
    }

    pub async fn next(&mut self) -> Result<KrakenResponse, Error> {
        if let Some(result_msg) = self.receiver.next().await {
            return match result_msg {
                Ok(Message::Text(msg)) => serde_json::from_str::<KrakenResponse>(&msg)
                    .map_err(|_| Error::UnsupportedMessage),
                Ok(Message::Close(_)) => Err(Error::ChannelClosed),
                Err(err) => match err {
                    TungsteniteError::Protocol(..) => Err(Error::ChannelClosed),
                    TungsteniteError::ConnectionClosed => Err(Error::ChannelClosed),
                    _ => Err(Error::UnsupportedMessage),
                },
                _ => Err(Error::UnsupportedMessage),
            };
        }

        Err(Error::ChannelClosed)
    }
}

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
