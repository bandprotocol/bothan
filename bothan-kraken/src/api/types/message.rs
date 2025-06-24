//! Types for Kraken WebSocket public messages.
//!
//! This module provides types for constructing and parsing public messages used with the Kraken WebSocket API,
//! including subscription commands and their responses.
//!
//! # Key Types
//!
//! - [`Method`] – Enum representing available message methods.
//! - [`PublicMessage<T>`] – Generic structure representing WebSocket requests.
//! - [`PublicMessageResponse`] – Structure representing WebSocket responses from Kraken.

use serde::{Deserialize, Serialize};

/// Represents available methods for public messages to the Kraken WebSocket API.
///
/// This enum defines operations such as subscribing and unsubscribing from Kraken data channels.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Method {
    /// Sends a ping to keep the WebSocket connection alive.
    Ping,

    /// Subscribes to a channel to receive data updates.
    Subscribe,

    /// Unsubscribes from a channel to stop receiving data updates.
    Unsubscribe,
}

/// Represents a public request message to the Kraken WebSocket API.
///
/// This generic struct defines the format for sending subscription-related requests.
/// It includes the method, optional parameters, and a request ID.
///
/// # Examples
///
/// ```rust
/// use bothan_kraken::api::types::message::{PublicMessage, Method};
/// use bothan_kraken::api::types::channel::ticker::TickerRequestParameters;
///
/// let ticker_params = TickerRequestParameters {
///     channel: "ticker".into(),
///     symbol: vec!["BTC/USD".into()],
///     event_trigger: None,
///     snapshot: Some(true),
/// };
///
/// let message = PublicMessage {
///     method: Method::Subscribe,
///     params: Some(ticker_params),
///     req_id: Some(42),
/// };
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicMessage<T> {
    /// The method of the request (e.g., subscribe, unsubscribe, ping).
    pub method: Method,

    /// Optional parameters specific to the method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<T>,

    /// Optional request identifier for correlating responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_id: Option<usize>,
}

/// Represents a response message received from the Kraken WebSocket API.
///
/// This struct contains metadata indicating the result of public WebSocket requests,
/// including subscription status and any associated errors.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicMessageResponse {
    /// Error message
    pub error: Option<String>,

    /// Method associated with the response (e.g., subscribe, unsubscribe).
    pub method: String,

    /// Optional client originated request identifier sent as acknowledgment in the response.
    pub req_id: Option<usize>,

    /// Indicates whether the operation succeeded.
    pub success: bool,

    /// Timestamp when the subscription was received on the wire, immediately before data parsing (RFC3339 format, e.g., `2022-12-25T09:30:59.123456Z`).
    pub time_in: String,

    /// Timestamp when the acknowledgment was sent on the wire, immediately before data transmission (RFC3339 format, e.g., `2022-12-25T09:30:59.123456Z`).
    pub time_out: String,
}
