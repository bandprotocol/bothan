use serde::{Deserialize, Serialize};

/// The default user agent for requests.
pub(crate) const DEFAULT_USER_AGENT: &str = "Bothan";

/// The default URL for the CoinGecko API.
pub(crate) const DEFAULT_URL: &str = "https://api.coingecko.com/api/v3/";

/// The default URL for the CoinGecko Pro API.
pub(crate) const DEFAULT_PRO_URL: &str = "https://pro-api.coingecko.com/api/v3/";

/// Represents a coin with basic information.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Coin {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

/// Represents market data for a coin.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Market {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub current_price: f64,
    pub last_updated: String,
}
