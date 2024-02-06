use crate::types::PriceData;
use tokio::time::Instant;

pub const DEFAULT_TIMEOUT: u64 = 6000;

#[derive(Debug, Clone)]
pub struct StoredPriceData {
    pub data: PriceData,
    pub last_used: Instant,
}
