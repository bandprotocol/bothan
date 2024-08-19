use crate::worker::AssetWorker;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::sync::Arc;

pub type WorkerMap<'a> = HashMap<String, Arc<dyn AssetWorker + 'a>>;

#[derive(Debug, Clone, PartialEq)]
pub enum PriceState {
    Available(Decimal),
    Unavailable,
    Unsupported,
}
