use serde::{Deserialize, Serialize};

pub(crate) const DEFAULT_URL: &str = "https://min-api.cryptocompare.com/";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Price {
    #[serde(rename = "USD")]
    pub usd: f64,
}
