//! Bitfinex worker implementation.
//!
//! This module provides an implementation of the [`AssetWorker`] trait for interacting with
//! the Bitfinex REST API. It defines the [`Worker`], which is responsible for polling
//! asset updates via REST API calls and storing the data into a shared [`WorkerStore`].
//!
//! The worker is configurable via [`WorkerOpts`] and uses [`RestApiBuilder`] to create
//! REST API clients for Bitfinex endpoints.
//!
//! # The module provides:
//!
//! - Periodic polling of asset updates via REST API calls in asynchronous tasks
//! - Ensures graceful cancellation by using a CancellationToken to signal shutdown and a DropGuard
//!   to automatically clean up resources when the worker is dropped
//! - Metrics collection for observability
//! - Configurable via endpoint URL and update interval
//!
//! # Examples
//!
//! ```rust
//! use bothan_bitfinex::worker::Worker;
//! use bothan_bitfinex::worker::opts::WorkerOpts;
//! use bothan_lib::worker::AssetWorker;
//! use bothan_lib::store::Store;
//!
//! #[tokio::test]
//! async fn test<T: Store>(store: T) {
//!     let opts = WorkerOpts::default();
//!     let ids = vec!["tBTCUSD".to_string(), "tETHUSD".to_string()];
//!
//!     let worker = Worker::build(opts, &store, ids).await?;
//! }
//! ```

use bothan_lib::metrics::rest::Metrics;
use bothan_lib::store::{Store, WorkerStore};
use bothan_lib::worker::AssetWorker;
use bothan_lib::worker::error::AssetWorkerError;
use bothan_lib::worker::rest::start_polling;
use tokio_util::sync::{CancellationToken, DropGuard};
use tracing::{Instrument, Level, span};

use crate::api::builder::RestApiBuilder;
use crate::worker::opts::WorkerOpts;

const WORKER_NAME: &str = "bitfinex";

pub mod opts;

/// Asset worker for polling asset updates via the Bitfinex REST API.
///
/// The `Worker` manages asynchronous REST API polling for asset updates
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
    /// This method provides a unique identifier for the Bitfinex worker,
    /// which is used for metrics collection and logging.
    ///
    /// # Returns
    ///
    /// A static string slice containing the worker name "bitfinex".
    fn name(&self) -> &'static str {
        WORKER_NAME
    }

    /// Builds and starts the `BitfinexWorker`.
    ///
    /// This method creates a Bitfinex REST API client, spawns an asynchronous task
    /// to poll for asset updates at the configured interval, and returns the running [`Worker`] instance.
    ///
    /// # Parameters
    ///
    /// - `opts`: Configuration options for the worker, including URL and update interval
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
        let api = RestApiBuilder::new(&opts.url).build()?;
        let worker_store = WorkerStore::new(store, WORKER_NAME);

        let token = CancellationToken::new();

        let metrics = Metrics::new(WORKER_NAME);

        let span = span!(Level::ERROR, "source", name = WORKER_NAME);
        tokio::spawn(
            start_polling(
                token.child_token(),
                opts.update_interval,
                api,
                worker_store,
                ids,
                metrics,
            )
            .instrument(span),
        );

        Ok(Worker {
            _drop_guard: token.drop_guard(),
        })
    }
}
