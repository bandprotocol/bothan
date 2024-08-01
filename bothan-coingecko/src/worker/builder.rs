use std::sync::Arc;

use tokio::time::Duration;

use bothan_core::store::WorkerStore;
use bothan_core::worker::AssetWorkerBuilder;

use crate::api::error::BuildError;
use crate::api::CoinGeckoRestAPIBuilder;
use crate::worker::asset_worker::start_asset_worker;
use crate::worker::opts::CoinGeckoWorkerBuilderOpts;
use crate::worker::CoinGeckoWorker;

/// Builds a `CoinGeckoWorker` with custom options.
/// Methods can be chained to set the configuration values and the
/// service is constructed by calling the [`build`](CoinGeckoWorker::build) method.
pub struct CoinGeckoWorkerBuilder {
    store: WorkerStore,
    opts: CoinGeckoWorkerBuilderOpts,
}

impl CoinGeckoWorkerBuilder {
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

    /// Sets the store for the `CoinGeckoWorker`.
    /// If not set, the store is created and owned by the worker.
    pub fn with_store(mut self, store: WorkerStore) -> Self {
        self.store = store;
        self
    }
}

#[async_trait::async_trait]
impl<'a> AssetWorkerBuilder<'a> for CoinGeckoWorkerBuilder {
    type Opts = CoinGeckoWorkerBuilderOpts;
    type Worker = CoinGeckoWorker;
    type Error = BuildError;

    /// Returns a new `CoinGeckoWorkerBuilder` with the given options.
    fn new(store: WorkerStore, opts: Self::Opts) -> Self {
        Self { store, opts }
    }
    /// Creates the configured `CoinGeckoWorker`.
    async fn build(self) -> Result<Arc<Self::Worker>, Self::Error> {
        let api =
            CoinGeckoRestAPIBuilder::new(self.opts.user_agent, self.opts.url, self.opts.api_key)
                .build()?;

        let worker = Arc::new(CoinGeckoWorker::new(api, self.store));

        start_asset_worker(Arc::downgrade(&worker), self.opts.update_interval);

        Ok(worker)
    }
}
