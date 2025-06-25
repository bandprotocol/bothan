//! Types for Coinbase WebSocket API subscription requests and responses.
//!
//! This module provides types for serializing and deserializing subscription requests,
//! channels, and error responses for the Coinbase WebSocket API.

use serde::{Deserialize, Serialize};

use crate::api::types::channels::Channel;

/// Represents the type of request for subscriptions.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestType {
    /// Subscribe to a channel.
    Subscribe,
    /// Unsubscribe from a channel.
    Unsubscribe,
}

/// Represents a subscription request.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Request {
    /// The type of request (subscribe or unsubscribe).
    #[serde(rename = "type")]
    pub type_: RequestType,
    /// The list of product IDs to be included in the request.
    pub product_ids: Vec<String>,
    /// The list of channels to be included in the request.
    pub channels: Vec<Channel>,
}

/// Represents a subscription channel.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionChannel {
    /// The name of the channel.
    pub name: String,
    /// The list of product IDs subscribed to this channel.
    pub product_ids: Vec<String>,
}

/// Represents the current subscriptions.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Subscriptions {
    /// The list of subscribed channels.
    pub channels: Vec<SubscriptionChannel>,
    /// The list of subscribed product IDs (optional).
    pub product_ids: Option<Vec<String>>,
    /// The list of subscribed account IDs (optional).
    pub account_ids: Option<Vec<String>>,
}

/// Represents an error response from the Coinbase WebSocket API.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    /// The error message describing what went wrong.
    pub message: String,
    /// The specific reason for the error.
    pub reason: String,
}
