//! Bybit worker implementation.
//!
//! This module provides an implementation of the [`AssetWorker`] trait for interacting with
//! the Bybit WebSocket API. It defines the [`Worker`], which is responsible for subscribing
//! to asset updates via WebSocket connections and storing the data into a shared [`WorkerStore`].
//!
//! The worker is configurable via [`WorkerOpts`] and uses [`WebSocketConnector`] to establish
//! WebSocket connections to Bybit endpoints.
//!
//! # The module provides:
//!
//! - Subscription to asset updates via WebSocket connections in asynchronous tasks
//! - Ensures graceful cancellation by using a CancellationToken to signal shutdown and a DropGuard
//!   to automatically clean up resources when the worker is dropped
//! - Metrics collection for observability
//! - Configurable via endpoint URL
//!
//! # Examples
//!
//! ```ignore
//! use bothan_bybit::worker::{Worker, WorkerOpts};
//! use bothan_lib::worker::AssetWorker;
//! use bothan_lib::store::Store;
//!
//! #[tokio::test]
//! async fn test<T: Store>(store: T) {
//!     let opts = WorkerOpts::default();
//!     let ids = vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()];
//!
//!     let worker = Worker::build(opts, &store, ids).await.unwrap();
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
use tokio_util::sync::{CancellationToken, DropGuard};
use tracing::{Instrument, Level, span};

use crate::WorkerOpts;
use crate::api::websocket::WebSocketConnector;

pub mod opts;

const WORKER_NAME: &str = "bybit";
const TIMEOUT: Duration = Duration::from_secs(600);

/// Asset worker for subscribing to asset updates via the Bybit WebSocket API.
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

    /// Builds and starts the `BybitWorker`.
    ///
    /// This method creates a Bybit WebSocket client, spawns asynchronous tasks
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

        let metrics = Metrics::new(WORKER_NAME, WORKER_NAME.to_string());

        let span = span!(Level::ERROR, "source", name = WORKER_NAME);
        tokio::spawn(
            start_listening(
                token.child_token(),
                connector.clone(),
                worker_store.clone(),
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
