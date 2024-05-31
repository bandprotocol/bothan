use serde::{Deserialize, Serialize};

/// The default URL for the CryptoCompare API.
pub(crate) const DEFAULT_URL: &str = "https://min-api.cryptocompare.com/";

/// Represents the price of a cryptocurrency in USD.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Price {
    #[serde(rename = "USD")]
    pub usd: f64,
}
