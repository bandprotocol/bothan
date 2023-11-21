use futures_util::{stream::FusedStream, SinkExt, Stream, StreamExt};
use reqwest::StatusCode;
use serde_json::json;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::{binance_websocket::types::RequestMethod, error::Error, types::PriceInfo};

use super::types::{MiniTickerInfo, MiniTickerResponse, SettingResponse, WebsocketMessage};

/// A binance websocket object.
pub struct BinanceWebsocket {
    url: String,
    socket: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ended: bool,
}

fn parse_message(resp: String) -> Result<WebsocketMessage, Error> {
    let setting_response = serde_json::from_str::<SettingResponse>(&resp);
    if let Ok(response) = setting_response {
        return Ok(WebsocketMessage::SettingResponse(response));
    }

    let mini_ticker_response = serde_json::from_str::<MiniTickerResponse>(&resp);
    if let Ok(response) = mini_ticker_response {
        let MiniTickerInfo {
            id,
            current_price,
            timestamp,
        } = response.data;

        let price = current_price.parse::<f64>()?;
        return Ok(WebsocketMessage::PriceInfo(PriceInfo {
            id,
            price,
            timestamp,
        }));
    }

    Err(Error::ParsingError("response".into(), "not support".into()))
}

impl BinanceWebsocket {
    pub fn new(url: &str) -> Self {
        let ws_url = format!("{}/stream", url);

        Self {
            url: ws_url,
            socket: None,
            ended: false,
        }
    }

    pub async fn subscribe(&mut self, ids: &[&str]) -> Result<(), Error> {
        let socket = self.socket.as_mut().ok_or(Error::NotConnected)?;
        let (mut write, _) = socket.split();

        let stream_ids = ids
            .iter()
            .map(|id| format!("{}@miniTicker", id))
            .collect::<Vec<_>>();

        let message = Message::Text(
            json!({
                "method": RequestMethod::Subscribe.to_string(),
                "params": stream_ids,
                "id": 1
            })
            .to_string(),
        );

        write.send(message).await?;

        Ok(())
    }

    pub async fn unsubscribe(&mut self, ids: &[&str]) -> Result<(), Error> {
        let socket = self.socket.as_mut().ok_or(Error::NotConnected)?;
        let (mut write, _) = socket.split();

        let stream_ids = ids
            .iter()
            .map(|id| format!("{}@miniTicker", id))
            .collect::<Vec<_>>();

        let message = Message::Text(
            json!({
                "method": RequestMethod::Unsubscribe.to_string(),
                "params": stream_ids,
                "id": 1
            })
            .to_string(),
        );

        write.send(message).await?;

        Ok(())
    }

    pub async fn connect(&mut self) -> Result<(), Error> {
        let (socket, response) = connect_async(&self.url).await?;

        let response_status = response.status();
        if StatusCode::is_success(&response_status) {
            tracing::trace!("query request get error status {}", response_status);
            return Err(Error::ResponseStatusNotOk(response_status));
        }

        self.socket = Some(socket);

        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.socket.is_some()
    }
}

impl Stream for BinanceWebsocket {
    type Item = Result<WebsocketMessage, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.ended {
            return Poll::Ready(None);
        }

        let Some(socket) = self.socket.as_mut() else {
            return Poll::Ready(None);
        };

        match socket.poll_next_unpin(cx) {
            Poll::Ready(Some(message)) => match message {
                Ok(Message::Text(text)) => {
                    tracing::info!("received text message: {}", text);
                    let response = match parse_message(text) {
                        Ok(info) => info,
                        Err(err) => {
                            tracing::trace!("cannot convert received text to PriceInfo: {}", err);
                            return Poll::Ready(Some(Err(err)));
                        }
                    };

                    Poll::Ready(Some(Ok(response)))
                }
                Ok(_) => {
                    tracing::trace!("received non-text message");
                    Poll::Pending
                }
                Err(err) => Poll::Ready(Some(Err(err.into()))),
            },
            Poll::Ready(None) => {
                self.ended = true;
                Poll::Ready(None)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl FusedStream for BinanceWebsocket {
    fn is_terminated(&self) -> bool {
        self.ended
    }
}
