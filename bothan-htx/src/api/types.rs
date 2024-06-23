use serde::{Deserialize, Serialize};

/// The default URL for the Huobi API.
pub(crate) const DEFAULT_URL: &str = "https://api.huobi.pro/";

/// Represents the status of a response.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    /// The request was successful.
    Ok,
    /// The request resulted in an error.
    Error,
}

/// A generic response wrapper.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Response<T> {
    /// The actual data of the response.
    pub data: T,
    /// The status of the response.
    pub status: Status,
    /// The timestamp of the response.
    #[serde(rename = "ts")]
    pub timestamp: usize,
}

/// Represents a ticker.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    pub symbol: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub amount: f64,
    pub vol: f64,
    pub count: usize,
    pub bid: f64,
    pub bid_size: f64,
    pub ask: f64,
    pub ask_size: f64,
}
