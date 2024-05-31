use serde::{Deserialize, Serialize};

/// Represents the status information of the Kraken API.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    /// The API version.
    pub api_version: String,
    /// The connection ID.
    pub connection_id: usize,
    /// The system name.
    pub system: String,
    /// The system version.
    pub version: String,
}
