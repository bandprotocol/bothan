use tracing::warn;

use bothan_core::manager::crypto_asset_info::types::PriceState;

use crate::api::crypto::PRECISION;
use crate::proto::price::{Price, Status};

pub fn parse_price_state(id: String, price_state: PriceState) -> Price {
    match price_state {
        PriceState::Available(mut raw_price) => {
            raw_price.rescale(PRECISION);
            let mantissa = raw_price.mantissa();
            match u64::try_from(mantissa) {
                Ok(p) => Price::new(id, p, Status::Available),
                Err(_) => {
                    warn!("failed to convert {mantissa} to u64 for id {id}");
                    Price::new(id, 0u64, Status::Unavailable)
                }
            }
        }
        PriceState::Unavailable => Price::new(id, 0u64, Status::Unavailable),
        PriceState::Unsupported => Price::new(id, 0u64, Status::Unsupported),
    }
}
