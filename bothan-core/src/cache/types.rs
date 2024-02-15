use tokio::time::{Duration, Instant};

pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(3600);
pub const DEFAULT_EVICTION_CHECK_INTERVAL: Duration = Duration::from_secs(10);

#[derive(Debug, Clone, PartialEq)]
pub struct Entry<T> {
    pub(crate) data: T,
    pub(crate) last_used: Instant,
}

impl<T> Entry<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            last_used: Instant::now(),
        }
    }

    pub fn update(&mut self, data: T) {
        self.data = data;
    }

    pub fn bump_last_used(&mut self) {
        self.last_used = Instant::now();
    }
}
