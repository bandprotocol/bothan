use std::collections::{HashMap, HashSet};

use crate::manager::crypto_asset_info::price::PRECISION;
use crate::price;
use crate::proto::query::{Price, PriceStatus};
use rust_decimal::{Decimal, RoundingStrategy};

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

pub fn is_stale(asset_timestamp: i64, current_time: i64, stale_threshold: i64) -> bool {
    (current_time - asset_timestamp) <= stale_threshold
}
