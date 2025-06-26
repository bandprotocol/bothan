//! # API Utilities Module
//!
//! This module provides utility functions for the Bothan API server implementation.
//! It includes helpers for parsing price state and converting to API response types.
//!
//! ## Functions
//!
//! - `parse_price_state`: Converts a `PriceState` to a `Price` API response.

use bothan_core::manager::crypto_asset_info::types::PriceState;
use rust_decimal::prelude::Zero;
use tracing::warn;

use crate::api::server::PRECISION;
use crate::proto::bothan::v1::{Price, Status};

/// Converts a `PriceState` to a `Price` API response.
///
/// # Arguments
///
/// * `id` - The signal or asset identifier.
/// * `price_state` - The price state to convert.
///
/// # Returns
///
/// Returns a `Price` struct suitable for API responses.
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
        PriceState::Unavailable => Price::new(id, u64::zero(), Status::Unavailable),
        PriceState::Unsupported => Price::new(id, u64::zero(), Status::Unsupported),
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::*;

    #[test]
    fn test_parse_price_state() {
        let id = "test".to_string();
        let price_state = PriceState::Available(Decimal::from_str_exact("0.123456789").unwrap());
        let price = parse_price_state(id.clone(), price_state);

        let expected_price = Price::new(id.clone(), 123456789_u64, Status::Available);
        assert_eq!(price, expected_price);
    }

    #[test]
    fn test_parse_price_state_with_lower_scale() {
        let id = "test".to_string();
        let price_state = PriceState::Available(Decimal::from_str_exact("0.1").unwrap());
        let price = parse_price_state(id.clone(), price_state);

        let expected_price = Price::new(id.clone(), 100000000_u64, Status::Available);
        assert_eq!(price, expected_price);
    }

    #[test]
    fn test_parse_price_state_with_higher_scale() {
        let id = "test".to_string();
        let price_state =
            PriceState::Available(Decimal::from_str_exact("0.0010000000000001").unwrap());
        let price = parse_price_state(id.clone(), price_state);

        let expected_price = Price::new(id.clone(), 1000000_u64, Status::Available);
        assert_eq!(price, expected_price);
    }

    #[test]
    fn test_parse_price_state_with_higher_scale_and_round_up() {
        let id = "test".to_string();
        let price_state = PriceState::Available(Decimal::from_str_exact("0.0010000019").unwrap());
        let price = parse_price_state(id.clone(), price_state);

        let expected_price = Price::new(id.clone(), 1000002_u64, Status::Available);
        assert_eq!(price, expected_price);
    }

    #[test]
    fn test_parse_price_state_unavailable() {
        let id = "test".to_string();
        let price_state = PriceState::Unavailable;
        let price = parse_price_state(id.clone(), price_state);

        let expected_price = Price::new(id.clone(), u64::zero(), Status::Unavailable);
        assert_eq!(price, expected_price);
    }

    #[test]
    fn test_parse_price_state_unsupported() {
        let id = "test".to_string();
        let price_state = PriceState::Unsupported;
        let price = parse_price_state(id.clone(), price_state);

        let expected_price = Price::new(id.clone(), u64::zero(), Status::Unsupported);
        assert_eq!(price, expected_price);
    }
}
