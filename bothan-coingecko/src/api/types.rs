//! Data types for interacting with the CoinGecko REST API.
//!
//! This module provides types for deserializing responses from CoinGecko REST API.
//!
use serde::{Deserialize, Serialize};

/// The default User-Agent header for HTTP requests made to the CoinGecko REST API.
/// Setting a custom user agent helps identify your application to the API provider,
/// which can be useful for monitoring and debugging purposes.
pub(crate) const DEFAULT_USER_AGENT: &str = "Bothan";

/// The base URL for the public CoinGecko REST API (version 3).
/// It's used for accessing publicly available endpoints that don't require authentication,
/// such as fetching coin lists, market data, and more.
pub(crate) const DEFAULT_URL: &str = "https://api.coingecko.com/api/v3/";

/// The base URL for the CoinGecko Pro API (version 3).
/// The Pro API offers additional features and higher rate limits compared to the public API.
/// Accessing this API requires a valid API key.
pub(crate) const DEFAULT_PRO_URL: &str = "https://pro-api.coingecko.com/api/v3/";

/// The header key used to pass the API key when making requests to the Pro API.
/// Including this header in your requests authenticates them,
/// granting access to Pro API features and higher rate limits.
pub(crate) const API_KEY_HEADER: &str = "x-cg-pro-api-key";

/// Represents coin information retrieved from CoinGecko REST API.
///
/// `Coin` contains fields matching those returned by the [CoinGecko coins list endpoint].
/// It serves as an interface for JSON deserialization of API responses.
///
/// **Note:** This struct does **not** include all fields returned by the CoinGecko REST API.
/// Specifically, fields like `platforms` and other additional data provided by certain endpoints
/// are not represented here.
///
/// [CoinGecko coins list endpoint]: https://docs.coingecko.com/reference/coins-list
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coin {
    /// Unique identifier for the coin
    pub id: String,
    /// Trading symbol
    pub symbol: String,
    /// Full coin name
    pub name: String,
}

/// Represents price information retrieved from the CoinGecko REST API.
///
/// `Price` struct contains fields matching those returned by the [CoinGecko Simple Price endpoint].
/// It serves as an interface for JSON deserialization of API responses.
///
/// **Note:** This struct only includes selected fields from the CoinGecko REST API response.
/// Specifically, additional data such as `usd_market_cap`, `usd_24h_vol`, and
/// `usd_24h_change` are not represented here.
///
/// [CoinGecko Simple Price endpoint]: https://docs.coingecko.com/reference/simple-price
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Price {
    /// Latest price in USD.
    pub usd: f64,
    /// Unix timestamp (in seconds) of the latest price update.
    pub last_updated_at: i64,
}
