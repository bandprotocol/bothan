use std::collections::{HashMap, HashSet};

use crate::manager::crypto_asset_info::price::PRECISION;
use crate::price;
use crate::proto::query::{Price, PriceStatus};
use crate::registry::Registry;
use bothan_core::types::AssetInfo;
use rust_decimal::{Decimal, RoundingStrategy};

/// Partitions the signal ids into two vectors containing the supported and unsupported ids based on
/// the given registry.
pub fn partition_supported_ids<T>(signal_ids: Vec<String>, registry: &Registry) -> (T, T)
where
    T: Default + Extend<String>,
{
    signal_ids
        .into_iter()
        .partition(|id| registry.contains_key(id))
}

pub fn get_price_id(
    id: String,
    signal_results: &HashMap<String, Decimal>,
    unsupported_ids: &HashSet<String>,
) -> Price {
    match (signal_results.get(&id), unsupported_ids.contains(&id)) {
        (Some(price), _) => {
            let mantissa_price = price
                .round_dp_with_strategy(PRECISION, RoundingStrategy::ToZero)
                .mantissa();

            match i64::try_from(mantissa_price) {
                Ok(price) => price!(id, PriceStatus::Available, price),
                Err(_) => price!(id, PriceStatus::Unavailable, 0),
            }
        }
        (_, true) => price!(&id, PriceStatus::Unsupported, 0),
        _ => price!(id, PriceStatus::Unavailable, 0),
    }
}

pub fn filter_stale_assets(
    asset: AssetInfo,
    current_time: i64,
    stale_threshold: i64,
) -> Option<AssetInfo> {
    if (current_time - asset.timestamp) <= stale_threshold {
        Some(asset)
    } else {
        None
    }
}
