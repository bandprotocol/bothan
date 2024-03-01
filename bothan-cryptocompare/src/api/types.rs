use serde::{Deserialize, Serialize};

pub(crate) const DEFAULT_USER_AGENT: &str = "Bothan";
pub(crate) const DEFAULT_URL: &str = "https://min-api.cryptocompare.com/";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Price {
    #[serde(rename = "USD")]
    pub usd: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Market {
    pub id: String,
    pub current_price: f64,
    pub timestamp: u64,
}
