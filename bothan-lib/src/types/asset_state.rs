use serde::{Deserialize, Serialize};

use crate::types::asset_info::AssetInfo;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AssetState {
    Unsupported,
    Pending,
    Available(AssetInfo),
}
