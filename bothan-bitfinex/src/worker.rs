use bothan_lib::metrics::rest::RestMetrics;
use bothan_lib::store::{Store, WorkerStore};
use bothan_lib::worker::AssetWorker;
use bothan_lib::worker::error::AssetWorkerError;
use bothan_lib::worker::rest::start_polling;
use tokio_util::sync::{CancellationToken, DropGuard};

use crate::api::builder::RestApiBuilder;
use crate::worker::opts::WorkerOpts;

const WORKER_NAME: &str = "bitfinex";

pub mod opts;

pub struct Worker {
    // We keep this DropGuard to ensure that all internal processes
    // to ensure that all internal processes that the worker holds are dropped
    // when the worker is dropped.
    _drop_guard: DropGuard,
}

#[async_trait::async_trait]
impl AssetWorker for Worker {
    type Opts = WorkerOpts;

    fn name(&self) -> &'static str {
        WORKER_NAME
    }

    async fn build<S: Store + 'static>(
        opts: Self::Opts,
        store: &S,
        ids: Vec<String>,
    ) -> Result<Self, AssetWorkerError> {
        let api = RestApiBuilder::new(&opts.url).build()?;
        let worker_store = WorkerStore::new(store, WORKER_NAME);

        let token = CancellationToken::new();

        let metrics = RestMetrics::new(WORKER_NAME);

        tokio::spawn(start_polling(
            token.child_token(),
            opts.update_interval,
            api,
            worker_store,
            ids,
            metrics,
        ));

        Ok(Worker {
            _drop_guard: token.drop_guard(),
        })
    }
}
