use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents different categories of trading products.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Category {
    /// Spot trading product.
    Spot,
    /// Linear trading product.
    Linear,
    /// Inverse trading product.
    Inverse,
    /// Option trading product.
    Option,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Category::Spot => write!(f, "spot"),
            Category::Linear => write!(f, "linear"),
            Category::Inverse => write!(f, "inverse"),
            Category::Option => write!(f, "option"),
        }
    }
}

/// Represents the ticker information for a spot trading product.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpotTicker {
    /// The symbol of the spot trading product.
    pub symbol: String,
    /// The price of the highest bid.
    #[serde(rename = "bid1Price")]
    pub bid1_price: String,
    /// The size of the highest bid.
    #[serde(rename = "bid1Size")]
    pub bid1_size: String,
    /// The price of the lowest ask.
    #[serde(rename = "ask1Price")]
    pub ask1_price: String,
    /// The size of the lowest ask.
    #[serde(rename = "ask1Size")]
    pub ask1_size: String,
    /// The last traded price.
    #[serde(rename = "lastPrice")]
    pub last_price: String,
    /// The price 24 hours ago.
    #[serde(rename = "prevPrice24h")]
    pub prev_price_24h: String,
    /// The percentage change in price over the last 24 hours.
    #[serde(rename = "price24hPcnt")]
    pub price_24h_pcnt: String,
    /// The highest price over the last 24 hours.
    #[serde(rename = "highPrice24h")]
    pub high_price_24h: String,
    /// The lowest price over the last 24 hours.
    #[serde(rename = "lowPrice24h")]
    pub low_price_24h: String,
    /// The turnover over the last 24 hours.
    #[serde(rename = "turnover24h")]
    pub turnover_24h: String,
    /// The volume traded over the last 24 hours.
    #[serde(rename = "volume24h")]
    pub volume_24h: String,
}

/// Represents different types of tickers.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Tickers {
    /// A list of spot tickers.
    Spot(Vec<SpotTicker>),
}

/// Represents the response containing tickers.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TickersResponse {
    /// The category of the tickers.
    #[serde(default)]
    pub category: Option<Category>,
    /// The list of tickers.
    #[serde(default)]
    pub list: Option<Tickers>,
}
