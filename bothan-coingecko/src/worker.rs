use crate::api::error::ProviderError;
use crate::api::{RestApi, RestApiBuilder};
use crate::WorkerOpts;
use bothan_lib::store::{Store, WorkerStore};
use bothan_lib::types::AssetState;
use bothan_lib::worker::error::AssetWorkerError;
use bothan_lib::worker::rest::{start_polling, AssetInfoProvider};
use bothan_lib::worker::AssetWorker;
use std::sync::{Arc, Weak};

pub mod opts;

const WORKER_NAME: &str = "coingecko";

pub struct Worker<S: Store> {
    // The `api` is owned by this struct to ensure that any weak references
    // are properly cleaned up when the worker is dropped.
    #[allow(dead_code)]
    api: Arc<RestApi>,
    store: WorkerStore<S>,
}

#[async_trait::async_trait]
impl<S: Store + 'static> AssetWorker<S> for Worker<S> {
    type Opts = WorkerOpts;

    fn name(&self) -> &'static str {
        WORKER_NAME
    }

    async fn build(opts: Self::Opts, store: &S) -> Result<Self, AssetWorkerError> {
        let api = Arc::new(RestApiBuilder::new(opts.url, opts.user_agent, opts.api_key).build()?);

        let worker_store = WorkerStore::new(store, WORKER_NAME);

        tokio::spawn(start_polling(
            opts.update_interval,
            Arc::downgrade(&api) as Weak<dyn AssetInfoProvider<Error = ProviderError>>,
            worker_store.clone(),
        ));

        Ok(Worker {
            api,
            store: worker_store,
        })
    }

    async fn get_asset(&self, id: &str) -> Result<AssetState, AssetWorkerError> {
        Ok(self.store.get_asset(id).await?)
    }

    async fn set_query_ids(&self, ids: Vec<String>) -> Result<(), AssetWorkerError> {
        self.store.set_query_ids(ids).await?;
        Ok(())
    }
}
