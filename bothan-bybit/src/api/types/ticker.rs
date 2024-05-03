use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Category {
    Spot,
    Linear,
    Inverse,
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpotTicker {
    pub symbol: String,
    #[serde(rename = "bid1Price")]
    pub bid1_price: String,
    #[serde(rename = "bid1Size")]
    pub bid1_size: String,
    #[serde(rename = "ask1Price")]
    pub ask1_price: String,
    #[serde(rename = "ask1Size")]
    pub ask1_size: String,
    #[serde(rename = "lastPrice")]
    pub last_price: String,
    #[serde(rename = "prevPrice24h")]
    pub prev_price_24h: String,
    #[serde(rename = "price24hPcnt")]
    pub price_24h_pcnt: String,
    #[serde(rename = "highPrice24h")]
    pub high_price_24h: String,
    #[serde(rename = "lowPrice24h")]
    pub low_price_24h: String,
    #[serde(rename = "turnover24h")]
    pub turnover_24h: String,
    #[serde(rename = "volume24h")]
    pub volume_24h: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Tickers {
    Spot(Vec<SpotTicker>),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TickersResponse {
    #[serde(default)]
    pub category: Option<Category>,
    #[serde(default)]
    pub list: Option<Tickers>,
}
