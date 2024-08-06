use std::collections::{HashMap, HashSet};

use rust_decimal::{Decimal, RoundingStrategy};

use crate::manager::crypto_asset_info::price::PRECISION;
use crate::manager::crypto_asset_info::types::PriceState;

pub fn get_price_state<T: AsRef<str>>(
    id: T,
    signal_results: &HashMap<String, Decimal>,
    unsupported_ids: &HashSet<String>,
) -> PriceState {
    let id = id.as_ref();
    if unsupported_ids.contains(id) {
        return PriceState::Unsupported;
    }

    match signal_results.get(id) {
        Some(price) => {
            let mantissa_price = price
                .round_dp_with_strategy(PRECISION, RoundingStrategy::ToZero)
                .mantissa();

            match i64::try_from(mantissa_price) {
                Ok(price) => PriceState::Available(price),
                Err(_) => PriceState::Unsupported,
            }
        }
        None => PriceState::Unavailable,
    }
}

pub fn is_stale(asset_timestamp: i64, current_time: i64, stale_threshold: i64) -> bool {
    (current_time - asset_timestamp) <= stale_threshold
}
