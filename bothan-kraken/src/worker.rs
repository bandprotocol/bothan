use std::sync::Arc;
use std::time::Duration;

use bothan_lib::metrics::websocket::Metrics;
use bothan_lib::store::{Store, WorkerStore};
use bothan_lib::worker::AssetWorker;
use bothan_lib::worker::error::AssetWorkerError;
use bothan_lib::worker::websocket::start_polling;
use tokio_util::sync::{CancellationToken, DropGuard};
use tracing::{Instrument, Level, span};

use crate::WorkerOpts;
use crate::api::websocket::WebSocketConnector;

pub mod opts;

const TIMEOUT: Duration = Duration::from_secs(60);
const WORKER_NAME: &str = "kraken";

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
        let url = opts.url;
        let connector = Arc::new(WebSocketConnector::new(url));

        let worker_store = WorkerStore::new(store, WORKER_NAME);

        let token = CancellationToken::new();

        let metrics = Metrics::new(WORKER_NAME, WORKER_NAME.to_string());

        let span = span!(Level::INFO, "source", name = WORKER_NAME);
        tokio::spawn(
            start_polling(
                token.child_token(),
                connector,
                worker_store,
                ids,
                TIMEOUT,
                metrics,
            )
            .instrument(span),
        );

        Ok(Worker {
            _drop_guard: token.drop_guard(),
        })
    }
}
