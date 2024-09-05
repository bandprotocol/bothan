use rust_decimal::RoundingStrategy;
use tracing::warn;

use bothan_core::manager::crypto_asset_info::types::PriceState;

use crate::api::crypto::PRECISION;
use crate::proto::price::{Price, Status};

pub fn parse_price_state(id: String, price_state: PriceState) -> Price {
    match price_state {
        PriceState::Available(raw_price) => {
            let mantissa_price = raw_price
                .round_dp_with_strategy(PRECISION, RoundingStrategy::ToZero)
                .mantissa();
            match i64::try_from(mantissa_price) {
                Ok(p) => Price::new(id, p, Status::Available),
                Err(_) => {
                    warn!("failed to convert {mantissa_price} to i64 for id {id}");
                    Price::new(id, 0, Status::Unavailable)
                }
            }
        }
        PriceState::Unavailable => Price::new(id, 0, Status::Unavailable),
        PriceState::Unsupported => Price::new(id, 0, Status::Unsupported),
    }
}
