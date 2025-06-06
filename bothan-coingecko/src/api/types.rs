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
/// # Examples
///
/// ```rust
/// use bothan_coingecko::api::types::Coin;
/// use serde_json::json;
///
/// let json_data = json!({
///     "id": "bitcoin",
///     "symbol": "btc",
///     "name": "Bitcoin"
/// });
///
/// let coin: Coin = serde_json::from_value(json_data).unwrap();
///
/// assert_eq!(coin.id, "bitcoin");
/// assert_eq!(coin.symbol, "btc");
/// assert_eq!(coin.name, "Bitcoin");
/// ```
///
/// # CoinGecko REST API Response Example
///
/// ```json
/// {
///   "id": "0chain",
///   "symbol": "zcn",
///   "name": "Zus",
///   "platforms": {
///     "ethereum": "0xb9ef770b6a5e12e45983c5d80545258aa38f3b78",
///     "polygon-pos": "0x8bb30e0e67b11b978a5040144c410e1ccddcba30"
///   }
/// }
/// ```
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
/// # Examples
///
/// ```rust
/// use bothan_coingecko::api::types::Price;
/// use serde_json::json;
///
/// let json_data = json!({
///     "usd": 67187.3358936566,
///     "last_updated_at": 1711356300
/// });
///
/// let price: Price = serde_json::from_value(json_data).unwrap();
///
/// assert_eq!(price.usd, 67187.3358936566);
/// assert_eq!(price.last_updated_at, 1711356300);
/// ```
///
/// # CoinGecko REST API Response Example
///
/// ```json
/// {
///   "bitcoin": {
///     "usd": 67187.3358936566,
///     "usd_market_cap": 1317802988326.25,
///     "usd_24h_vol": 31260929299.5248,
///     "usd_24h_change": 3.63727894677354,
///     "last_updated_at": 1711356300
///   }
/// }
/// ```
///
/// [CoinGecko Simple Price endpoint]: https://docs.coingecko.com/reference/simple-price
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Price {
    /// Latest price in USD.
    pub usd: f64,
    /// Unix timestamp (in seconds) of the latest price update.
    pub last_updated_at: i64,
}
