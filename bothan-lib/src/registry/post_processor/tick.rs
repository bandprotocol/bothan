// ! Tick value conversion post-processor.
//!
//! This module provides a post-processor that converts decimal values into 
//! tick values based on a logarithmic scale. Tick values represent price points 
//! on a non-linear scale, which can be useful for certain financial applications.
//!
//! The module provides:
//!
//! - The [`TickPostProcessor`] struct which implements conversion to tick values
//! - Constants defining the tick scale parameters
//! - Error handling for out-of-bound conversions
//!
//! # Tick System
//!
//! The tick system uses a logarithmic scale with base 1.0001, where:
//!
//! - Tick 262144 (MID_TICK) corresponds to a value of 1.0
//! - Higher tick values represent larger numbers
//! - Lower tick values represent smaller numbers
//! - The valid range is from tick 1 to tick 524287
//!
//! This provides a compact representation for a wide range of values with
//! consistent relative precision.

use bincode::{Decode, Encode};
use num_traits::FromPrimitive;
use rust_decimal::{Decimal, MathematicalOps};
use serde::{Deserialize, Serialize};

use crate::registry::post_processor::PostProcessError;

/// The base value for the tick system (1.0001).
/// Each tick represents a 0.01% change in value.
const TICK: f64 = 1.0001;

/// The tick value that corresponds to a decimal value of 1.0.
/// This serves as the midpoint of the tick scale.
const MID_TICK: f64 = 262144.0;

/// The maximum allowed tick value in the system.
const MAX_TICK: f64 = 524287.0;

/// The minimum allowed tick value in the system.
const MIN_TICK: f64 = 1.0;

/// Post-processor that converts decimal values to tick values.
///
/// The `TickPostProcessor` converts decimal values to tick values based on a logarithmic
/// scale. This conversion can be useful for representing a wide range of values in a
/// compact form with consistent relative precision.
///
/// # Tick Formula
///
/// The conversion uses the formula:
/// ```text
/// tick = log(value) / log(1.0001) + 262144
/// ```
///
/// # Constraints
///
/// The resulting tick value must be within the range [1, 524287]. If the conversion
/// yields a value outside this range, an error is returned.
///
/// # Examples
///
/// ```
/// use bothan_lib::registry::post_processor::{PostProcessor, tick::TickPostProcessor};
/// use rust_decimal::Decimal;
///
/// // Create a tick convertor post-processor
/// let post_processor = PostProcessor::TickConvertor(TickPostProcessor {});
///
/// // Convert a decimal value to a tick value
/// let value = Decimal::new(20, 0);  // The value 20.0
/// let result = post_processor.post_process(value).unwrap();
/// 
/// // The result will be approximately 292102.8
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct TickPostProcessor {}

impl TickPostProcessor {
    /// Converts a decimal value to its corresponding tick value.
    ///
    /// This method applies the logarithmic conversion to transform a decimal value
    /// into a tick value. The tick value represents the position on a non-linear scale
    /// where each step corresponds to a 0.01% change in value.
    ///
    /// # Errors
    ///
    /// Returns a `PostProcessError` if the computed tick value is outside the range [1, 524287].
    pub fn process(&self, data: Decimal) -> Result<Decimal, PostProcessError> {
        // Unwrap here is safe because the constants are hardcoded.
        let tick = Decimal::from_f64(TICK).unwrap();
        let min_tick = Decimal::from_f64(MIN_TICK).unwrap();
        let mid_tick = Decimal::from_f64(MID_TICK).unwrap();
        let max_tick = Decimal::from_f64(MAX_TICK).unwrap();

        let result = (data.log10() / tick.log10()) + mid_tick;
        if !(min_tick..=max_tick).contains(&result) {
            return Err(PostProcessError::new("Tick value out of bound"));
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::post_processor::PostProcessor;
    use crate::registry::post_processor::tick::TickPostProcessor;

    #[test]
    fn test_process() {
        let tick_convertor = PostProcessor::TickConvertor(TickPostProcessor {});
        let result = tick_convertor.post_process(Decimal::from(20));
        assert_eq!(
            result.unwrap(),
            Decimal::from_str_exact("292102.82057671349939971087257").unwrap()
        );
    }
}
