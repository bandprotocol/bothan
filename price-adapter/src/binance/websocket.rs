use crate::mapper::types::Mapper;
use crate::mapper::BandStaticMapper;
use crate::types::WebsocketMessage;
use crate::{error::Error, types::PriceInfo};
use futures_util::{Stream, StreamExt};
use price_adapter_raw::types::WebsocketMessage as WebsocketMessageRaw;
use price_adapter_raw::BinanceWebsocket as BinanceWebsocketRaw;
use std::{
    collections::HashMap,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tokio::sync::Mutex;

// Generic struct `BinanceWebsocket` parameterized over a `Mapper` type.
pub struct BinanceWebsocket<M: Mapper> {
    raw: Option<Arc<Mutex<BinanceWebsocketRaw>>>,
    mapping_back: HashMap<String, String>,
    mapper: M,
}

impl<M: Mapper> BinanceWebsocket<M> {
    // Constructor for the `BinanceWebsocket` struct.
    pub fn new(mapper: M) -> Self {
        Self {
            raw: None,
            mapping_back: HashMap::new(),
            mapper,
        }
    }

    pub async fn connect(&mut self) -> Result<(), Error> {
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

    pub async fn subscribe(&mut self, symbols: &[&str]) -> Result<u32, Error> {
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

    pub fn is_connected(&self) -> bool {
        self.raw.is_some()
    }
}

impl BinanceWebsocket<BandStaticMapper> {
    pub fn default() -> Result<Self, Error> {
        let mapper = BandStaticMapper::from_source("binance")?;
        Ok(Self::new(mapper))
    }
}

impl<M: Mapper> Stream for BinanceWebsocket<M> {
    type Item = Result<WebsocketMessage, Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let Some(raw) = &self.raw else {
            return Poll::Ready(None);
        };

        let Ok(mut locked_raw) = raw.try_lock() else {
            cx.waker().wake_by_ref();
            return Poll::Pending;
        };

        match locked_raw.poll_next_unpin(cx) {
            Poll::Ready(Some(message)) => {
                let result = match message {
                    Ok(WebsocketMessageRaw::PriceInfo(price_info_raw)) => {
                        if let Some(symbol) = self.mapping_back.get(&price_info_raw.id) {
                            return Poll::Ready(Some(Ok(WebsocketMessage::PriceInfo(PriceInfo {
                                symbol: symbol.to_string(),
                                price: price_info_raw.price,
                                timestamp: price_info_raw.timestamp,
                            }))));
                        }

                        cx.waker().wake_by_ref();
                        return Poll::Pending;
                    }
                    Ok(_) => {
                        cx.waker().wake_by_ref();
                        return Poll::Pending;
                    }
                    Err(err) => Poll::Ready(Some(Err(err.into()))),
                };
                return result;
            }
            Poll::Ready(None) => return Poll::Ready(None),
            Poll::Pending => {
                return Poll::Pending;
            }
        };
    }
}
