use std::sync::Arc;

use tokio::time::Duration;

use bothan_core::store::WorkerStore;
use bothan_core::worker::AssetWorkerBuilder;

use crate::api::error::BuildError;
use crate::api::CoinMarketCapRestAPIBuilder;
use crate::worker::asset_worker::start_asset_worker;
use crate::worker::opts::CoinMarketCapWorkerBuilderOpts;
use crate::worker::CoinMarketCapWorker;

/// Builds a `CoinMarketCapWorker` with custom options.
/// Methods can be chained to set the configuration values and the
/// service is constructed by calling the [`build`](CoinMarketCapWorker::build) method.
pub struct CoinMarketCapWorkerBuilder {
    store: WorkerStore,
    opts: CoinMarketCapWorkerBuilderOpts,
}

impl CoinMarketCapWorkerBuilder {
    /// Set the URL for the `CoinMarketCapWorker`.
    /// The default URL is `DEFAULT_URL`
    pub fn with_url<T: Into<String>>(mut self, url: T) -> Self {
        self.opts.url = url.into();
        self
    }

    /// Sets the API key for the `CoinMarketCapWorker`.
    /// The default api key is `None`.
    pub fn with_api_key<T: Into<String>>(mut self, api_key: T) -> Self {
        self.opts.api_key = Some(api_key.into());
        self
    }

    /// Sets the update interval for the `CoinMarketCapWorker`.
    /// The default interval is `DEFAULT_UPDATE_INTERVAL`.
    pub fn with_update_interval(mut self, update_interval: Duration) -> Self {
        self.opts.update_interval = update_interval;
        self
    }

    /// Sets the update supported assets interval for the `CoinMarketCapWorker`.
    /// The default interval is `DEFAULT_UPDATE_SUPPORTED_ASSETS_INTERVAL`.
    pub fn with_update_supported_assets_interval(mut self, update_interval: Duration) -> Self {
        self.opts.update_supported_assets_interval = update_interval;
        self
    }

    /// Sets the store for the `CoinMarketCapWorker`.
    /// If not set, the store is created and owned by the worker.
    pub fn with_store(mut self, store: WorkerStore) -> Self {
        self.store = store;
        self
    }
}

#[async_trait::async_trait]
impl<'a> AssetWorkerBuilder<'a> for CoinMarketCapWorkerBuilder {
    type Opts = CoinMarketCapWorkerBuilderOpts;
    type Worker = CoinMarketCapWorker;
    type Error = BuildError;

    /// Returns a new `CoinMarketCapWorkerBuilder` with the given options.
    fn new(store: WorkerStore, opts: Self::Opts) -> Self {
        Self { store, opts }
    }

    /// Returns the name of the worker.
    fn worker_name() -> &'static str {
        "coinmarketcap"
    }

    /// Creates the configured `CoinMarketCapWorker`.
    async fn build(self) -> Result<Arc<Self::Worker>, Self::Error> {
        let api = CoinMarketCapRestAPIBuilder::new(self.opts.url, self.opts.api_key).build()?;

        let worker = Arc::new(CoinMarketCapWorker::new(api, self.store));

        start_asset_worker(Arc::downgrade(&worker), self.opts.update_interval);

        Ok(worker)
    }
}
