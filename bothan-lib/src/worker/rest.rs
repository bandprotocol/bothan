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
///
/// # Examples
///
/// Implementing a provider for a cryptocurrency price API:
///
/// ```
/// use std::fmt;
/// use async_trait::async_trait;
/// use reqwest::Client;
/// use serde::Deserialize;
/// use rust_decimal::Decimal;
/// use bothan_lib::worker::rest::AssetInfoProvider;
/// use bothan_lib::types::AssetInfo;
/// use thiserror::Error;
///
/// #[derive(Debug, Error)]
/// #[error("API Error: {0}")]
/// struct ApiError(String);
///
/// struct CryptoApiProvider {
///     client: Client,
///     base_url: String,
/// }
///
/// impl CryptoApiProvider {
///     fn new(base_url: String) -> Self {
///         Self {
///             client: Client::new(),
///             base_url,
///         }
///     }
/// }
///
/// #[derive(Deserialize)]
/// struct PriceResponse {
///     price: String,
///     timestamp: i64,
/// }
///
/// #[async_trait]
/// impl AssetInfoProvider for CryptoApiProvider {
///     type Error = ApiError;
///
///     async fn get_asset_info(&self, ids: &[String]) -> Result<Vec<AssetInfo>, Self::Error> {
///         let mut results = Vec::new();
///
///         for id in ids {
///             let url = format!("{}/price/{}", self.base_url, id);
///             let response = self.client.get(&url)
///                 .send()
///                 .await
///                 .map_err(|e| ApiError(e.to_string()))?;
///
///             let data: PriceResponse = response.json()
///                 .await
///                 .map_err(|e| ApiError(e.to_string()))?;
///
///             // Parse the price string into a Decimal
///             let price = data.price.parse::<Decimal>()
///                 .map_err(|e| ApiError(e.to_string()))?;
///
///             results.push(AssetInfo::new(
///                 id.clone(),
///                 price,
///                 data.timestamp,
///             ));
///         }
///
///         Ok(results)
///     }
/// }
/// ```
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
///
/// # Examples
///
/// Starting a polling loop for cryptocurrency prices:
///
/// ```
/// use std::time::Duration;
/// use tokio_util::sync::CancellationToken;
/// use bothan_lib::store::{Store, WorkerStore};
/// use bothan_lib::worker::rest::{start_polling, AssetInfoProvider};
///
/// async fn start_crypto_polling<S: Store, P: AssetInfoProvider>(
///     store: S,
///     provider: P,
///     asset_ids: Vec<String>,
/// ) {
///     let cancellation_token = CancellationToken::new();
///     let worker_store = WorkerStore::new(&store, "crypto_prices");
///     let update_interval = Duration::from_secs(60); // Poll every minute
///
///     // Clone the token for later cancellation
///     let cancel_handle = cancellation_token.clone();
///
///     // Start polling in a separate task
///     tokio::spawn(async move {
///         start_polling(
///             cancellation_token,
///             update_interval,
///             provider,
///             worker_store,
///             asset_ids,
///         ).await;
///     });
///
///     // Later, when we want to stop polling:
///     // cancel_handle.cancel();
/// }
/// ```
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
