//! Data types for interacting with the CoinMarketCap Worker.
use tokio::time::Duration;

pub(crate) const DEFAULT_UPDATE_INTERVAL: Duration = Duration::from_secs(60);
