use serde::{Deserialize, Serialize};

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
