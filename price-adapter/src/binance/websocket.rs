use crate::mapper::{types::Mapper, BandStaticMapper};
use crate::stable_coin::{types::StableCoin, BandStableCoin};
use crate::types::{WebsocketMessage, WebsocketPriceAdapter};
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

// Generic struct `BinanceWebsocket` parameterized over a `Mapper` type.
pub struct BinanceWebsocket<M: Mapper, S: StableCoin> {
    mapper: M,
    stable_coin: S,

    raw: Option<Arc<Mutex<BinanceWebsocketRaw>>>,
    mapping_back: HashMap<String, String>,
    ended: bool,
}
impl<M: Mapper, S: StableCoin> BinanceWebsocket<M, S> {
    // Constructor for the `BinanceWebsocket` struct.
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

#[async_trait::async_trait]
impl<M: Mapper, S: StableCoin> WebsocketPriceAdapter for BinanceWebsocket<M, S> {
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

    fn is_connected(&self) -> bool {
        self.raw.is_some()
    }
}

impl BinanceWebsocket<BandStaticMapper, BandStableCoin> {
    pub fn default() -> Result<Self, Error> {
        let mapper = BandStaticMapper::from_source("binance")?;
        let stable_coin = BandStableCoin::new();
        Ok(Self::new(mapper, stable_coin))
    }
}

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
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                }
                Ok(_) => {
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
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

impl<M: Mapper, S: StableCoin> FusedStream for BinanceWebsocket<M, S> {
    fn is_terminated(&self) -> bool {
        self.ended
    }
}
