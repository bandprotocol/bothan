use serde::{Deserialize, Serialize};

/// Represents the event trigger type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EventTrigger {
    #[serde(rename = "bbo")]
    BBO,
    #[serde(rename = "trades")]
    Trades,
}

/// Parameters for a ticker request.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TickerRequestParameters {
    /// The channel name.
    pub channel: String,
    /// The symbols to subscribe to.
    pub symbol: Vec<String>,
    /// The event trigger type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_trigger: Option<EventTrigger>,
    /// Whether to request a snapshot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
}

/// Result of a ticker subscription.
pub struct TickerSubscriptionResult {
    /// The channel name.
    pub channel: String,
    /// Whether a snapshot was requested.
    pub snapshot: bool,
    /// The subscribed symbols.
    pub symbol: Vec<String>,
}

/// Response from a ticker request.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TickerResponse {
    /// The symbol of the ticker.
    pub symbol: String,
    /// The bid price.
    pub bid: f64,
    /// The bid quantity.
    pub bid_qty: f64,
    /// The ask price.
    pub ask: f64,
    /// The ask quantity.
    pub ask_qty: f64,
    /// The last traded price.
    pub last: f64,
    /// The volume of trades.
    pub volume: f64,
    /// The volume-weighted average price.
    pub vwap: f64,
    /// The lowest price.
    pub low: f64,
    /// The highest price.
    pub high: f64,
    /// The price change.
    pub change: f64,
    /// The percentage change in price.
    pub change_pct: f64,
}
