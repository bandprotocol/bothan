//! Types for OKX ticker data interaction.
//!
//! This module provides types for deserializing ticker data from the OKX WebSocket API,
//! including request structures and ticker information. The module supports the OKX v5
//! API format for real-time market data streaming.
//!
//! # Key Types
//!
//! - [`Request`] - Ticker subscription request structure
//! - [`Ticker`] - Ticker data received from the API
//! - [`InstrumentType`] - Supported instrument types

use serde::{Deserialize, Serialize};

/// Represents the arguments for a ticker subscription request.
///
/// This struct defines the parameters needed to subscribe to ticker updates
/// from the OKX WebSocket API. It follows the OKX v5 API specification for
/// channel subscriptions.
///
/// # Examples
///
/// ```rust
/// use bothan_okx::api::types::ticker::{Request, InstrumentType};
///
/// let request = Request {
///     channel: "tickers".to_string(),
///     inst_type: Some(InstrumentType::Spot),
///     inst_family: None,
///     inst_id: Some("BTC-USDT".to_string()),
/// };
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// The name of the channel to subscribe to (e.g., "tickers").
    pub channel: String,

    /// The type of instrument (e.g., Spot).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<InstrumentType>,

    /// The instrument family (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,

    /// The instrument ID (e.g., "BTC-USDT").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
}

/// Represents ticker data received from the OKX WebSocket API.
///
/// This struct contains comprehensive market data information including price,
/// volume, and 24-hour statistics for a specific trading instrument.
///
/// # Examples
///
/// ```rust
/// use bothan_okx::api::types::ticker::Ticker;
/// use serde_json::json;
///
/// let json_data = json!({
///     "instType": "SPOT",
///     "instId": "BTC-USDT",
///     "last": "50000",
///     "lastSz": "0.1",
///     "askPx": "50001",
///     "askSz": "1.5",
///     "bidPx": "49999",
///     "bidSz": "2.0",
///     "open24h": "49000",
///     "high24h": "51000",
///     "low24h": "48000",
///     "volCcy24h": "1000000",
///     "vol24h": "20",
///     "sodUtc0": "49000",
///     "sodUtc8": "49000",
///     "ts": "1640995200000"
/// });
///
/// let ticker: Ticker = serde_json::from_value(json_data).unwrap();
///
/// assert_eq!(ticker.inst_id, "BTC-USDT");
/// assert_eq!(ticker.last, "50000");
/// assert_eq!(ticker.ask_px, "50001");
/// assert_eq!(ticker.bid_px, "49999");
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    /// The instrument type (e.g., "SPOT", "FUTURES").
    pub inst_type: String,

    /// The instrument ID (e.g., "BTC-USDT").
    pub inst_id: String,

    /// The last traded price.
    pub last: String,

    /// The size of the last trade.
    pub last_sz: String,

    /// The current best ask price.
    pub ask_px: String,

    /// The current best ask size.
    pub ask_sz: String,

    /// The current best bid price.
    pub bid_px: String,

    /// The current best bid size.
    pub bid_sz: String,

    /// The opening price from 24 hours ago.
    pub open_24h: String,

    /// The highest price in the last 24 hours.
    pub high_24h: String,

    /// The lowest price in the last 24 hours.
    pub low_24h: String,

    /// The 24-hour volume in quote currency.
    pub vol_ccy_24h: String,

    /// The 24-hour volume in base currency.
    pub vol_24h: String,

    /// The start of day price in UTC+0.
    pub sod_utc0: String,

    /// The start of day price in UTC+8.
    pub sod_utc8: String,

    /// The timestamp of the data in milliseconds.
    pub ts: String,
}

/// Represents the supported instrument types for OKX.
///
/// This enum defines the different types of instruments that can be parsed
/// from the OKX platform. Currently, only spot is supported.
///
/// # Examples
///
/// ```rust
/// use bothan_okx::api::types::ticker::InstrumentType;
///
/// let inst_type = InstrumentType::Spot;
/// assert_eq!(inst_type, InstrumentType::Spot);
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum InstrumentType {
    /// Spot instruments.
    Spot,
}
