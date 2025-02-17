use crate::types::asset_info::AssetInfo;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AssetState {
    Unsupported,
    Pending,
    Available(AssetInfo),
}
