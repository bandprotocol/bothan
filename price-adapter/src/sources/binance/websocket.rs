use crate::mappers::BandStaticMapper;
use crate::sources::BandStableCoin;
use crate::types::{Mapper, SettingResponse, Source, WebSocketSource, WebsocketMessage};
use crate::{error::Error, types::PriceInfo};
use futures_util::{stream::FusedStream, Stream, StreamExt};
use price_adapter_raw::{
    types::WebsocketMessage as WebsocketMessageRaw, BinanceWebsocket as BinanceWebsocketRaw,
};
use std::time::Duration;
use std::{
    collections::HashMap,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tokio::sync::Mutex;
use tokio::time::sleep;

pub type DefaultBinanceWebsocket = BinanceWebsocket<BandStaticMapper, BandStableCoin>;

/// A generic struct `BinanceWebsocket` parameterized over `Mapper` and `Source` types.
pub struct BinanceWebsocket<M: Mapper, S: Source> {
    mapper: M,
    usdt_source: Arc<S>,
    usdt_interval: Duration,

    usdt_price: Arc<Mutex<Option<PriceInfo>>>,
    raw: Arc<Mutex<BinanceWebsocketRaw>>,
    mapping_back: HashMap<String, String>,
    ended: bool,
}

impl<M: Mapper, S: Source> BinanceWebsocket<M, S> {
    /// Constructor for the `BinanceWebsocket` struct.
    pub fn new(mapper: M, usdt_source: S, usdt_interval: Duration) -> Self {
        Self {
            mapper,
            usdt_source: Arc::new(usdt_source),
            usdt_interval,
            usdt_price: Arc::new(Mutex::new(None)),
            raw: Arc::new(Mutex::new(BinanceWebsocketRaw::new(
                "wss://stream.binance.com:9443",
            ))),
            mapping_back: HashMap::new(),
            ended: false,
        }
    }
}

// Implementing the WebSocketSource trait for BinanceWebsocket.
#[async_trait::async_trait]
impl<M: Mapper, S: Source> WebSocketSource for BinanceWebsocket<M, S> {
    /// Asynchronous function to connect to the WebSocket.
    async fn connect(&mut self) -> Result<(), Error> {
        let mut locked_raw = self.raw.lock().await;
        if !locked_raw.is_connected() {
            locked_raw.connect().await?;
        }
        drop(locked_raw);

        let cloned_usdt_source = Arc::clone(&self.usdt_source);
        let cloned_usdt_price = Arc::clone(&self.usdt_price);
        let cloned_usdt_interval = self.usdt_interval;

        tokio::spawn(async move {
            loop {
                let price_info = cloned_usdt_source.get_price("USDT").await;
                let mut locked_usdt_price = cloned_usdt_price.lock().await;
                if let Ok(price) = price_info {
                    *locked_usdt_price = Some(price);
                } else {
                    *locked_usdt_price = None;
                }
                drop(locked_usdt_price);

                sleep(cloned_usdt_interval).await;
            }
        });

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

        let mut locked_raw = self.raw.lock().await;
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

        let mut locked_raw = self.raw.lock().await;
        locked_raw
            .unsubscribe(ids.as_slice())
            .await
            .map_err(Error::PriceAdapterRawError)
    }

    /// Check if the WebSocket is connected.
    async fn is_connected(&self) -> bool {
        let locked_raw = self.raw.lock().await;
        locked_raw.is_connected()
    }
}

// Implementing BinanceWebsocket for specific types (BandStaticMapper, BandStableCoin).
impl DefaultBinanceWebsocket {
    /// Constructor for creating a new BinanceWebsocket with default settings.
    pub fn new_with_default() -> Result<Self, Error> {
        let mapper = BandStaticMapper::from_source("binance")?;
        let band_stable_coin = BandStableCoin::new();
        Ok(Self::new(mapper, band_stable_coin, Duration::from_secs(5)))
    }
}

// Implementing Stream for BinanceWebsocket.
impl<M: Mapper, S: Source> Stream for BinanceWebsocket<M, S> {
    type Item = Result<WebsocketMessage, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.ended {
            return Poll::Ready(None);
        }

        let Ok(mut locked_raw) = self.raw.try_lock() else {
            cx.waker().wake_by_ref();
            return Poll::Pending;
        };

        let clone_usdt_price = Arc::clone(&self.usdt_price);
        let Ok(locked_usdt_price) = clone_usdt_price.try_lock() else {
            cx.waker().wake_by_ref();
            return Poll::Pending;
        };

        let Some(usdt_price) = &*locked_usdt_price else {
            cx.waker().wake_by_ref();
            return Poll::Pending;
        };

        match locked_raw.poll_next_unpin(cx) {
            Poll::Ready(Some(message)) => match message {
                Ok(WebsocketMessageRaw::PriceInfo(price_info_raw)) => {
                    tracing::trace!("received price info raw: {}", price_info_raw);
                    if let Some(symbol) = self.mapping_back.get(&price_info_raw.id) {
                        Poll::Ready(Some(Ok(WebsocketMessage::PriceInfo(PriceInfo {
                            symbol: symbol.to_string(),
                            price: price_info_raw.price / usdt_price.price,
                            timestamp: price_info_raw.timestamp,
                        }))))
                    } else {
                        // If symbol not found, wake up the waker and return Pending.
                        tracing::trace!("received symbol doesn't match");
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                }
                Ok(WebsocketMessageRaw::SettingResponse(response)) => {
                    tracing::trace!("received setting response raw: {:?}", response);
                    Poll::Ready(Some(Ok(WebsocketMessage::SettingResponse(
                        SettingResponse {
                            data: response.data,
                        },
                    ))))
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

// Implementing FusedStream for BinanceWebsocket.
impl<M: Mapper, S: Source> FusedStream for BinanceWebsocket<M, S> {
    /// Check if the stream is terminated.
    fn is_terminated(&self) -> bool {
        self.ended
    }
}
