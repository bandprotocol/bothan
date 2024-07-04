use tokio::time::Duration;

pub const DEFAULT_CHANNEL_SIZE: usize = 1000;
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(720);
pub const RECONNECT_BUFFER: Duration = Duration::from_secs(5);
