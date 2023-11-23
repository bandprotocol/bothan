use crate::mappers::BandStaticMapper;
use crate::stable_coin::BandStableCoin;
use crate::types::{Mapper, SettingResponse, StableCoin, WebSocketSource, WebsocketMessage};
use crate::{error::Error, types::PriceInfo};
use futures_util::{stream::FusedStream, Stream, StreamExt};
use price_adapter_raw::{
    types::WebsocketMessage as WebsocketMessageRaw, BinanceWebsocket as BinanceWebsocketRaw,
};
use std::{
    collections::HashMap,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tokio::sync::Mutex;

/// A generic struct `BinanceWebsocket` parameterized over `Mapper` and `StableCoin` types.
pub struct BinanceWebsocket<M: Mapper, S: StableCoin> {
    mapper: M,
    stable_coin: S,
    raw: Option<Arc<Mutex<BinanceWebsocketRaw>>>,
    mapping_back: HashMap<String, String>,
    ended: bool,
}

impl<M: Mapper, S: StableCoin> BinanceWebsocket<M, S> {
    /// Constructor for the `BinanceWebsocket` struct.
    pub fn new(mapper: M, stable_coin: S) -> Self {
        Self {
            mapper,
            stable_coin,
            raw: None,
            mapping_back: HashMap::new(),
            ended: false,
        }
    }
}

// Implementing the WebSocketSource trait for BinanceWebsocket.
#[async_trait::async_trait]
impl<M: Mapper, S: StableCoin> WebSocketSource for BinanceWebsocket<M, S> {
    /// Asynchronous function to connect to the WebSocket.
    async fn connect(&mut self) -> Result<(), Error> {
        let raw = Arc::new(Mutex::new(BinanceWebsocketRaw::new(
            "wss://stream.binance.com:9443",
        )));

        let mut locked_raw = raw.lock().await;
        if !locked_raw.is_connected() {
            locked_raw.connect().await?;
        }
        drop(locked_raw);

        self.raw = Some(raw);

        Ok(())
    }

    /// Asynchronous function to subscribe to symbols.
    async fn subscribe(&mut self, symbols: &[&str]) -> Result<u32, Error> {
        // Retrieve the symbol-to-id mapping from the provided mapper.
        let mapping = self.mapper.get_mapping().await?;

        for (key, value) in mapping {
            if let Some(pair) = value.as_str() {
                self.mapping_back
                    .insert(pair.to_string().to_uppercase(), key.to_string());
            }
        }

        let ids: Vec<&str> = symbols
            .iter()
            .filter_map(|&symbol| mapping.get(symbol))
            .filter_map(|val| val.as_str())
            .collect();

        if ids.len() != symbols.len() {
            return Err(Error::UnsupportedSymbol);
        }

        let raw = self.raw.as_mut().ok_or(Error::Unknown)?;
        let mut locked_raw = raw.lock().await;

        locked_raw
            .subscribe(ids.as_slice())
            .await
            .map_err(Error::PriceAdapterRawError)
    }

    /// Asynchronous function to unsubscribe from symbols.
    async fn unsubscribe(&mut self, symbols: &[&str]) -> Result<u32, Error> {
        let ids: Vec<&str> = symbols
            .iter()
            .filter_map(|&symbol| self.mapping_back.get(symbol))
            .map(|string_ref| string_ref.as_str())
            .collect();

        if ids.len() != symbols.len() {
            return Err(Error::UnsupportedSymbol);
        }

        let raw = self.raw.as_mut().ok_or(Error::Unknown)?;
        let mut locked_raw = raw.lock().await;
        locked_raw
            .unsubscribe(ids.as_slice())
            .await
            .map_err(Error::PriceAdapterRawError)
    }

    /// Check if the WebSocket is connected.
    fn is_connected(&self) -> bool {
        self.raw.is_some()
    }
}

// Implementing BinanceWebsocket for specific types (BandStaticMapper, BandStableCoin).
impl BinanceWebsocket<BandStaticMapper, BandStableCoin> {
    /// Constructor for creating a new BinanceWebsocket with default settings.
    pub fn new_with_default() -> Result<Self, Error> {
        let mapper = BandStaticMapper::from_source("binance")?;
        let stable_coin = BandStableCoin::new();
        Ok(Self::new(mapper, stable_coin))
    }
}

// Implementing Stream for BinanceWebsocket.
impl<M: Mapper, S: StableCoin> Stream for BinanceWebsocket<M, S> {
    type Item = Result<WebsocketMessage, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.ended {
            return Poll::Ready(None);
        }

        let Some(raw) = &self.raw else {
            return Poll::Ready(None);
        };

        let Ok(mut locked_raw) = raw.try_lock() else {
            cx.waker().wake_by_ref();
            return Poll::Pending;
        };

        match locked_raw.poll_next_unpin(cx) {
            Poll::Ready(Some(message)) => match message {
                Ok(WebsocketMessageRaw::PriceInfo(price_info_raw)) => {
                    if let Some(symbol) = self.mapping_back.get(&price_info_raw.id) {
                        let Ok(usdt_price) = self.stable_coin.get_price("USDT".to_string()) else {
                            return Poll::Pending;
                        };

                        Poll::Ready(Some(Ok(WebsocketMessage::PriceInfo(PriceInfo {
                            symbol: symbol.to_string(),
                            price: price_info_raw.price / usdt_price,
                            timestamp: price_info_raw.timestamp,
                        }))))
                    } else {
                        // If symbol not found, wake up the waker and return Pending.
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                }
                Ok(WebsocketMessageRaw::SettingResponse(response)) => Poll::Ready(Some(Ok(
                    WebsocketMessage::SettingResponse(SettingResponse {
                        data: response.data,
                    }),
                ))),
                Err(err) => Poll::Ready(Some(Err(err.into()))),
            },
            Poll::Ready(None) => {
                drop(locked_raw);
                self.ended = true;
                Poll::Ready(None)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

// Implementing FusedStream for BinanceWebsocket.
impl<M: Mapper, S: StableCoin> FusedStream for BinanceWebsocket<M, S> {
    /// Check if the stream is terminated.
    fn is_terminated(&self) -> bool {
        self.ended
    }
}
