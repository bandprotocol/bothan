use tokio::time::Duration;

pub(crate) const DEFAULT_UPDATE_INTERVAL: Duration = Duration::from_secs(60);
pub(crate) const DEFAULT_PAGE_SIZE: usize = 250;
pub(crate) const DEFAULT_PAGE_QUERY_DELAY: Duration = Duration::from_secs(12);
