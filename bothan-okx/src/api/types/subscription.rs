//! Types for OKX WebSocket subscription management.
//!
//! This module provides types for handling WebSocket subscription requests and responses
//! from the OKX API. It includes structures for subscribing to and unsubscribing from
//! various data channels, as well as processing subscription confirmations and errors.
//!
//! # Key Types
//!
//! - [`Request<T>`] - Subscription request structure
//! - [`Response<T>`] - Subscription response structure
//! - [`Operation`] - Available subscription operations

use serde::{Deserialize, Serialize};

/// Represents a request to the OKX WebSocket API.
///
/// This generic struct defines the structure for subscription and unsubscription
/// requests sent to the OKX WebSocket API. It includes the operation type and
/// optional arguments specific to the subscription.
///
/// # Examples
///
/// ```rust
/// use bothan_okx::api::types::subscription::{Request, Operation};
/// use bothan_okx::api::types::ticker::Request as TickerRequest;
///
/// let ticker_req = TickerRequest {
///     channel: "tickers".to_string(),
///     inst_type: Some(bothan_okx::api::types::ticker::InstrumentType::Spot),
///     inst_family: None,
///     inst_id: Some("BTC-USDT".to_string()),
/// };
///
/// let request = Request {
///     op: Operation::Subscribe,
///     args: Some(vec![ticker_req]),
/// };
/// ```
///
/// # OKX API Request Example
///
/// ```json
/// {
///   "op": "subscribe",
///   "args": [
///     {
///       "channel": "tickers",
///       "instType": "SPOT",
///       "instId": "BTC-USDT"
///     }
///   ]
/// }
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Request<T> {
    /// The operation type (subscribe or unsubscribe).
    pub op: Operation,

    /// The arguments for the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<T>>,
}

/// Represents the available subscription operations.
///
/// This enum defines the two main operations that can be performed on OKX WebSocket
/// channels: subscribing to receive data and unsubscribing to stop receiving data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Operation {
    /// Subscribe to a channel to receive data updates.
    Subscribe,

    /// Unsubscribe from a channel to stop receiving data updates.
    Unsubscribe,
}

/// Represents a subscription response from the OKX WebSocket API.
///
/// This generic struct contains the response information for subscription and
/// unsubscription operations, including success/failure status, error codes,
/// and connection metadata.
///
/// # Examples
///
/// ```rust
/// use bothan_okx::api::types::subscription::Response;
/// use serde_json::json;
///
/// let response_json = json!({
///     "event": "subscribe",
///     "arg": {
///         "channel": "tickers",
///         "instId": "BTC-USDT"
///     },
///     "code": "0",
///     "msg": "",
///     "connId": "1234567890"
/// });
///
/// let response: Response<serde_json::Value> = serde_json::from_value(response_json).unwrap();
/// assert_eq!(response.event, "subscribe");
/// assert_eq!(response.conn_id, "1234567890");
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    /// The event name (e.g., "subscribe", "unsubscribe").
    pub event: String,

    /// The argument related to the response.
    pub arg: Option<T>,

    /// The response code, if any (0 indicates success).
    pub code: Option<String>,

    /// The response message, if any (error description for failures).
    pub msg: Option<String>,

    /// The connection ID for the WebSocket connection.
    pub conn_id: String,
}
