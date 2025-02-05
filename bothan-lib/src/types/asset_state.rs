use crate::types::asset_info::AssetInfo;

#[derive(Clone, Debug, PartialEq)]
pub enum AssetState {
    Unsupported,
    Pending,
    Available(AssetInfo),
}
