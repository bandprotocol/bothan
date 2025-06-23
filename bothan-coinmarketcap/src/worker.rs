//! CoinMarketCap worker implementation.
//!
//! This module provides an implementation of the [`AssetWorker`] trait for interacting with
//! the CoinMarketCap REST API. It defines the [`Worker`], which is responsible for periodically
//! polling [`AssetInfo`](bothan_lib::types::AssetInfo) from CoinMarketCap REST API and storing it to a shared [`WorkerStore`].
//!
//! The worker is configurable via [`WorkerOpts`] and uses [`RestApiBuilder`] to construct
//! the API client.
//!
//! # The module provides:
//! - Polling of [`AssetInfo`](bothan_lib::types::AssetInfo) periodically in asynchronous task
//! - Ensures graceful cancellation by using a CancellationToken to signal shutdown and a DropGuard
//!   to automatically clean up resources when the worker is dropped
//! - Metrics collection for observability
//! - Configurable via API key, polling interval, and endpoint URL
//!
//! # Examples
//!
//! ```rust
//! use bothan_coinmarketcap::worker::{Worker, WorkerOpts};
//! use bothan_lib::worker::AssetWorker;
//! use bothan_lib::store::Store;
//!
//! #[tokio::test]
//! async fn test<T: Store>(store: T) {
//!     let opts = WorkerOpts::default();
//!     let ids = vec!["1".to_string(), "1027".to_string()];
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
use tracing::instrument::Instrument;
use tracing::{Level, span};

pub use crate::WorkerOpts;
pub use crate::api::RestApiBuilder;

pub mod error;
pub mod opts;
pub mod types;

const WORKER_NAME: &str = "coinmarketcap";

/// Asset worker for fetching data from the CoinMarketCap REST API.
///
/// The `Worker` manages asynchronous polling for [`AssetInfo`](bothan_lib::types::AssetInfo)
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
    fn name(&self) -> &'static str {
        WORKER_NAME
    }

    /// Builds and starts the `CoinMarketCapWorker`.
    ///
    /// This method creates a CoinMarketCap REST API client, spawns an asynchronous polling task
    /// to periodically fetch asset data, and returns the running [`Worker`] instance.
    ///
    /// # Errors
    ///
    /// Returns an [`AssetWorkerError`](bothan_lib::worker::error::AssetWorkerError) if:
    /// - The API client fails to build due to invalid configuration
    async fn build<S: Store + 'static>(
        opts: Self::Opts,
        store: &S,
        ids: Vec<String>,
    ) -> Result<Self, AssetWorkerError> {
        let api = RestApiBuilder::new(opts.url, opts.api_key).build()?;
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
