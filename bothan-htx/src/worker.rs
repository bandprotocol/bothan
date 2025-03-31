use std::sync::Arc;
use std::time::Duration;

use bothan_lib::metrics::websocket::WebSocketMetrics;
use bothan_lib::store::{Store, WorkerStore};
use bothan_lib::worker::AssetWorker;
use bothan_lib::worker::error::AssetWorkerError;
use bothan_lib::worker::websocket::{PollOptions, start_polling};
use tokio_util::sync::{CancellationToken, DropGuard};
use tracing::{Instrument, Level, span};

use crate::WorkerOpts;
use crate::api::WebSocketConnector;

pub mod opts;

const WORKER_NAME: &str = "htx";
const WORKER_IDX: u64 = 0;
const TIMEOUT: Duration = Duration::from_secs(60);
const RECONNECT_BUFFER: Duration = Duration::from_secs(5);
const MAX_RETRY: u64 = 3;

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

        let poll_options = PollOptions {
            timeout: TIMEOUT,
            reconnect_buffer: RECONNECT_BUFFER,
            max_retry: MAX_RETRY,
            worker_name: format!("{WORKER_NAME}_{WORKER_IDX}"),
        };

        let token = CancellationToken::new();

        let metrics = WebSocketMetrics::new(WORKER_NAME);

        let span = span!(Level::INFO, "source", name = WORKER_NAME);
        tokio::spawn(
            start_polling(
                token.child_token(),
                connector.clone(),
                worker_store.clone(),
                ids,
                poll_options.clone(),
                metrics,
            )
            .instrument(span),
        );

        Ok(Worker {
            _drop_guard: token.drop_guard(),
        })
    }
}
