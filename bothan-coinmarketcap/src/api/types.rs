//! Data types for interacting with the CoinMarketCap REST API.
//!
//! This module provides types for deserializing responses from the CoinMarketCap REST API.
//!
use serde::{Deserialize, Serialize};

/// The base URL for the CoinMarketCap Pro API.
/// Used for accessing endpoints that require authentication and provide market data.
pub(crate) const DEFAULT_URL: &str = "https://pro-api.coinmarketcap.com";

/// Represents the status part of a CoinMarketCap API response.
///
/// `Status` contains metadata about the API response, such as error codes and timestamps.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Status {
    /// Current timestamp (ISO 8601) on the server.
    pub timestamp: String,
    /// An internal error code for the current error.
    /// If a unique platform error code is not available the HTTP status code is returned.
    pub error_code: u64,
    /// An error message to go along with the error code.
    pub error_message: Option<String>,
    /// Number of milliseconds taken to generate this response.
    pub elapsed: u64,
    /// Number of API call credits that were used for this call.
    pub credit_count: u64,
    /// Optional notice about API key information.
    pub notice: Option<String>,
}

/// Generic API response wrapper for CoinMarketCap.
///
/// `Response` wraps the data and status fields returned by the API.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Response<T> {
    /// The actual data returned by the API.
    pub data: T,
    /// The status metadata for the response.
    pub status: Status,
}

/// Represents price and market data for a single asset in USD.
///
/// `PriceQuote` contains fields matching those returned by the [CoinMarketCap latest quotes endpoint].
///
/// [CoinMarketCap latest quotes endpoint]: https://coinmarketcap.com/api/documentation/v1/#operation/getV2CryptocurrencyQuotesLatest
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PriceQuote {
    /// Price in the specified currency.
    pub price: Option<f64>,
    /// Rolling 24 hour adjusted volume in the specified currency.
    pub volume_24h: f64,
    /// 24 hour change in the specified currencies volume.
    pub volume_change_24h: f64,
    /// 1 hour change in the specified currency.
    pub percent_change_1h: f64,
    /// 24 hour change in the specified currency.
    pub percent_change_24h: f64,
    /// 7-day percent price change.
    pub percent_change_7d: f64,
    /// 30-day percent price change.
    pub percent_change_30d: f64,
    /// Market cap in the specified currency.
    pub market_cap: Option<f64>,
    /// Market cap dominance in the specified currency.
    pub market_cap_dominance: f64,
    /// Fully diluted market cap in the specified currency.
    pub fully_diluted_market_cap: f64,
    /// Timestamp (ISO 8601) of when the conversion currency's current value was referenced.
    pub last_updated: String,
}

/// Wrapper for price quotes in different currencies (currently only USD is supported).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PriceQuotes {
    /// A market quote in the currency conversion option.
    #[serde(rename = "USD")]
    pub usd: PriceQuote,
}

/// Represents a single asset quote retrieved from CoinMarketCap REST API.
///
/// `Quote` contains fields matching those returned by the [CoinMarketCap latest quotes endpoint].
///  It serves as an interface for JSON deserialization of API responses.
///
/// **Note:** This struct does **not** include all fields returned by the CoinMarketCap REST API.
/// Specifically, fields like `is_active` and other additional data provided by certain endpoints
/// are not represented here.
///
/// [CoinMarketCap latest quotes endpoint]: https://coinmarketcap.com/api/documentation/v1/#operation/getV2CryptocurrencyQuotesLatest
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Quote {
    /// The unique CoinMarketCap ID for this cryptocurrency.
    pub id: u64,
    /// The name of this cryptocurrency.
    pub name: String,
    /// The ticker symbol for this cryptocurrency.
    pub symbol: String,
    /// The web URL friendly shorthand version of this cryptocurrency name.
    pub slug: String,
    /// A map of market quotes in different currency conversions.
    /// The default map included is USD.
    #[serde(rename = "quote")]
    pub price_quotes: PriceQuotes,
}
