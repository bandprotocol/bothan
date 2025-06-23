//! OKX worker implementation.
//!
//! This module provides an implementation of the [`AssetWorker`] trait for interacting with
//! the OKX WebSocket API. It defines the [`Worker`], which is responsible for subscribing
//! to asset updates via WebSocket connections and storing the data into a shared [`WorkerStore`].
//!
//! The worker is configurable via [`WorkerOpts`] and uses [`WebSocketConnector`] to establish
//! WebSocket connections to OKX endpoints.
//!
//! # The module provides:
//!
//! - Subscription to asset updates via WebSocket connections in asynchronous tasks
//! - Ensures graceful cancellation by using a CancellationToken to signal shutdown and a DropGuard
//!   to automatically clean up resources when the worker is dropped
//! - Metrics collection for observability
//! - Configurable via endpoint URL

use std::sync::Arc;
use std::time::Duration;

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

const WORKER_NAME: &str = "okx";
const TIMEOUT: Duration = Duration::from_secs(60);

/// Asset worker for subscribing to asset updates via the OKX WebSocket API.
///
/// The `Worker` manages asynchronous WebSocket connections for asset updates
/// and ensures resources are properly cleaned up when dropped.
pub struct Worker {
    // We keep this DropGuard to ensure that all internal processes
    // that the worker holds are dropped when the worker is dropped.
    _drop_guard: DropGuard,
}

#[async_trait::async_trait]
impl AssetWorker for Worker {
    type Opts = WorkerOpts;

    /// Returns the name identifier for the worker.
    ///
    /// This method provides a unique identifier for the OKX worker,
    /// which is used for metrics collection and logging.
    ///
    /// # Returns
    ///
    /// A static string slice containing the worker name "okx".
    fn name(&self) -> &'static str {
        WORKER_NAME
    }

    /// Builds and starts the `OKXWorker`.
    ///
    /// This method creates an OKX WebSocket client, spawns an asynchronous task
    /// to subscribe to asset updates, and returns the running [`Worker`] instance.
    ///
    /// # Parameters
    ///
    /// - `opts`: Configuration options for the worker, including URL
    /// - `store`: The store instance for persisting asset data
    /// - `ids`: A vector of asset identifiers to monitor
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `Worker` instance on success,
    /// or an `AssetWorkerError` if the worker cannot be built.
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
