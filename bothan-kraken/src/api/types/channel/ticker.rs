use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum EventTrigger {
    #[serde(rename = "bbo")]
    BBO,
    #[serde(rename = "trades")]
    Trades,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TickerRequestParameters {
    pub channel: String,
    pub symbol: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_trigger: Option<EventTrigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
}

pub struct TickerSubscriptionResult {
    pub channel: String,
    pub snapshot: bool,
    pub symbol: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TickerResponse {
    pub symbol: String,
    pub bid: f64,
    pub bid_qty: f64,
    pub ask: f64,
    pub ask_qty: f64,
    pub last: f64,
    pub volume: f64,
    pub vwap: f64,
    pub low: f64,
    pub high: f64,
    pub change: f64,
    pub change_pct: f64,
}
