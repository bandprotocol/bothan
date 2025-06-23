//! Types for Kraken WebSocket API ticker channel.
//!
//! This module defines types used for subscribing to and processing ticker data
//! from the Kraken WebSocket API, including subscription parameters, event triggers,
//! and ticker data responses.
//!
//! # Key Types
//!
//! - [`EventTrigger`] – Event types triggering ticker data updates.
//! - [`TickerRequestParameters`] – Parameters for subscribing to ticker updates.
//! - [`TickerResponse`] – Response containing ticker data.

use serde::{Deserialize, Serialize};

/// Represents the event trigger types available for ticker subscriptions.
///
/// `EventTrigger` determine the events that will initiate data updates.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EventTrigger {
    /// Best Bid and Offer (BBO) updates trigger events.
    #[serde(rename = "bbo")]
    BBO,

    /// Trades trigger events.
    #[serde(rename = "trades")]
    Trades,
}

/// Represents subscription parameters for the ticker channel.
///
/// `TickerRequestParameters` is used to specify subscription options, including the symbols, event triggers,
/// and whether a snapshot should be included.
///
/// # Examples
///
/// ```rust
/// use bothan_kraken::api::types::channel::ticker::{TickerRequestParameters, EventTrigger};
///
/// let params = TickerRequestParameters {
///     channel: "ticker".into(),
///     symbol: vec!["BTC/USD".into(), "ETH/USD".into()],
///     event_trigger: Some(EventTrigger::Trades),
///     snapshot: Some(true),
/// };
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TickerRequestParameters {
    /// Channel name (typically "ticker").
    pub channel: String,

    /// A list of currency pairs subscribe to (e.g., BTC/USD).
    pub symbol: Vec<String>,

    /// Optional event trigger type for updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_trigger: Option<EventTrigger>,

    /// Optional flag indicating whether a snapshot should be requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
}

/// Represents the result details of a successful ticker subscription.
///
/// This type is primarily informational, summarizing subscription outcomes.
pub struct TickerSubscriptionResult {
    /// Channel subscribed to ("ticker").
    pub channel: String,

    /// Indicates if a snapshot was requested during subscription.
    pub snapshot: bool,

    /// List of symbols successfully subscribed.
    pub symbol: Vec<String>,
}

/// Represents ticker data responses from the Kraken WebSocket API.
///
/// `TickerResponse` includes market information such as bid and ask prices, volumes,
/// and percentage changes.
///
/// # Examples
///
/// ```rust
/// use bothan_kraken::api::types::channel::ticker::TickerResponse;
/// use serde_json::json;
///
/// let ticker_json = json!({
///     "symbol": "BTC/USD",
///     "bid": 30000.0,
///     "bid_qty": 0.5,
///     "ask": 30001.0,
///     "ask_qty": 0.4,
///     "last": 30000.5,
///     "volume": 125.0,
///     "vwap": 29950.0,
///     "low": 29500.0,
///     "high": 30500.0,
///     "change": 500.0,
///     "change_pct": 1.69
/// });
///
/// let ticker: TickerResponse = serde_json::from_value(ticker_json).unwrap();
/// assert_eq!(ticker.symbol, "BTC/USD");
/// assert_eq!(ticker.bid, 30000.0);
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TickerResponse {
    /// Symbol identifier for the trading pair (e.g., "BTC/USD").
    pub symbol: String,

    /// Best bid price.
    pub bid: f64,

    /// Best bid quantity.
    pub bid_qty: f64,

    /// Best ask price.
    pub ask: f64,

    /// Best ask quantity.
    pub ask_qty: f64,

    /// Last traded price (only guaranteed if traded within the past 24 hours).
    pub last: f64,

    /// 24-hour traded volume (in base currency terms).
    pub volume: f64,

    /// 24-hour volume weighted average price.
    pub vwap: f64,

    /// 24-hour lowest trade price.
    pub low: f64,

    /// 24-hour highest trade price.
    pub high: f64,

    /// 24-hour price change (in quote currency).
    pub change: f64,

    /// 24-hour price change (in percentage points).
    pub change_pct: f64,
}
