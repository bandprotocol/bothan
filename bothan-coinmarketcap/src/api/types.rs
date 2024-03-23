use serde::{Deserialize, Serialize};

pub(crate) const DEFAULT_URL: &str = "https://pro-api.coinmarketcap.com";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub timestamp: String,
    pub error_code: usize,
    pub error_message: Option<String>,
    pub elapsed: usize,
    pub credit_count: usize,
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
    pub market_cap: f64,
    pub market_cap_dominance: f64,
    pub fully_diluted_market_cap: f64,
    pub percent_change_1h: f64,
    pub percent_change_24h: f64,
    pub percent_change_7d: f64,
    pub percent_change_30d: f64,
    pub last_updated: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PriceQuotes {
    #[serde(rename = "USD")]
    pub usd: PriceQuote,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Quote {
    pub id: usize,
    pub name: String,
    pub symbol: String,
    pub slug: String,
    #[serde(rename = "quote")]
    pub price_quotes: PriceQuotes,
}
