//! Types for Binance WebSocket API interaction.
//!
//! This module provides types for deserializing events and responses from the Binance WebSocket API,
//! including success responses, errors, stream events, and specific event data like mini ticker updates.

use serde::{Deserialize, Serialize};

/// Represents the different types of events that can be received from the Binance WebSocket API.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Event {
    /// Represents a successful event response with an optional result and an identifier.
    Success(SuccessEvent),
    /// Represents an error event with an error code, message, and identifier.
    Error(ErrorEvent),
    /// Represents a stream event containing the stream name and associated data.
    Stream(StreamEvent),
    /// Represents a ping message from the WebSocket API.
    Ping,
}

/// Represents a successful event response from the Binance WebSocket API.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuccessEvent {
    /// The optional result of the event, as some events may not return a result.
    pub result: Option<String>,
    /// The identifier for the event.
    pub id: i64,
}

/// Represents an error event from the Binance WebSocket API.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorEvent {
    /// The error code indicating the type of error that occurred.
    pub code: i16,
    /// A human-readable message describing the error.
    pub msg: String,
    /// The identifier for the event associated with the error.
    pub id: i64,
}

/// Represents a stream event received from the Binance WebSocket API.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreamEvent {
    /// The name of the stream event.
    pub stream: String,
    /// The data associated with the stream event from the Binance WebSocket API.
    pub data: StreamEventData,
}

/// Represents the data associated with a stream event from the Binance WebSocket API.
/// The `StreamEventData` enum can represent different types of stream events,
/// such as a mini ticker event. Each variant of the enum corresponds to a specific type of event,
/// allowing for flexible handling of various event types.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "e")]
pub enum StreamEventData {
    /// The `MiniTickerInfo` struct is used to deserialize the data for this event type.
    #[serde(rename = "24hrMiniTicker")]
    MiniTicker(MiniTickerInfo),
}

/// Represents price information retrieved from the Binance WebSocket API.
///
/// `MiniTickerInfo` struct contains fields matching those returned by the Binance WebSocket API
/// for the 24hr mini ticker event. It serves as an interface for JSON deserialization of event data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MiniTickerInfo {
    /// The unix timestamp of the event in milliseconds.
    #[serde(rename = "E")]
    pub event_time: i64,

    /// The trading pair symbol (e.g., "BTCUSDT").
    #[serde(rename = "s")]
    pub symbol: String,

    /// The last price of the trading pair.
    #[serde(rename = "c")]
    pub close_price: String,

    /// The opening price of the trading pair.
    #[serde(rename = "o")]
    pub open_price: String,

    /// The highest price of the trading pair during the event.
    #[serde(rename = "h")]
    pub high_price: String,

    /// The lowest price of the trading pair during the event.
    #[serde(rename = "l")]
    pub low_price: String,

    /// Total traded base asset volume
    #[serde(rename = "v")]
    pub base_volume: String,

    /// Total traded quote asset volume
    #[serde(rename = "q")]
    pub quote_volume: String,
}
