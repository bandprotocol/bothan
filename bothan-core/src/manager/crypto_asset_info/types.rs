use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use rust_decimal::Decimal;

use crate::monitoring::records::{SignalComputationRecord, SignalComputationRecords};
use crate::worker::AssetWorker;

pub const MONITORING_TTL: Duration = Duration::from_secs(60);
pub const HEARTBEAT: Duration = Duration::from_secs(60);

pub type WorkerMap<'a> = HashMap<String, Arc<dyn AssetWorker + 'a>>;
pub type PriceSignalComputationRecords = SignalComputationRecords<Decimal, Decimal>;
pub type PriceSignalComputationRecord = SignalComputationRecord<Decimal, Decimal>;

#[derive(Debug, Clone, PartialEq)]
pub enum PriceState {
    Available(Decimal),
    Unavailable,
    Unsupported,
}
