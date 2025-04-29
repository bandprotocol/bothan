//! REST-based asset information providers and polling mechanisms.
//!
//! This module provides functionality for polling asset information from REST APIs.
//! It defines a trait for asset information providers and a function for starting
//! a polling loop that periodically fetches and stores asset information.
//!
//! The module provides:
//!
//! - The [`AssetInfoProvider`] trait which defines the interface for REST-based asset info providers
//! - The [`start_polling`] function which implements the polling loop mechanism
//!
//! # Polling Strategy
//!
//! The polling strategy follows these principles:
//!
//! 1. **Regular Intervals**: Asset information is polled at regular intervals
//! 2. **Timeout Protection**: Requests that take too long are cancelled to prevent blocking
//! 3. **Error Handling**: Errors during polling are logged but don't stop the polling process
//! 4. **Graceful Cancellation**: Polling can be gracefully stopped using a cancellation token
//!
//! When implementing new REST-based asset providers, implement the [`AssetInfoProvider`] trait
//! and use the [`start_polling`] function to handle the polling lifecycle.

use std::fmt::Display;
use std::time::Duration;

use tokio::select;
use tokio::time::{interval, timeout};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};

use crate::store::{Store, WorkerStore};
use crate::types::AssetInfo;

/// Trait for providers that can fetch asset information from REST APIs.
///
/// This trait defines the interface that REST-based asset information providers
/// must implement. Providers are responsible for making HTTP requests to fetch
/// asset data and converting the responses into [`AssetInfo`] structures.
#[async_trait::async_trait]
pub trait AssetInfoProvider: Send + Sync {
    /// The type returned in the event of an operation failure.
    ///
    /// This should be a custom error type that implements the Display trait
    /// and captures all possible error conditions specific to the API.
    type Error: Display;

    /// Fetches asset information for the specified asset IDs.
    ///
    /// This method should make HTTP requests to the appropriate endpoints,
    /// parse the responses, and convert the data into a vector of [`AssetInfo`] structures.
    ///
    /// # Errors
    ///
    /// Returns a provider-specific error if the operation fails, such as when
    /// the API is unavailable, returns an error response, or the response cannot
    /// be parsed correctly.
    async fn get_asset_info(&self, ids: &[String]) -> Result<Vec<AssetInfo>, Self::Error>;
}

/// Starts polling asset information from a provider at the specified update interval.
///
/// This function implements a polling loop that periodically fetches asset information
/// from the provider and stores it using the provided worker store. The loop continues
/// until the cancellation token is triggered.
///
/// # Features
///
/// * Polls at regular intervals specified by `update_interval`
/// * Times out requests that take too long (based on the update interval)
/// * Handles errors gracefully by logging them and continuing
/// * Cancels polling gracefully when requested via the cancellation token
#[tracing::instrument(skip(cancellation_token, provider, store, ids))]
pub async fn start_polling<S: Store, E: Display, P: AssetInfoProvider<Error = E>>(
    cancellation_token: CancellationToken,
    update_interval: Duration,
    provider: P,
    store: WorkerStore<S>,
    ids: Vec<String>,
) {
    if ids.is_empty() {
        debug!("no ids to poll");
        return;
    }
    let mut interval = interval(update_interval);

    loop {
        select! {
            _ = cancellation_token.cancelled() => {
                info!("polling: cancelled");
                break
            },
            _ = interval.tick() => {
                info!("polling");
                match timeout(interval.period(), provider.get_asset_info(&ids)).await {
                    Ok(Ok(asset_info)) => {
                        if let Err(e) = store.set_batch_asset_info(asset_info).await {
                            error!("failed to store asset info with error: {e}");
                        } else {
                            info!("asset info updated successfully");
                        }
                    }
                    Ok(Err(e)) => {
                        error!("failed to poll asset info with error: {e}");
                    }
                    Err(_) => {
                        error!("updating interval exceeded timeout");
                    }
                }
            }
        }
    }
}
