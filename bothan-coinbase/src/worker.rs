use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use bothan_lib::store::{Store, WorkerStore};
use bothan_lib::worker::AssetWorker;
use bothan_lib::worker::error::AssetWorkerError;
use bothan_lib::worker::websocket::{PollOptions, start_polling};
use itertools::Itertools;
use tokio_util::sync::{CancellationToken, DropGuard};
use tracing::{Instrument, Level, span};

use crate::WorkerOpts;
use crate::api::websocket::WebSocketConnector;

pub mod opts;

const WORKER_NAME: &str = "coinbase";
const TIMEOUT: Duration = Duration::from_secs(60);
const RECONNECT_BUFFER: Duration = Duration::from_secs(5);
const MAX_RETRY: u64 = 3;
const MAX_SUBSCRIPTION_PER_CONNECTION: usize = 10;

pub struct Worker {
    // We keep this DropGuard to ensure that all internal processes
    // that the worker holds are dropped when the worker is dropped.
    _drop_guard: DropGuard,
}

#[async_trait]
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
            meter_name: WORKER_NAME,
        };

        let token = CancellationToken::new();

        for (i, chunk) in ids
            .into_iter()
            .chunks(opts.max_subscription_per_connection)
            .into_iter()
            .enumerate()
        {
            let span = span!(
                Level::INFO,
                "source",
                name = WORKER_NAME,
                connection_idx = i
            );
            tokio::spawn(
                start_polling(
                    token.child_token(),
                    connector.clone(),
                    worker_store.clone(),
                    chunk.collect(),
                    poll_options.clone(),
                )
                .instrument(span),
            );
        }

        Ok(Worker {
            _drop_guard: token.drop_guard(),
        })
    }
}
