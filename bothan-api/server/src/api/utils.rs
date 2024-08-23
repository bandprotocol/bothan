use crate::api::crypto::PRECISION;
use crate::proto::query::{Price, PriceStatus, UpdateRegistryResponse, UpdateStatusCode};
use bothan_core::manager::crypto_asset_info::types::PriceState;
use rust_decimal::RoundingStrategy;
use tonic::Response;
use tracing::warn;

pub fn registry_resp(status: UpdateStatusCode) -> Response<UpdateRegistryResponse> {
    let update_registry_response = UpdateRegistryResponse {
        code: status.into(),
    };
    Response::new(update_registry_response)
}

pub fn parse_price_state(id: String, price_state: PriceState) -> Price {
    match price_state {
        PriceState::Available(raw_price) => {
            let mantissa_price = raw_price
                .round_dp_with_strategy(PRECISION, RoundingStrategy::ToZero)
                .mantissa();
            match i64::try_from(mantissa_price) {
                Ok(p) => Price::new(id, p, PriceStatus::Available),
                Err(_) => {
                    warn!("Failed to convert {mantissa_price} to i64 for id {id}");
                    Price::new(id, 0, PriceStatus::Unavailable)
                }
            }
        }
        PriceState::Unavailable => Price::new(id, 0, PriceStatus::Unavailable),
        PriceState::Unsupported => Price::new(id, 0, PriceStatus::Unsupported),
    }
}
