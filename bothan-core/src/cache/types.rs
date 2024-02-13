use tokio::time::{Duration, Instant};

use crate::types::PriceData;

pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(6000);
pub const DEFAULT_EVICTION_CHECK_INTERVAL: Duration = Duration::from_secs(10);

#[derive(Debug, Clone)]
pub struct StoredPriceData {
    pub data: PriceData,
    pub last_used: Instant,
}

impl StoredPriceData {
    pub fn new(data: PriceData) -> Self {
        Self {
            data,
            last_used: Instant::now(),
        }
    }

    pub fn update(&mut self, data: PriceData) {
        self.data = data;
        self.last_used = Instant::now();
    }

    pub fn bump_last_used(&mut self) {
        self.last_used = Instant::now();
    }
}
