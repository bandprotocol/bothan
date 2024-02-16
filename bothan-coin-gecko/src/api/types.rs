use serde::{Deserialize, Serialize};

pub(crate) const MAX_PAGE_SIZE: usize = 250;
pub(crate) const DEFAULT_USER_AGENT: &str = "Bothan";
pub(crate) const DEFAULT_URL: &str = "https://api.coingecko.com/api/v3";
pub(crate) const DEFAULT_PRO_URL: &str = "https://pro-api.coingecko.com/api/v3";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Coin {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Market {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub current_price: f64,
    pub last_updated: String,
}
