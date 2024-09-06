use serde::{Deserialize, Serialize};

pub(crate) const DEFAULT_USER_AGENT: &str = "Bothan";

pub(crate) const DEFAULT_URL: &str = "https://api.coingecko.com/api/v3/";

pub(crate) const DEFAULT_PRO_URL: &str = "https://pro-api.coingecko.com/api/v3/";

pub(crate) const API_KEY_HEADER: &str = "x-cg-pro-api-key";

/// Represents a coin with basic information.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Coin {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

/// Represents market data for a coin.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Price {
    pub usd: f64,
    pub last_updated_at: i64,
}
