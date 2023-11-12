use std::{
    collections::HashMap,
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::{stream::FusedStream, Stream, StreamExt};
use serde::Deserialize;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::{error::Error, types::PriceInfo};

pub struct BinanceWebsocket {
    socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
    stream_id_to_symbols: HashMap<String, (String, String)>,
    ended: bool,
}

#[derive(Debug, Deserialize)]
pub struct MiniTickerInfo {
    #[serde(rename = "c")]
    current_price: String,
}

#[derive(Debug, Deserialize)]
pub struct MiniTickerResponse {
    pub stream: String,
    pub data: MiniTickerInfo,
}

impl BinanceWebsocket {
    pub fn new(
        socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
        symbols: &[(String, String)],
    ) -> Self {
        let stream_id_to_symbols = symbols
            .iter()
            .map(|(base, quote)| {
                let stream_id = format!("{}{}@miniTicker", base, quote);
                (stream_id, (base.clone(), quote.clone()))
            })
            .collect();

        Self {
            socket,
            stream_id_to_symbols,
            ended: false,
        }
    }

    fn to_price_info(&self, mini_ticker_resp: String) -> Result<PriceInfo, Error> {
        let mini_ticker_response = serde_json::from_str::<MiniTickerResponse>(&mini_ticker_resp)?;

        let (base, quote) = self
            .stream_id_to_symbols
            .get(&mini_ticker_response.stream)
            .ok_or(Error::NotFound(mini_ticker_response.stream.clone()))?;

        let price = mini_ticker_response.data.current_price.parse::<f64>()?;
        Ok(PriceInfo {
            base: base.to_string(),
            quote: quote.to_string(),
            price,
            timestamp: chrono::Utc::now().timestamp() as u64,
        })
    }
}

impl Stream for BinanceWebsocket {
    type Item = Result<PriceInfo, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.ended {
            return Poll::Ready(None);
        }

        match self.socket.poll_next_unpin(cx) {
            Poll::Ready(Some(message)) => match message {
                Ok(Message::Text(text)) => {
                    let price_info = match self.to_price_info(text) {
                        Ok(info) => info,
                        Err(err) => {
                            tracing::error!("cannot convert received text to PriceInfo: {}", err);
                            return Poll::Ready(Some(Err(err.into())));
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
