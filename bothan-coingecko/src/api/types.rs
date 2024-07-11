use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub(crate) const DEFAULT_USER_AGENT: &str = "Bothan";

pub(crate) const DEFAULT_URL: &str = "https://api.coingecko.com/api/v3/";

pub(crate) const DEFAULT_PRO_URL: &str = "https://pro-api.coingecko.com/api/v3/";

pub(crate) const API_KEY_HEADER: &str = "x-cg-pro-api-key";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Order {
    MarketCapAsc,
    MarketCapDesc,
    VolumeAsc,
    VolumeDesc,
    IdAsc,
    IdDesc,
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Order::MarketCapAsc => "market_cap_asc",
            Order::MarketCapDesc => "market_cap_desc",
            Order::VolumeAsc => "volume_asc",
            Order::VolumeDesc => "volume_desc",
            Order::IdAsc => "id_asc",
            Order::IdDesc => "id_desc",
        };
        write!(f, "{}", str)
    }
}

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
