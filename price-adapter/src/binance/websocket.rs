use crate::error::Error;
use crate::mapper::types::Mapper;
use crate::mapper::BandStaticMapper;
use futures_util::{Stream, StreamExt};
use price_adapter_raw::types::WebsocketMessage as WebsocketMessageRaw;
use price_adapter_raw::BinanceWebsocket as BinanceWebsocketRaw;
use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tokio::sync::Mutex;

// Generic struct `BinanceWebsocket` parameterized over a `Mapper` type.
pub struct BinanceWebsocket<M: Mapper> {
    raw: Option<Arc<Mutex<BinanceWebsocketRaw>>>,
    mapper: M,
}

impl<M: Mapper> BinanceWebsocket<M> {
    // Constructor for the `BinanceWebsocket` struct.
    pub fn new(mapper: M) -> Self {
        Self { raw: None, mapper }
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
    type Item = Result<u64, Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let Some(raw) = &self.raw else {
            return Poll::Ready(None);
        };

        loop {
            let Ok(mut locked_raw) = raw.try_lock() else {

                continue;
            };

            match locked_raw.poll_next_unpin(cx) {
                Poll::Ready(Some(message)) => {
                    let result = match message {
                        Ok(WebsocketMessageRaw::PriceInfo(price_info)) => {
                            println!("internal: {:?}", price_info);
                            Poll::Ready(Some(Ok(0_u64)))
                        }
                        Ok(_) => continue,
                        Err(err) => Poll::Ready(Some(Err(err.into()))),
                    };
                    return result;
                }
                Poll::Ready(None) => return Poll::Ready(None),
                Poll::Pending => return Poll::Pending,
            };
        }
    }
}
