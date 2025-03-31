use bothan_lib::store::{Store, WorkerStore};
use bothan_lib::worker::AssetWorker;
use bothan_lib::worker::error::AssetWorkerError;
use bothan_lib::worker::rest::start_polling;
use tokio_util::sync::{CancellationToken, DropGuard};
use tracing::{Instrument, Level, span};

use crate::WorkerOpts;
use crate::api::RestApiBuilder;

pub mod opts;

const WORKER_NAME: &str = "coingecko";

pub struct Worker {
    // We keep this DropGuard to ensure that all internal processes
    // that the worker holds are dropped when the worker is dropped.
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
        let api = RestApiBuilder::new(opts.url, opts.user_agent, opts.api_key).build()?;
        let worker_store = WorkerStore::new(store, WORKER_NAME);
        let token = CancellationToken::new();

        let span = span!(Level::INFO, "source", name = WORKER_NAME);
        tokio::spawn(
            start_polling(
                token.child_token(),
                opts.update_interval,
                api,
                worker_store,
                ids,
            )
            .instrument(span),
        );

        Ok(Worker {
            _drop_guard: token.drop_guard(),
        })
    }
}
