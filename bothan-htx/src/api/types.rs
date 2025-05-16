use serde::{Deserialize, Serialize};

/// The default URL for the Htx WebSocket API.
pub const DEFAULT_URL: &str = "wss://api.huobi.pro/ws";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Response {
    Subscribed(Subscribed),
    Unsubscribed(Unsubscribed),
    DataUpdate(Data),
    Ping(Ping),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Subscribed {
    pub id: Option<String>,
    pub status: String,
    pub subbed: String,
    #[serde(rename = "ts")]
    pub timestamp: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Unsubscribed {
    pub id: Option<String>,
    pub status: String,
    pub unsubbed: String,
    #[serde(rename = "ts")]
    pub timestamp: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub ch: String,
    #[serde(rename = "ts")]
    pub timestamp: i64,
    pub tick: Tick,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Tick {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub amount: f64,
    pub vol: f64,
    pub count: u64,
    pub bid: f64,
    pub bid_size: f64,
    pub ask: f64,
    pub ask_size: f64,
    pub last_price: f64,
    pub last_size: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Ping {
    pub ping: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Pong {
    pub pong: u64,
}
