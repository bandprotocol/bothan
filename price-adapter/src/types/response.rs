use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents information about the price of a symbol.
///
/// This struct is used to store details about the price of a symbol,
/// including the symbol name, the price value, and the timestamp.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PriceInfo {
    pub symbol: String,
    pub price: f64,
    pub timestamp: u64,
}

impl fmt::Display for PriceInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PriceInfo {{ symbol: {}, price: {}, timestamp: {} }}",
            self.symbol, self.price, self.timestamp
        )
    }
}

/// Represents the response structure for setting-related data.
///
/// This struct is used to deserialize JSON responses containing
/// setting-related information.
#[derive(Debug, Deserialize)]
pub struct SettingResponse {
    pub data: Value,
}

/// Represents different types of messages received over a WebSocket connection.
///
/// This enum encapsulates various types of messages that can be received
/// over a WebSocket connection, such as price information or setting responses.
#[derive(Debug)]
pub enum WebsocketMessage {
    /// Represents a message containing price information.
    PriceInfo(PriceInfo),

    /// Represents a message containing setting-related data.
    SettingResponse(SettingResponse),
}
