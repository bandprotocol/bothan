use tokio::time::{Duration, Instant};

use crate::types::PriceData;

pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(6000);
pub const DEFAULT_CLEANUP_INTERVAL: Duration = Duration::from_secs(10);

#[derive(Debug, Clone)]
pub struct StoredPriceData {
    pub data: PriceData,
    pub last_used: Instant,
}
