use num_traits::FromPrimitive;
use rust_decimal::{Decimal, MathematicalOps};
use serde::{Deserialize, Serialize};

use crate::registry::post_processor::{PostProcessor, PostProcessorError};

const TICK: f64 = 1.0001;
const MID_TICK: f64 = 262144.0;
const MAX_TICK: f64 = 524287.0;
const MIN_TICK: f64 = 1.0;

/// `TickPostProcessor` processes the given data into its tick value.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TickPostProcessor {}

impl PostProcessor<Decimal> for TickPostProcessor {
    /// Processes the given data into its tick value and returns it. If the data is out of bounds,
    /// it returns an error.
    fn process(&self, data: Decimal) -> Result<Decimal, PostProcessorError> {
        // Unwrap here is safe because the constants are hardcoded.
        let tick = Decimal::from_f64(TICK).unwrap();
        let min_tick = Decimal::from_f64(MIN_TICK).unwrap();
        let mid_tick = Decimal::from_f64(MID_TICK).unwrap();
        let max_tick = Decimal::from_f64(MAX_TICK).unwrap();

        let result = (data.log10() / tick.log10()) + mid_tick;
        if !(min_tick..=max_tick).contains(&result) {
            return Err(PostProcessorError::new("Tick value out of bound"));
        }

        Ok(result)
    }
}
