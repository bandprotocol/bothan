use tokio::time::Duration;

pub(crate) const DEFAULT_UPDATE_INTERVAL: Duration = Duration::from_secs(60);
pub(crate) const DEFAULT_UPDATE_SUPPORTED_ASSETS_INTERVAL: Duration = Duration::from_secs(86400);
pub(crate) const DEFAULT_PAGE_SIZE: usize = 250;
