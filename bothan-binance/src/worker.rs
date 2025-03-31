use std::sync::Arc;
use std::time::Duration;

use bothan_lib::metrics::websocket::WebSocketMetrics;
use bothan_lib::store::{Store, WorkerStore};
use bothan_lib::worker::AssetWorker;
use bothan_lib::worker::error::AssetWorkerError;
use bothan_lib::worker::websocket::{PollOptions, start_polling};
use itertools;
use itertools::Itertools;
use tokio_util::sync::{CancellationToken, DropGuard};

use crate::WorkerOpts;
use crate::api::websocket::WebSocketConnector;

pub mod opts;

const WORKER_NAME: &str = "binance";
const TIMEOUT: Duration = Duration::from_secs(720);
const RECONNECT_BUFFER: Duration = Duration::from_secs(5);
const MAX_SUBSCRIPTION_PER_CONNECTION: usize = 200;
const MAX_RETRY: u64 = 3;

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
        let url = opts.url;
        let connector = Arc::new(WebSocketConnector::new(url));

        let worker_store = WorkerStore::new(store, WORKER_NAME);
        let token = CancellationToken::new();

        let poll_options = PollOptions {
            timeout: TIMEOUT,
            reconnect_buffer: RECONNECT_BUFFER,
            max_retry: MAX_RETRY,
            worker_name: WORKER_NAME.to_string(),
        };

        let metrics = WebSocketMetrics::new(WORKER_NAME);

        for (idx, set) in ids
            .into_iter()
            .chunks(opts.max_subscription_per_connection)
            .into_iter()
            .enumerate()
        {
            let worker_name_with_idx = format!("{}_{}", WORKER_NAME, idx);
            tokio::spawn(start_polling(
                token.child_token(),
                connector.clone(),
                worker_store.clone(),
                set.collect(),
                PollOptions {
                    worker_name: worker_name_with_idx,
                    ..poll_options.clone()
                },
                metrics.clone(),
            ));
        }

        Ok(Worker {
            _drop_guard: token.drop_guard(),
        })
    }
}
