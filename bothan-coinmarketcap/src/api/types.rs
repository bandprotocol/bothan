use serde::{Deserialize, Serialize};

pub(crate) const DEFAULT_URL: &str = "https://pro-api.coinmarketcap.com";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub timestamp: String,
    pub error_code: u64,
    pub error_message: Option<String>,
    pub elapsed: u64,
    pub credit_count: u64,
    pub notice: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Response<T> {
    pub data: T,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PriceQuote {
    pub price: Option<f64>,
    pub volume_24h: f64,
    pub volume_change_24h: f64,
    pub percent_change_1h: f64,
    pub percent_change_24h: f64,
    pub percent_change_7d: f64,
    pub percent_change_30d: f64,
    pub market_cap: Option<f64>,
    pub market_cap_dominance: f64,
    pub fully_diluted_market_cap: f64,
    pub last_updated: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PriceQuotes {
    #[serde(rename = "USD")]
    pub usd: PriceQuote,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Quote {
    pub id: u64,
    pub name: String,
    pub symbol: String,
    pub slug: String,
    #[serde(rename = "quote")]
    pub price_quotes: PriceQuotes,
}
