use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::{stream::FusedStream, Stream, StreamExt};
use reqwest::StatusCode;
use serde::Deserialize;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::{error::Error, types::PriceInfo};

/// A binance websocket object.
pub struct BinanceWebsocket {
    url: String,
    socket: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ended: bool,
}

#[derive(Debug, Deserialize)]
pub struct MiniTickerInfo {
    #[serde(rename = "s")]
    id: String,

    #[serde(rename = "c")]
    current_price: String,

    #[serde(rename = "E")]
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct MiniTickerResponse {
    pub stream: String,
    pub data: MiniTickerInfo,
}

fn parse_into_price_info(mini_ticker_resp: String) -> Result<PriceInfo, Error> {
    let mini_ticker_response = serde_json::from_str::<MiniTickerResponse>(&mini_ticker_resp)?;

    let MiniTickerInfo {
        id,
        current_price,
        timestamp,
        ..
    } = mini_ticker_response.data;

    let price = current_price.parse::<f64>()?;
    Ok(PriceInfo {
        id,
        price,
        timestamp,
    })
}

impl BinanceWebsocket {
    pub fn new(url: &str, ids: &[&str]) -> Self {
        let stream_ids = ids
            .iter()
            .map(|id| format!("{}@miniTicker", id))
            .collect::<Vec<_>>();

        let ws_url = format!("{}/stream?streams={}", url, stream_ids.join("/"));

        Self {
            url: ws_url,
            socket: None,
            ended: false,
        }
    }

    pub async fn connect(&mut self) -> Result<(), Error> {
        let (socket, response) = connect_async(&self.url).await?;

        let response_status = response.status();
        if StatusCode::is_success(&response_status) {
            tracing::error!("query request get error status {}", response_status);
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
    type Item = Result<PriceInfo, Error>;

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
                    let price_info = match parse_into_price_info(text) {
                        Ok(info) => info,
                        Err(err) => {
                            tracing::error!("cannot convert received text to PriceInfo: {}", err);
                            return Poll::Ready(Some(Err(err)));
                        }
                    };

                    Poll::Ready(Some(Ok(price_info)))
                }
                Ok(_) => {
                    tracing::error!("received non-text message");
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
