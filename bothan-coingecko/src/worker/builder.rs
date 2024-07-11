use std::sync::Arc;

use tokio::time::{interval, Duration};

use bothan_core::store::Store;

use crate::api::error::BuilderError;
use crate::api::CoinGeckoRestAPIBuilder;
use crate::worker::asset_worker::start_asset_worker;
use crate::worker::opts::CoinGeckoWorkerBuilderOpts;
use crate::worker::CoinGeckoWorker;

/// Builds a `CoinGeckoWorker` with custom options.
/// Methods can be chained to set the configuration values and the
/// service is constructed by calling the [`build`](CoinGeckoWorker::build) method.
/// # Example
/// ```no_run
/// use bothan_coingecko::CoinGeckoWorkerBuilder;
///
///
/// #[tokio::main]
/// async fn main() {
///     let worker = CoinGeckoWorkerBuilder::default()
///         .build()
///         .await
///         .unwrap();
///
///     // use worker ...
/// }
/// ```
pub struct CoinGeckoWorkerBuilder {
    store: Arc<Store>,
    opts: CoinGeckoWorkerBuilderOpts,
}

impl CoinGeckoWorkerBuilder {
    /// Returns a new `CoinGeckoWorkerBuilder` with the given options.
    pub fn new(store: Arc<Store>, opts: CoinGeckoWorkerBuilderOpts) -> Self {
        Self { store, opts }
    }

    /// Set the URL for the `CoinGeckoWorker`.
    /// The default URL is `DEFAULT_URL` when no API key is provided
    /// and is `DEFAULT_PRO_URL` when an API key is provided.
    pub fn with_url<T: Into<String>>(mut self, url: T) -> Self {
        self.opts.url = Some(url.into());
        self
    }

    /// Sets the API key for the `CoinGeckoWorker`.
    /// The default api key is `None`.
    pub fn with_api_key<T: Into<String>>(mut self, api_key: T) -> Self {
        self.opts.api_key = Some(api_key.into());
        self
    }

    /// Sets the User-Agent header for the `CoinGeckoWorker`.
    /// The default user agent is `DEFAULT_USER_AGENT`.
    pub fn with_user_agent<T: Into<String>>(mut self, user_agent: T) -> Self {
        self.opts.user_agent = user_agent.into();
        self
    }

    /// Sets the update interval for the `CoinGeckoWorker`.
    /// The default interval is `DEFAULT_UPDATE_INTERVAL`.
    pub fn with_update_interval(mut self, update_interval: Duration) -> Self {
        self.opts.update_interval = update_interval;
        self
    }

    /// Sets the page size for the service.
    /// The default is `DEFAULT_PAGE_SIZE`.
    pub fn with_page_size(mut self, page_size: usize) -> Self {
        self.opts.page_size = page_size;
        self
    }

    /// Sets the page query delay for the `CoinGeckoWorker`.
    /// The default delay is `DEFAULT_PAGE_QUERY_DELAY`.
    pub fn with_page_query_delay(mut self, page_query_delay: Duration) -> Self {
        self.opts.page_query_delay = page_query_delay;
        self
    }

    /// Sets the store for the `CoinGeckoWorker`.
    /// If not set, the store is created and owned by the worker.
    pub fn with_store(mut self, store: Arc<Store>) -> Self {
        self.store = store;
        self
    }

    /// Creates the configured `CoinGeckoWorker`.
    pub async fn build(self) -> Result<Arc<CoinGeckoWorker>, BuilderError> {
        let api =
            CoinGeckoRestAPIBuilder::new(self.opts.url, self.opts.api_key, self.opts.user_agent)
                .build()?;

        let worker = Arc::new(CoinGeckoWorker::new(api, self.store));

        start_asset_worker(
            Arc::downgrade(&worker),
            interval(self.opts.update_interval),
            self.opts.page_size,
            self.opts.page_query_delay,
        );

        Ok(worker)
    }
}

impl Default for CoinGeckoWorkerBuilder {
    /// Creates a new `CoinGeckoWorkerBuilder` with its default values.
    fn default() -> Self {
        Self::new(
            Arc::new(Store::default()),
            CoinGeckoWorkerBuilderOpts::default(),
        )
    }
}