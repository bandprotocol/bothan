use std::time::Duration;

use derive_more::Display;
use serde::Deserialize;

pub const DEFAULT_CHANNEL_SIZE: usize = 100;
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(720);

#[derive(Clone, Debug, Deserialize, Display)]
#[display("id: {}, price: {}, timestamp: {}", id, price, timestamp)]
pub struct PriceData {
    pub id: String,
    pub price: String,
    pub timestamp: u64,
}

pub(crate) enum Command {
    Subscribe(Vec<String>),
}
