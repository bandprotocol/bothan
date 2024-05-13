use serde::{Deserialize, Serialize};

pub mod ticker;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Channel {
    Ticker,
    TickerBatch,
}
