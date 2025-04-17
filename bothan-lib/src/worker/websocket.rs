//! WebSocket-based asset information providers and streaming mechanisms.
//!
//! This module provides functionality for streaming asset information in real-time
//! from WebSocket APIs. It defines traits for WebSocket connectors and providers,
//! as well as a function for starting a listening loop.
//!
//! The module provides:
//!
//! - The [`AssetInfoProviderConnector`] trait for creating WebSocket connections
//! - The [`AssetInfoProvider`] trait for managing WebSocket subscriptions and data
//! - The [`Data`] enum for different types of received data
//! - The [`start_listening`] function which implements the WebSocket listener loop
//!
//! # WebSocket Strategy
//!
//! The WebSocket strategy follows these principles:
//!
//! 1. **Persistent Connections**: Maintains persistent WebSocket connections for real-time data
//! 2. **Automatic Reconnection**: Automatically reconnects with exponential backoff if the connection is lost
//! 3. **Subscription Management**: Provides a standard way to subscribe to asset updates
//! 4. **Error Resilience**: Handles connection failures and data errors gracefully
//!
//! When implementing new WebSocket-based asset providers, implement both the
//! [`AssetInfoProviderConnector`] and [`AssetInfoProvider`] traits, and use the
//! [`start_listening`] function to handle the connection lifecycle.

use std::fmt::Display;
use std::sync::Arc;
use std::time::Duration;

use tokio::select;
use tokio::time::{sleep, timeout};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};

use crate::store::{Store, WorkerStore};
use crate::types::AssetInfo;

/// Represents different types of data that can be received from a WebSocket connection.
///
/// This enum allows WebSocket providers to distinguish between different types of
/// messages they might receive, handling each appropriately.
///
/// # Variants
///
/// * `AssetInfo` - Contains asset information updates to be stored
/// * `Unused` - Represents data that is not relevant to asset information updates
pub enum Data {
    /// Asset information updates to be stored in the system.
    ///
    /// This variant contains a vector of [`AssetInfo`] structures that should be
    /// saved to the store.
    AssetInfo(Vec<AssetInfo>),

    /// Data that is not relevant to asset information updates.
    ///
    /// This variant is used for messages that should be acknowledged but don't
    /// contain asset information, such as heartbeats or subscription acknowledgments.
    Unused,
}

/// Trait for factory objects that can establish connections to WebSocket providers.
///
/// This trait separates the connection establishment logic from the provider itself,
/// allowing for cleaner error handling and reconnection strategies. Implementors
/// should handle the initial connection setup and return a provider that's ready
/// to subscribe to asset information.
///
/// # Examples
///
/// Implementing a connector for a cryptocurrency exchange WebSocket:
///
/// ```
/// use std::fmt;
/// use async_trait::async_trait;
/// use tokio_tungstenite::connect_async;
/// use futures::StreamExt;
/// use bothan_lib::worker::websocket::{AssetInfoProviderConnector, AssetInfoProvider, Data};
/// use thiserror::Error;
///
/// #[derive(Debug, Error)]
/// #[error("Connection Error: {0}")]
/// struct ConnectionError(String);
///
/// struct CryptoExchangeConnector {
///     ws_url: String,
/// }
///
/// impl CryptoExchangeConnector {
///     fn new(ws_url: String) -> Self {
///         Self { ws_url }
///     }
/// }
///
/// #[async_trait]
/// impl AssetInfoProviderConnector for CryptoExchangeConnector {
///     type Provider = CryptoExchangeProvider;
///     type Error = ConnectionError;
///
///     async fn connect(&self) -> Result<Self::Provider, Self::Error> {
///         let (ws_stream, _) = connect_async(&self.ws_url)
///             .await
///             .map_err(|e| ConnectionError(e.to_string()))?;
///
///         let (write, read) = ws_stream.split();
///
///         Ok(CryptoExchangeProvider {
///             write,
///             read,
///         })
///     }
/// }
///
/// // Provider implementation would be defined elsewhere
/// # struct CryptoExchangeProvider;
/// # #[async_trait::async_trait]
/// # impl AssetInfoProvider for CryptoExchangeProvider {
/// #     type SubscriptionError = String;
/// #     type ListeningError = String;
/// #     async fn subscribe(&mut self, _ids: &[String]) -> Result<(), Self::SubscriptionError> { Ok(()) }
/// #     async fn next(&mut self) -> Option<Result<Data, Self::ListeningError>> { None }
/// #     async fn try_close(mut self) {}
/// # }
/// ```
#[async_trait::async_trait]
pub trait AssetInfoProviderConnector: Send + Sync {
    /// The type of provider that this connector creates.
    ///
    /// This should be a type that implements the [`AssetInfoProvider`] trait.
    type Provider: AssetInfoProvider;

    /// The type returned in the event of a connection failure.
    ///
    /// This should be a custom error type that implements the Display trait
    /// and captures all possible error conditions during connection.
    type Error: Display;

    /// Establishes a connection to the WebSocket and returns a provider.
    ///
    /// This method should handle the initial WebSocket connection setup,
    /// including any authentication or handshaking required by the API.
    ///
    /// # Errors
    ///
    /// Returns a connector-specific error if the connection fails, such as when
    /// the API is unavailable, authentication fails, or the connection cannot be
    /// established for any other reason.
    async fn connect(&self) -> Result<Self::Provider, Self::Error>;
}

/// Trait for providers that can stream asset information from WebSocket APIs.
///
/// This trait defines the interface that WebSocket-based asset information providers
/// must implement. Providers are responsible for subscribing to asset updates,
/// receiving and parsing WebSocket messages, and converting them into [`Data`] structures.
///
/// # Examples
///
/// Implementing a provider for a cryptocurrency exchange WebSocket:
///
/// ```
/// use std::fmt;
/// use async_trait::async_trait;
/// use futures::{SinkExt, StreamExt};
/// use tokio_tungstenite::{tungstenite::Message, WebSocketStream};
/// use tokio::net::TcpStream;
/// use serde_json::{json, Value};
/// use rust_decimal::Decimal;
/// use bothan_lib::worker::websocket::{AssetInfoProvider, Data};
/// use bothan_lib::types::AssetInfo;
/// use thiserror::Error;
///
/// #[derive(Debug, Error)]
/// #[error("Subscription Error: {0}")]
/// struct SubscriptionError(String);
///
/// #[derive(Debug, Error)]
/// #[error("Listening Error: {0}")]
/// struct ListeningError(String);
///
/// struct CryptoExchangeProvider {
///     write: futures::stream::SplitSink<WebSocketStream<TcpStream>, Message>,
///     read: futures::stream::SplitStream<WebSocketStream<TcpStream>>,
/// }
///
/// #[async_trait]
/// impl AssetInfoProvider for CryptoExchangeProvider {
///     type SubscriptionError = SubscriptionError;
///     type ListeningError = ListeningError;
///
///     async fn subscribe(&mut self, ids: &[String]) -> Result<(), Self::SubscriptionError> {
///         // Create subscription message for each asset ID
///         for id in ids {
///             let subscribe_msg = json!({
///                 "type": "subscribe",
///                 "product_id": id,
///                 "channels": ["ticker"]
///             });
///
///             // Send subscription request
///             self.write.send(Message::Text(subscribe_msg.to_string()))
///                 .await
///                 .map_err(|e| SubscriptionError(e.to_string()))?;
///         }
///
///         Ok(())
///     }
///
///     async fn next(&mut self) -> Option<Result<Data, Self::ListeningError>> {
///         // Wait for the next message
///         let message = self.read.next().await?;
///
///         // Process the message
///         match message {
///             Ok(Message::Text(text)) => {
///                 // Parse JSON message
///                 let parsed: Value = match serde_json::from_str(&text) {
///                     Ok(data) => data,
///                     Err(e) => return Some(Err(ListeningError(e.to_string()))),
///                 };
///
///                 // Check if this is a ticker update
///                 if parsed["type"] == "ticker" {
///                     let id = match parsed["product_id"].as_str() {
///                         Some(id) => id.to_string(),
///                         None => return Some(Err(ListeningError("Missing product_id".to_string()))),
///                     };
///
///                     let price_str = match parsed["price"].as_str() {
///                         Some(price) => price,
///                         None => return Some(Err(ListeningError("Missing price".to_string()))),
///                     };
///
///                     let timestamp = match parsed["time"].as_str() {
///                         Some(time) => {
///                             // Parse ISO timestamp to Unix timestamp
///                             match chrono::DateTime::parse_from_rfc3339(time) {
///                                 Ok(dt) => dt.timestamp_millis(),
///                                 Err(e) => return Some(Err(ListeningError(e.to_string()))),
///                             }
///                         },
///                         None => return Some(Err(ListeningError("Missing time".to_string()))),
///                     };
///
///                     // Parse price string to Decimal
///                     let price = match price_str.parse::<Decimal>() {
///                         Ok(p) => p,
///                         Err(e) => return Some(Err(ListeningError(e.to_string()))),
///                     };
///
///                     // Create asset info
///                     let asset_info = AssetInfo::new(id, price, timestamp);
///                     Some(Ok(Data::AssetInfo(vec![asset_info])))
///                 } else {
///                     // Not a ticker update, just acknowledge
///                     Some(Ok(Data::Unused))
///                 }
///             },
///             Ok(Message::Ping(_)) => {
///                 // Automatically respond to ping with pong to keep connection alive
///                 // In a real implementation, we would send a pong response
///                 Some(Ok(Data::Unused))
///             },
///             Ok(_) => Some(Ok(Data::Unused)), // Other message types
///             Err(e) => Some(Err(ListeningError(e.to_string()))),
///         }
///     }
///
///     async fn try_close(mut self) {
///         // Send close message if possible
///         let _ = self.write.send(Message::Close(None)).await;
///     }
/// }
/// ```
#[async_trait::async_trait]
pub trait AssetInfoProvider: Send + Sync {
    /// The type returned in the event of a subscription failure.
    ///
    /// This should be a custom error type that implements the Display trait
    /// and captures all possible error conditions during subscription.
    type SubscriptionError: Display;

    /// The type returned in the event of a message reception failure.
    ///
    /// This should be a custom error type that implements the Display trait
    /// and captures all possible error conditions during message reception.
    type ListeningError: Display;

    /// Subscribes to asset updates for the specified asset IDs.
    ///
    /// This method should send subscription requests to the WebSocket API
    /// for each of the specified asset IDs, configuring the connection to
    /// receive updates for these assets.
    ///
    /// # Errors
    ///
    /// Returns a subscription-specific error if the operation fails, such as when
    /// the API rejects the subscription request or the request cannot be sent.
    async fn subscribe(&mut self, ids: &[String]) -> Result<(), Self::SubscriptionError>;

    /// Waits for and returns the next data update from the WebSocket.
    ///
    /// This method should wait for the next WebSocket message, parse it,
    /// and return the appropriate data structure. It returns None if the
    /// connection has been closed.
    ///
    /// # Returns
    ///
    /// * `Some(Ok(Data))` - If a message was successfully received and parsed
    /// * `Some(Err(ListeningError))` - If there was an error receiving or parsing the message
    /// * `None` - If the connection has been closed
    async fn next(&mut self) -> Option<Result<Data, Self::ListeningError>>;

    /// Attempts to gracefully close the WebSocket connection.
    ///
    /// This method should send a close frame to the WebSocket server
    /// and perform any necessary cleanup. It may fail silently if the
    /// connection is already closed.
    async fn try_close(mut self);
}

/// Starts listening for asset information from a WebSocket provider.
///
/// This function implements a listener loop that continuously receives asset
/// information from a WebSocket provider and stores it using the provided worker store.
/// The loop continues until the cancellation token is triggered.
///
/// # Features
///
/// * Maintains persistent WebSocket connections
/// * Automatically reconnects with exponential backoff if the connection is lost
/// * Monitors for timeouts to detect connection issues
/// * Handles errors gracefully by logging them and continuing
/// * Cancels listening gracefully when requested via the cancellation token
///
/// # Examples
///
/// Starting a WebSocket listener for cryptocurrency prices:
///
/// ```
/// use std::time::Duration;
/// use std::sync::Arc;
/// use tokio_util::sync::CancellationToken;
/// use bothan_lib::store::{Store, WorkerStore};
/// use bothan_lib::worker::websocket::{start_listening, AssetInfoProviderConnector};
///
/// async fn start_crypto_websocket<
///     S: Store,
///     C: AssetInfoProviderConnector + 'static
/// >(
///     store: S,
///     connector: C,
///     asset_ids: Vec<String>,
/// ) {
///     let cancellation_token = CancellationToken::new();
///     let worker_store = WorkerStore::new(&store, "crypto_prices_ws");
///     let connection_timeout = Duration::from_secs(30); // Reconnect if no message for 30 seconds
///
///     // Clone the token for later cancellation
///     let cancel_handle = cancellation_token.clone();
///
///     // Start listening in a separate task
///     tokio::spawn(async move {
///         start_listening(
///             cancellation_token,
///             Arc::new(connector),
///             worker_store,
///             asset_ids,
///             connection_timeout,
///         ).await;
///     });
///
///     // Later, when we want to stop listening:
///     // cancel_handle.cancel();
/// }
/// ```
#[tracing::instrument(skip(
    cancellation_token,
    provider_connector,
    store,
    ids,
    connection_timeout
))]
pub async fn start_listening<S, E1, E2, P, C>(
    cancellation_token: CancellationToken,
    provider_connector: Arc<C>,
    store: WorkerStore<S>,
    ids: Vec<String>,
    connection_timeout: Duration,
) where
    E1: Display,
    E2: Display,
    S: Store,
    P: AssetInfoProvider<SubscriptionError = E1, ListeningError = E2>,
    C: AssetInfoProviderConnector<Provider = P>,
{
    let mut connection = connect(provider_connector.as_ref(), &ids).await;
    loop {
        select! {
            _ = cancellation_token.cancelled() => break,
            result = timeout(connection_timeout, connection.next()) => {
                    match result {
                        // If timeout, we assume the connection has been dropped, and we attempt to reconnect
                        Err(_) | Ok(None) => {
                            let new_conn = connect(provider_connector.as_ref(), &ids).await;
                            connection = new_conn
                        }
                        Ok(Some(Ok(Data::AssetInfo(assets)))) => {
                            if let Err(e) = store.set_batch_asset_info(assets).await {
                                error!("failed to set asset info with error: {e}")
                            } else {
                                info!("asset info updated successfully");
                            }
                        }
                        Ok(Some(Ok(Data::Unused))) => debug!("received irrelevant data"),
                        Ok(Some(Err(e))) => error!("{}", e),
                    }
            }
        }
    }
}

/// Helper function to establish a connection with exponential backoff.
///
/// This function attempts to connect to the WebSocket provider and subscribe
/// to the specified asset IDs. If the connection or subscription fails, it will
/// retry with an exponential backoff strategy.
///
/// # Returns
///
/// A connected and subscribed provider ready to receive WebSocket messages
async fn connect<C, P, E1, E2>(connector: &C, ids: &[String]) -> P
where
    P: AssetInfoProvider<SubscriptionError = E1, ListeningError = E2>,
    C: AssetInfoProviderConnector<Provider = P>,
{
    let mut retry_count = 0;
    let mut backoff = Duration::from_secs(1);
    let max_backoff = Duration::from_secs(64);

    loop {
        if let Ok(mut provider) = connector.connect().await {
            if provider.subscribe(ids).await.is_ok() {
                return provider;
            }
        }

        retry_count += 1;
        if backoff < max_backoff {
            backoff *= 2;
        }

        error!("failed to reconnect. current attempt: {}", retry_count);
        sleep(backoff).await;
    }
}
