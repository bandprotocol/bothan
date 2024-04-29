use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    pub api_version: String,
    pub connection_id: usize,
    pub system: String,
    pub version: String,
}
