//! Types and constants for the crypto asset info manager.

use std::time::Duration;

use bothan_lib::types::AssetInfo;
use rust_decimal::Decimal;

use crate::monitoring::types::SignalComputationRecord;

pub const MONITORING_TTL: Duration = Duration::from_secs(60);
pub const HEARTBEAT: Duration = Duration::from_secs(60);

pub type PriceSignalComputationRecord = SignalComputationRecord<AssetInfo, Decimal>;

#[derive(Debug, Clone, PartialEq)]
pub enum PriceState {
    Available(Decimal),
    Unavailable,
    Unsupported,
}

pub struct CryptoAssetManagerInfo {
    pub bothan_version: String,
    pub registry_hash: String,
    pub registry_version_requirement: String,
    pub active_sources: Vec<String>,
    pub monitoring_enabled: bool,
}

impl CryptoAssetManagerInfo {
    pub fn new(
        bothan_version: String,
        registry_hash: String,
        registry_version_requirement: String,
        active_sources: Vec<String>,
        monitoring_enabled: bool,
    ) -> Self {
        CryptoAssetManagerInfo {
            bothan_version,
            registry_hash,
            registry_version_requirement,
            active_sources,
            monitoring_enabled,
        }
    }
}
