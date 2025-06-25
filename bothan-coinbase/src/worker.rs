//! Coinbase worker implementation.
//!
//! This module provides an implementation of the [`AssetWorker`] trait for interacting with
//! the Coinbase WebSocket API. It defines the [`Worker`], which is responsible for subscribing
//! to asset updates via WebSocket connections and storing the data into a shared [`WorkerStore`].
//!
//! The worker is configurable via [`WorkerOpts`] and uses [`WebSocketConnector`] to establish
//! WebSocket connections to Coinbase endpoints.
//!
//! # The module provides:
//!
//! - Subscription to asset updates via WebSocket connections in asynchronous tasks
//! - Ensures graceful cancellation by using a CancellationToken to signal shutdown and a DropGuard
//!   to automatically clean up resources when the worker is dropped
//! - Metrics collection for observability
//! - Configurable via endpoint URL and maximum subscriptions per connection
//!
//! # Examples
//!
//! ```ignore
//! use bothan_coinbase::worker::{Worker, WorkerOpts};
//! use bothan_lib::worker::AssetWorker;
//! use bothan_lib::store::Store;
//!
//! #[tokio::test]
//! async fn test<T: Store>(store: T) {
//!     let opts = WorkerOpts::default();
//!     let ids = vec!["BTC-USD".to_string(), "ETH-USD".to_string()];
//!
//!     let worker = Worker::build(opts, &store, ids).await?;
//! }
//! ```

use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use bothan_lib::metrics::websocket::Metrics;
use bothan_lib::store::{Store, WorkerStore};
use bothan_lib::worker::AssetWorker;
use bothan_lib::worker::error::AssetWorkerError;
use bothan_lib::worker::websocket::start_listening;
use itertools::Itertools;
use tokio_util::sync::{CancellationToken, DropGuard};
use tracing::{Instrument, Level, span};

use crate::WorkerOpts;
use crate::api::websocket::WebSocketConnector;

pub mod opts;

const WORKER_NAME: &str = "coinbase";
const TIMEOUT: Duration = Duration::from_secs(60);
const MAX_SUBSCRIPTION_PER_CONNECTION: usize = 10;

/// Asset worker for subscribing to asset updates via the Coinbase WebSocket API.
///
/// The `Worker` manages asynchronous WebSocket connections for asset updates
/// and ensures resources are properly cleaned up when dropped.
pub struct Worker {
    // We keep this DropGuard to ensure that all internal processes
    // that the worker holds are dropped when the worker is dropped.
    _drop_guard: DropGuard,
}

#[async_trait]
impl AssetWorker for Worker {
    type Opts = WorkerOpts;

    /// Returns the name identifier for the worker.
    fn name(&self) -> &'static str {
        WORKER_NAME
    }

    /// Builds and starts the `CoinbaseWorker`.
    ///
    /// This method creates a Coinbase WebSocket client, spawns asynchronous tasks
    /// to subscribe to asset updates, and returns the running [`Worker`] instance.
    async fn build<S: Store + 'static>(
        opts: Self::Opts,
        store: &S,
        ids: Vec<String>,
    ) -> Result<Self, AssetWorkerError> {
        let url = opts.url;
        let connector = Arc::new(WebSocketConnector::new(url));

        let worker_store = WorkerStore::new(store, WORKER_NAME);

        let token = CancellationToken::new();

        for (i, chunk) in ids
            .into_iter()
            .chunks(opts.max_subscription_per_connection)
            .into_iter()
            .enumerate()
        {
            let span = span!(
                Level::ERROR,
                "source",
                name = WORKER_NAME,
                connection_idx = i
            );
            let worker = format!("{WORKER_NAME}_{i}");
            let metrics = Metrics::new(WORKER_NAME, worker);
            tokio::spawn(
                start_listening(
                    token.child_token(),
                    connector.clone(),
                    worker_store.clone(),
                    chunk.collect(),
                    TIMEOUT,
                    metrics,
                )
                .instrument(span),
            );
        }

        Ok(Worker {
            _drop_guard: token.drop_guard(),
        })
    }
}
